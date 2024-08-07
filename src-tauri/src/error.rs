// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum CmdError {
    #[error("tauri error")]
    Tauri,
    #[error("internal error")]
    Internal,
    #[error("unknown error")]
    Unknown,
}

impl From<tauri::Error> for CmdError {
    fn from(_error: tauri::Error) -> Self {
        Self::Tauri
    }
}
