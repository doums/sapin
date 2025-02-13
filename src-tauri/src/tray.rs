// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use anyhow::Result;
use strum::AsRefStr;
use tauri::image::Image;
use tauri::menu::{MenuBuilder, MenuEvent};
use tauri::tray::{TrayIcon, TrayIconBuilder, TrayIconEvent};
use tauri::{include_image, AppHandle, Manager};
use tauri_plugin_opener::OpenerExt;
use tracing::{debug, error, info, instrument, trace, warn};

use crate::config::app_config::AppConfig;
use crate::config::config_file::ConfigFile;
use crate::util;
#[cfg(not(target_os = "linux"))]
use crate::APP_NAME;

pub const TRAY_ICON_ID: &str = "main";
pub const TRAY_MENU_ID: &str = "tray_menu";
const APP_ICON: Image<'_> = include_image!("icons/32x32.png");

#[derive(AsRefStr, Debug)]
enum MenuItemId {
    ShowHide,
    Config,
    Reload,
    Quit,
}

#[instrument(skip_all)]
fn on_tray_event(_tray_icon: &TrayIcon, event: TrayIconEvent) {
    if let TrayIconEvent::Click { button, .. } = event {
        trace!("tray event [click] {:?}", button);
    }
}

#[instrument(skip(app))]
fn on_menu_event(app: &AppHandle, event: MenuEvent) {
    trace!("menu event [{}]", event.id.0);
    match event.id().as_ref() {
        "ShowHide" => {
            let window = app.get_webview_window("main").unwrap();
            if window.is_visible().unwrap() {
                info!("hiding overlay");
                window.hide().unwrap();
            } else {
                info!("showing overlay");
                window.show().unwrap();
            }
        }
        "Config" => {
            trace!("config menu clicked");
            let config_path = ConfigFile::try_from(app).ok();
            if let Some(cfg) = config_path {
                app.opener()
                    .open_path(cfg.path.to_string_lossy(), None::<&str>)
                    .inspect_err(|e| error!("failed to open {}: {e}", cfg.path.display()))
                    .ok();
            }
        }
        "Reload" => {
            trace!("reload menu clicked");
            let (config, _) = AppConfig::load(app, true);
            util::update_state(app, config);
        }
        "Quit" => {
            trace!("quit menu clicked");
            app.exit(0);
        }
        _ => warn!("unhandled menu event: {:?}", event.id),
    }
}

#[instrument(skip_all)]
pub fn setup(app: &AppHandle) -> Result<()> {
    debug!("building system tray");
    let menu = MenuBuilder::with_id(app, TRAY_MENU_ID)
        .text(MenuItemId::ShowHide.as_ref(), "Show/Hide")
        .separator()
        .text(MenuItemId::Config.as_ref(), "Config")
        .text(MenuItemId::Reload.as_ref(), "Reload")
        .separator()
        .text(MenuItemId::Quit.as_ref(), "Quit")
        .build()
        .inspect_err(|e| error!("failed to build tray menu: {e}"))?;

    let tray = TrayIconBuilder::with_id(TRAY_ICON_ID)
        .icon(APP_ICON)
        .menu(&menu)
        .on_tray_icon_event(on_tray_event)
        .on_menu_event(on_menu_event)
        .build(app)?;

    #[cfg(not(target_os = "linux"))]
    tray.set_tooltip(Some(APP_NAME))
        .inspect_err(|e| error!("failed to set tray tooltip {e}"))
        .ok();

    Ok(())
}
