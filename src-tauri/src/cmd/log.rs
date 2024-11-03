use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, instrument, trace, warn};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[instrument(skip_all, name = "js")]
#[tauri::command]
pub fn log_js(message: String, level: Option<Level>) -> () {
    match level {
        Some(Level::Trace) => trace!(message),
        Some(Level::Debug) => debug!(message),
        Some(Level::Info) => info!(message),
        Some(Level::Warn) => warn!(message),
        Some(Level::Error) => error!(message),
        None => info!(message),
    }
}
