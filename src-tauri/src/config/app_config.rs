// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::config::config_file::{ConfigData, ConfigFile};
use crate::event;
use crate::window::setup;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use tracing::{debug, info, instrument, trace, warn};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Crosshair {
    pub size: u32,
    pub thickness: u32,
    pub gap: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dot {
    pub radius: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Shape {
    Crosshair(Crosshair),
    Dot(Dot),
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub enum Position {
    #[default]
    Center,
    Custom {
        x: u32,
        y: u32,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub shape: Shape,
    pub color: String,
    pub alpha: f64, // 0.0 - 1.0
    pub debug: bool,
    pub position: Position,
    pub protected: bool,
}

impl AppConfig {
    #[instrument(skip(app))]
    pub fn load(app: &AppHandle, notify: bool) -> (AppConfig, Option<ConfigFile>) {
        trace!("___load");
        let config_file = ConfigFile::try_from(app).ok();
        if config_file.is_none() {
            warn!("failed to read config file, using default config");
        }
        config_file
            .as_ref()
            .inspect(|cfg| info!("config file: {}", cfg.full_path().display()));
        let config: AppConfig = AppConfig::from(&config_file);
        debug!("config: {:?}", config);
        let window = app.get_webview_window("main").unwrap();
        setup(&window, &config).ok();
        debug!("config loaded");

        // notify the frontend
        if notify {
            event::send_config_event(app, &config);
        }
        (config, config_file)
    }
}

impl From<&Option<ConfigFile>> for AppConfig {
    fn from(config_file: &Option<ConfigFile>) -> Self {
        config_file
            .as_ref()
            .and_then(|f| f.parse().ok())
            .map(AppConfig::from)
            .unwrap_or_else(|| {
                debug!("failed to read config file, using default config");
                AppConfig::default()
            })
    }
}

impl From<ConfigData> for AppConfig {
    fn from(cfg: ConfigData) -> Self {
        let default = AppConfig::default();
        AppConfig {
            shape: cfg
                .crosshair
                .map(Shape::Crosshair)
                .or_else(|| cfg.dot.map(Shape::Dot))
                .unwrap_or_default(),
            color: cfg.color.unwrap_or(default.color),
            alpha: cfg.alpha.unwrap_or(default.alpha),
            debug: cfg.debug.unwrap_or(default.debug),
            position: cfg.position.unwrap_or(default.position),
            protected: cfg.protected.unwrap_or(true),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            shape: Shape::default(),
            color: "green".to_string(),
            alpha: 0.7,
            debug: false,
            position: Position::Center,
            protected: true,
        }
    }
}

impl Default for Shape {
    fn default() -> Self {
        Shape::Crosshair(Crosshair {
            size: 22,
            thickness: 4,
            gap: Some(12),
        })
    }
}

impl Shape {
    pub fn size(&self) -> u32 {
        match *self {
            Shape::Crosshair(Crosshair { size, .. }) => size,
            Shape::Dot(Dot { radius }) => radius * 2,
        }
    }
}
