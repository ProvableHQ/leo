// Copyright (C) 2019-2020 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::api::Api;
use anyhow::Result;

pub const PACKAGE_MANAGER_URL: &str = "https://api.aleo.pm/";

/// Project context, manifest, current directory etc
/// All the info that is relevant in most of the commands
pub struct Context {
    // will contain manifest
    pub api: Api,
}

impl Context {}

/// Create a new context for the current directory.
pub fn create_context() -> Result<Context> {
    Ok(Context {
        api: Api::new(PACKAGE_MANAGER_URL.to_string()),
    })
}

/// Returns project context.
pub fn get_context() -> Result<Context> {
    Ok(Context {
        api: Api::new(PACKAGE_MANAGER_URL.to_string()),
    })
}
