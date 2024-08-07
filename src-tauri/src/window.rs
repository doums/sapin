// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use anyhow::Result;
use tauri::{PhysicalPosition, PhysicalSize, Size, WebviewWindow};
use tracing::{error, instrument, trace};

use crate::config::config::AppConfig;

#[instrument(skip_all)]
pub fn setup(window: &WebviewWindow, config: &AppConfig) -> Result<()> {
    trace!("___setup window");
    let canvas_size = config.shape.size();
    window
        .set_size(Size::Physical(PhysicalSize {
            width: canvas_size,
            height: canvas_size,
        }))
        .inspect_err(|e| error!("failed to resize window: {}", e))?;
    let monitor = window
        .current_monitor()
        .inspect_err(|e| error!("failed to get current monitor: {}", e))?;
    let offset = canvas_size / 2;
    if let Some(m) = monitor {
        let size = m.size();
        window
            .set_position(PhysicalPosition::new(
                (size.width / 2) - offset,
                (size.height / 2) - offset,
            ))
            .inspect_err(|e| error!("failed to move window: {}", e))?;
    }

    window
        .set_content_protected(config.protected)
        .inspect_err(|e| error!("failed to set content protected: {}", e))
        .ok();

    Ok(())
}
