// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tracing::{error, instrument, trace};

use crate::config::app_config::AppConfig;

pub const CONFIG_RELOADED_EVENT: &str = "config-reloaded";

#[derive(Clone, Serialize)]
struct ConfigReloadedPayload {
    config: AppConfig,
}

#[instrument(skip_all)]
pub fn send_config_event(app: &AppHandle, config: &AppConfig) {
    trace!("___sending event {CONFIG_RELOADED_EVENT}");
    app.emit(
        CONFIG_RELOADED_EVENT,
        ConfigReloadedPayload {
            config: config.clone(),
        },
    )
    .inspect_err(|e| error!("failed to send event {e}"))
    .ok();
}
