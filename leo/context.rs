// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use crate::{api::Api, config};
use leo_errors::{emitter::Handler, CliError, Result};
use leo_package::root::{LockFile, Manifest};
use leo_package::PackageFile as _;

use std::{convert::TryFrom, env::current_dir, path::PathBuf};

pub const PACKAGE_MANAGER_URL: &str = "https://api.aleo.pm/";

/// Project context, manifest, current directory etc
/// All the info that is relevant in most of the commands
#[derive(Clone)]
pub struct Context<'a> {
    /// Handler/Sink for error messages.
    pub handler: &'a Handler,

    /// Api client for Aleo PM
    pub api: Api,

    /// Path at which the command is called, None when default
    pub path: Option<PathBuf>,
}

impl Context<'_> {
    pub fn dir(&self) -> Result<PathBuf> {
        match &self.path {
            Some(path) => Ok(path.clone()),
            None => Ok(current_dir().map_err(CliError::cli_io_error)?),
        }
    }

    /// Get package manifest for current context.
    pub fn manifest(&self) -> Result<Manifest> {
        Ok(Manifest::try_from(self.dir()?.as_path())?)
    }

    /// Get lock file for current context.
    pub fn lock_file(&self) -> Result<LockFile> {
        Ok(LockFile::try_from(self.dir()?.as_path())?)
    }

    /// Check if lock file exists.
    pub fn lock_file_exists(&self) -> Result<bool> {
        Ok(LockFile::new().exists_at(&self.dir()?))
    }
}

/// Create a new context for the current directory.
pub fn create_context(handler: &Handler, path: PathBuf, api_url: Option<String>) -> Result<Context<'_>> {
    let token = config::read_token().ok();

    let api = Api::new(api_url.unwrap_or_else(|| PACKAGE_MANAGER_URL.to_string()), token);

    Ok(Context {
        handler,
        api,
        path: Some(path),
    })
}

/// Returns project context.
pub fn get_context(handler: &Handler, api_url: Option<String>) -> Result<Context<'_>> {
    let token = config::read_token().ok();

    let api = Api::new(api_url.unwrap_or_else(|| PACKAGE_MANAGER_URL.to_string()), token);

    Ok(Context {
        handler,
        api,
        path: None,
    })
}
