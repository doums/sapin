// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::config::config::AppConfig;
use crate::CfgState;

use anyhow::{anyhow, Result};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use tracing::{debug, error, instrument};

/// Check if a directory exists, if not create it including all
/// parent components
#[instrument]
pub fn check_dir(path: &PathBuf) -> Result<()> {
    if !path.is_dir() {
        debug!("directory `{}` does not exist, creating it", path.display());
        return fs::create_dir_all(path)
            .inspect_err(|e| error!("Failed to create directory `{}`: {e}", path.display()))
            .map_err(|e| anyhow!(e));
    }
    Ok(())
}

#[instrument(skip(app))]
pub fn update_state(app: &AppHandle, config: AppConfig) {
    let state = app.state::<CfgState>();
    let mut guard = state
        .0
        .lock()
        .inspect_err(|e| error!("failed to lock config state: {}", e))
        .unwrap();
    *guard = config.clone();
}
