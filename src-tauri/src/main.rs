// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod command;
mod config;
mod error;
mod event;
mod log;
mod tray;
mod util;
mod window;

use std::sync::Mutex;
use tauri::Manager;
use tracing::{debug, error};

use crate::command::config;
use crate::config::config::AppConfig;
use crate::config::config_file::ConfigFile;

pub const APP_NAME: &str = "sapin";

struct CfgState(Mutex<AppConfig>);

#[tokio::main]
async fn main() {
    log::setup_tracing();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![config])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            #[cfg(dev)]
            window.open_devtools();

            // Check if a config file exists, if not create a default one
            ConfigFile::check(app.handle()).ok();

            let (config, config_file) = AppConfig::load(app.handle(), false);
            if let Some(file) = config_file.as_ref() {
                let file = file.clone();
                let handle = app.handle().clone();
                tokio::spawn(async move {
                    file.watch(&handle).ok();
                });
            }
            app.manage(CfgState(Mutex::new(config)));

            #[cfg(not(dev))]
            {
                window
                    .set_ignore_cursor_events(true)
                    .inspect_err(|e| error!("failed to set ignore cursor event: {}", e))
                    .ok();
            }

            tray::setup(app.handle())?;
            debug!("SETUP DONE");
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
