// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use std::env::current_dir;

use crate::{api::Api, config};
use anyhow::{anyhow, Result};
use leo_package::root::Manifest;
use std::{convert::TryFrom, path::PathBuf};

pub const PACKAGE_MANAGER_URL: &str = "https://api.aleo.pm/";

/// Project context, manifest, current directory etc
/// All the info that is relevant in most of the commands
#[derive(Clone)]
pub struct Context {
    /// Api client for Aleo PM
    pub api: Api,

    /// Path at which the command is called, None when default
    pub path: Option<PathBuf>,
}

impl Context {
    pub fn dir(&self) -> Result<PathBuf> {
        match &self.path {
            Some(path) => Ok(path.clone()),
            None => Ok(current_dir()?),
        }
    }

    /// Get package manifest for current context
    pub fn manifest(&self) -> Result<Manifest> {
        Ok(Manifest::try_from(self.dir()?.as_path())?)
    }
}

/// Create a new context for the current directory.
pub fn create_context(path: PathBuf) -> Result<Context> {
    let token = match config::read_token() {
        Ok(token) => Some(token),
        Err(_) => None,
    };

    let api = Api::new(PACKAGE_MANAGER_URL.to_string(), token);

    Ok(Context { api, path: Some(path) })
}

/// Returns project context.
pub fn get_context() -> Result<Context> {
    let token = match config::read_token() {
        Ok(token) => Some(token),
        Err(_) => None,
    };

    let api = Api::new(PACKAGE_MANAGER_URL.to_string(), token);

    Ok(Context { api, path: None })
}
