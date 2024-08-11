// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use tauri::State;
use tracing::{debug, error, instrument};

use crate::config::app_config::AppConfig;
use crate::error::CmdError;
use crate::CfgState;

#[instrument(skip(state))]
#[tauri::command]
pub fn config(state: State<CfgState>) -> Result<AppConfig, CmdError> {
    debug!("___config");
    let guard = state.0.lock().map_err(|e| {
        error!("failed to lock config state: {}", e);
        CmdError::Internal
    })?;
    Ok(guard.clone())
}
