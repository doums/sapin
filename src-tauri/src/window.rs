// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use anyhow::Result;
use tauri::{PhysicalPosition, PhysicalSize, Size, WebviewWindow};
use tracing::{debug, error, info, instrument, trace};

use crate::config::app_config::AppConfig;

// NOTE: Tauri `center` implementation fails to center the window properly
// on the screen, so let's implement it
#[instrument(skip(window))]
fn center(window: &WebviewWindow, size: u32) -> Result<()> {
    trace!("center window");
    let monitor = window
        .current_monitor()
        .inspect_err(|e| error!("failed to get current monitor: {}", e))?;
    let offset = size / 2;
    if let Some(m) = monitor {
        let res = m.size();
        debug!("monitor resolution: {:?}", res);
        let position = PhysicalPosition::new((res.width / 2) - offset, (res.height / 2) - offset);
        debug!("centering window at: {:?}", position);
        window
            .set_position(position)
            .inspect_err(|e| error!("failed to move window: {}", e))?;
    }
    Ok(())
}

#[instrument(skip(window))]
fn resize(window: &WebviewWindow, size: u32) -> Result<()> {
    trace!("resize window");
    window
        .set_size(Size::Physical(PhysicalSize {
            width: size,
            height: size,
        }))
        .inspect_err(|e| error!("failed to resize window: {}", e))?;
    Ok(())
}

#[instrument(skip_all)]
pub fn setup(window: &WebviewWindow, config: &AppConfig) -> Result<()> {
    debug!("setup window");
    let scale = window
        .scale_factor()
        .inspect_err(|e| error!("failed to get window scale factor: {}", e))?;
    let canvas_size = config.shape.size();
    let size = if scale != 1.0 {
        debug!("scale factor: {scale}");
        (scale * canvas_size as f64).round_ties_even() as u32
    } else {
        canvas_size
    };
    debug!("window pixel size: {size}");
    resize(window, size)?;
    center(window, size)?;

    window
        .set_content_protected(config.protected)
        .inspect_err(|e| error!("failed to set content protected: {}", e))
        .ok();

    info!("window setup done");
    Ok(())
}
