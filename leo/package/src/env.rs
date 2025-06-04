// Copyright (C) 2019-2025 Provable Inc.
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

use crate::NetworkName;

use leo_errors::{CliError, PackageError, Result};

use std::{fmt, fs, path::Path};

pub const ENV_FILENAME: &str = ".env";

#[derive(Clone, Debug)]
pub struct Env {
    pub network: NetworkName,
    pub private_key: String,
    pub endpoint: String,
}

impl Env {
    pub fn new(network: NetworkName, private_key: String, endpoint: String) -> Self {
        Env { network, private_key, endpoint }
    }

    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), PackageError> {
        let contents = self.to_string();
        fs::write(path, contents).map_err(PackageError::io_error_env_file)
    }

    pub fn read_from_file_or_environment<P: AsRef<Path>>(path: P) -> Result<Self> {
        // Read the `.env` file from the given directory.
        // If the file does not exist, then attempt to read it from its parent directory recursively until
        // there are no more parent directories.
        let path = path.as_ref().to_path_buf();
        let mut contents = String::new();
        let mut current_path = path;
        while current_path.exists() {
            let env_path = current_path.join(ENV_FILENAME);
            if env_path.exists() {
                contents = fs::read_to_string(env_path).map_err(PackageError::io_error_env_file)?;
                break;
            }
            current_path = match current_path.parent() {
                Some(parent) => parent.to_path_buf(),
                None => break,
            };
        }

        let mut network: Option<String> = None;
        let mut private_key: Option<String> = None;
        let mut endpoint: Option<String> = None;

        for line in contents.lines() {
            if let Some((lhs, rhs)) = line.split_once('=') {
                match lhs.trim() {
                    "NETWORK" => network = Some(rhs.to_string()),
                    "PRIVATE_KEY" => private_key = Some(rhs.to_string()),
                    "ENDPOINT" => endpoint = Some(rhs.to_string()),
                    _ => {}
                }
            }
        }

        for (variable, name) in
            [(&mut network, "NETWORK"), (&mut private_key, "PRIVATE_KEY"), (&mut endpoint, "ENDPOINT")]
        {
            if let Ok(env_var_value) = std::env::var(name) {
                if !env_var_value.is_empty() {
                    *variable = Some(env_var_value);
                }
            }
        }

        let network: Option<NetworkName> = network.and_then(|net| net.parse().ok());

        match (network, private_key, endpoint) {
            (Some(network), Some(private_key), Some(endpoint)) => Ok(Env { network, private_key, endpoint }),
            (None, _, _) => Err(CliError::failed_to_get_network_from_env().into()),
            (_, None, _) => Err(CliError::failed_to_get_private_key_from_env().into()),
            (_, _, None) => Err(CliError::failed_to_get_endpoint_from_env().into()),
        }
    }
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NETWORK={}\nPRIVATE_KEY={}\nENDPOINT={}\n", self.network, self.private_key, self.endpoint)
    }
}
