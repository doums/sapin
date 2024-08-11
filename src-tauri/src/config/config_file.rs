// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::config::app_config::{AppConfig, Crosshair, Dot, Position};
use crate::{util, APP_NAME};

use anyhow::Result;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc;
use tauri::{AppHandle, Manager};
use tracing::{debug, error, info, instrument, warn};

pub const CONFIG_FILE: &str = "config.toml";

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ConfigData {
    pub crosshair: Option<Crosshair>,
    pub dot: Option<Dot>,
    pub color: Option<String>,
    pub alpha: Option<f64>, // 0.0 - 1.0
    pub debug: Option<bool>,
    pub position: Option<Position>,
    pub protected: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct ConfigFile {
    pub path: PathBuf,
    pub filename: String,
}

impl ConfigFile {
    #[instrument(skip_all)]
    pub fn try_from(app: &AppHandle) -> Result<ConfigFile> {
        let mut config_dir = app
            .path()
            .config_dir()
            .inspect_err(|e| error!("failed to get config dir: {e}"))?;

        config_dir.push(APP_NAME);
        util::check_dir(&config_dir)?;

        Ok(ConfigFile {
            path: config_dir,
            filename: CONFIG_FILE.to_string(),
        })
    }

    #[instrument(skip_all)]
    pub fn parse(&self) -> Result<ConfigData> {
        let file = self.path.join(&self.filename);
        let data = std::fs::read_to_string(&file)
            .inspect_err(|e| warn!("failed to read config file {}: {e}", file.display()))?;
        toml::from_str(&data)
            .inspect_err(|e| error!("failed to parse config file {}: {e}", file.display()))
            .map_err(|e| e.into())
    }

    #[instrument(skip_all)]
    pub fn watch(&self, app: &AppHandle) -> Result<()> {
        if !self.full_path().try_exists()? {
            info!("config file does not exist, skipping watcher");
            return Ok(());
        }

        let (tx, rx) = mpsc::channel();
        let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
        watcher
            .watch(&self.full_path(), RecursiveMode::NonRecursive)
            .inspect_err(|e| error!("error while trying to watch config file {e}"))?;

        for res in rx {
            match res {
                Ok(event) => {
                    debug!("config watch event {event:?}");
                    // reload config and notify the frontend
                    let (config, _) = AppConfig::load(app, true);
                    util::update_state(app, config);
                }
                Err(e) => error!("config watcher received an error {e:?}"),
            }
        }
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn full_path(&self) -> PathBuf {
        self.path.join(&self.filename)
    }

    /// Check if a config file exists, if not create a default one
    #[instrument(skip_all)]
    pub fn check(app: &AppHandle) -> Result<()> {
        let file = ConfigFile::try_from(app)?;
        let path = file.full_path();

        let config = r##"# any valid CSS color, e.g. "limegreen", "#034017", "rgb(255, 0, 0)"
color = "green"
alpha = 0.7 # 0.0 - 1.0
debug = false
# prevents the window contents from being captured by other apps
protected = true

[crosshair]
size = 22
thickness = 4
gap = 12

# to use a dot uncomment the following lines (and remove the [crosshair] section)
#[dot]
#radius = 4
"##;
        if !path.is_file() {
            info!("config file does not exist, generating a default one");
            fs::write(path, config)
                .inspect_err(|e| error!("failed to write default config file: {e}"))?;
        }
        Ok(())
    }
}

impl Display for ConfigFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.full_path().display())
    }
}
