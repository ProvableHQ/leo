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

use leo_errors::{CliError, Result};

use std::{
    fs::{
        create_dir_all,
        File,
        {self},
    },
    io::prelude::*,
    path::{Path, PathBuf},
};

use dirs::home_dir;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

pub const LEO_CREDENTIALS_FILE: &str = "credentials";
pub const LEO_CONFIG_FILE: &str = "config.toml";
pub const LEO_USERNAME_FILE: &str = "username";

lazy_static! {
    pub static ref LEO_CONFIG_DIRECTORY: PathBuf = {
        let mut path = home_dir().expect("Invalid home directory");
        path.push(".leo");
        path
    };
    pub static ref LEO_CREDENTIALS_PATH: PathBuf = {
        let mut path = LEO_CONFIG_DIRECTORY.to_path_buf();
        path.push(LEO_CREDENTIALS_FILE);
        path
    };
    pub static ref LEO_USERNAME_PATH: PathBuf = {
        let mut path = LEO_CONFIG_DIRECTORY.to_path_buf();
        path.push(LEO_USERNAME_FILE);
        path
    };
    pub static ref LEO_CONFIG_PATH: PathBuf = {
        let mut path = LEO_CONFIG_DIRECTORY.to_path_buf();
        path.push(LEO_CONFIG_FILE);
        path
    };
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Update {
    pub automatic: bool,
}

impl Default for Update {
    fn default() -> Self {
        Self { automatic: true }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub update: Update,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            update: Update::default(),
        }
    }
}

impl Config {
    /// Read the config from the `config.toml` file
    pub fn read_config() -> Result<Self> {
        let config_dir = LEO_CONFIG_DIRECTORY.clone();
        let config_path = LEO_CONFIG_PATH.clone();

        if !Path::exists(&config_path) {
            // Create a new default `config.toml` file if it doesn't already exist
            create_dir_all(&config_dir).map_err(CliError::cli_io_error)?;

            let default_config_string =
                toml::to_string(&Config::default()).map_err(CliError::failed_to_convert_to_toml)?;

            fs::write(&config_path, default_config_string).map_err(CliError::cli_io_error)?;
        }

        let toml_string = match fs::read_to_string(&config_path) {
            Ok(mut toml) => {
                // If the config is using an incorrect format, rewrite it.
                if toml::from_str::<Config>(&toml).is_err() {
                    let default_config_string =
                        toml::to_string(&Config::default()).map_err(CliError::failed_to_convert_to_toml)?;
                    fs::write(&config_path, default_config_string.clone()).map_err(CliError::cli_io_error)?;
                    toml = default_config_string;
                }

                toml
            }
            Err(_) => {
                create_dir_all(&config_dir).map_err(CliError::cli_io_error)?;
                toml::to_string(&Config::default()).map_err(CliError::failed_to_convert_to_toml)?
            }
        };

        // Parse the contents into the `Config` struct
        let config: Config = toml::from_str(&toml_string).map_err(CliError::failed_to_convert_from_toml)?;

        Ok(config)
    }

    /// Update the `automatic` configuration in the `config.toml` file.
    pub fn set_update_automatic(automatic: bool) -> Result<()> {
        let mut config = Self::read_config()?;

        if config.update.automatic != automatic {
            config.update.automatic = automatic;

            // Update the config file
            let config_path = LEO_CONFIG_PATH.clone();
            fs::write(
                &config_path,
                toml::to_string(&config).map_err(CliError::failed_to_convert_to_toml)?,
            )
            .map_err(CliError::cli_io_error)?;
        }

        Ok(())
    }
}

pub fn write_token_and_username(token: &str, username: &str) -> Result<()> {
    let config_dir = LEO_CONFIG_DIRECTORY.clone();

    // Create Leo config directory if it not exists
    if !Path::new(&config_dir).exists() {
        create_dir_all(&config_dir).map_err(CliError::cli_io_error)?;
    }

    let mut credentials = File::create(&LEO_CREDENTIALS_PATH.to_path_buf()).map_err(CliError::cli_io_error)?;
    credentials
        .write_all(token.as_bytes())
        .map_err(CliError::cli_io_error)?;

    let mut username_file = File::create(&LEO_USERNAME_PATH.to_path_buf()).map_err(CliError::cli_io_error)?;
    username_file
        .write_all(username.as_bytes())
        .map_err(CliError::cli_io_error)?;

    Ok(())
}

pub fn read_token() -> Result<String> {
    let mut credentials = File::open(&LEO_CREDENTIALS_PATH.to_path_buf()).map_err(CliError::cli_io_error)?;
    let mut buf = String::new();
    credentials.read_to_string(&mut buf).map_err(CliError::cli_io_error)?;
    Ok(buf)
}

pub fn read_username() -> Result<String> {
    let mut username = File::open(&LEO_USERNAME_PATH.to_path_buf()).map_err(CliError::cli_io_error)?;
    let mut buf = String::new();
    username.read_to_string(&mut buf).map_err(CliError::cli_io_error)?;
    Ok(buf)
}

pub fn remove_token_and_username() -> Result<()> {
    if let Err(err) = fs::remove_file(&LEO_CREDENTIALS_PATH.to_path_buf()) {
        return Err(CliError::remove_token_and_username(err).into());
    }
    if let Err(err) = fs::remove_file(&LEO_USERNAME_PATH.to_path_buf()) {
        return Err(CliError::remove_token_and_username(err).into());
    }
    Ok(())
}
