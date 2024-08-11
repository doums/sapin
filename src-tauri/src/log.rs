// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::EnvFilter;

pub fn setup_tracing() {
    let filter = EnvFilter::builder()
        .with_default_directive({
            if cfg!(dev) {
                LevelFilter::DEBUG.into()
            } else {
                LevelFilter::INFO.into()
            }
        })
        .from_env()
        .unwrap();
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .compact()
        .init();
}
