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

use aleo_std;
use leo_errors::{CliError, Result};
use leo_package::Manifest;

use aleo_std::aleo_dir;
use snarkvm::prelude::{Network, PrivateKey};
use std::{env::current_dir, path::PathBuf, str::FromStr};

/// Project context, manifest, current directory etc
/// All the info that is relevant in most of the commands
// TODO: Make `path` and `home` not pub, to prevent misuse through direct access.
#[derive(Clone)]
pub struct Context {
    /// Path at which the command is called, None when default
    pub path: Option<PathBuf>,
    /// Path to use for the Aleo registry, None when default
    pub home: Option<PathBuf>,
    /// Recursive flag.
    // TODO: Shift from callee to caller by including display method
    pub recursive: bool,
}

impl Context {
    pub fn new(path: Option<PathBuf>, home: Option<PathBuf>, recursive: bool) -> Result<Context> {
        Ok(Context { path, home, recursive })
    }

    /// Returns the path of the parent directory to the Leo package.
    pub fn parent_dir(&self) -> Result<PathBuf> {
        match &self.path {
            Some(path) => {
                let mut path = path.clone();
                path.pop();
                Ok(path)
            }
            None => Ok(current_dir().map_err(CliError::cli_io_error)?),
        }
    }

    /// Returns the path to the Leo package.
    pub fn dir(&self) -> Result<PathBuf> {
        match &self.path {
            Some(path) => Ok(path.clone()),
            None => Ok(current_dir().map_err(CliError::cli_io_error)?),
        }
    }

    /// Returns the path to the Aleo registry directory.
    pub fn home(&self) -> Result<PathBuf> {
        match &self.home {
            Some(path) => Ok(path.clone()),
            None => Ok(aleo_dir()),
        }
    }

    /// Opens the manifest file `program.json`.
    pub fn open_manifest(&self) -> Result<Manifest> {
        let path = self.dir()?;
        let manifest_path = path.join(leo_package::MANIFEST_FILENAME);
        let manifest = Manifest::read_from_file(manifest_path)?;
        Ok(manifest)
    }

    /// Returns the private key from the .env file specified in the directory.
    pub fn dotenv_private_key<N: Network>(&self) -> Result<PrivateKey<N>> {
        dotenvy::from_path(self.dir()?.join(".env")).map_err(|_| CliError::failed_to_get_private_key_from_env())?;
        // Load the private key from the environment.
        let private_key = dotenvy::var("PRIVATE_KEY").map_err(|_| CliError::failed_to_get_private_key_from_env())?;
        // Parse the private key.
        Ok(PrivateKey::<N>::from_str(&private_key)?)
    }

    /// Returns the endpoint from the .env file specified in the directory.
    pub fn dotenv_endpoint(&self) -> Result<String> {
        dotenvy::from_path(self.dir()?.join(".env")).map_err(|_| CliError::failed_to_get_endpoint_from_env())?;
        // Load the endpoint from the environment.
        Ok(dotenvy::var("ENDPOINT").map_err(|_| CliError::failed_to_get_endpoint_from_env())?)
    }

    /// Returns the network from the .env file specified in the directory.
    pub fn dotenv_network(&self) -> Result<String> {
        dotenvy::from_path(self.dir()?.join(".env")).map_err(|_| CliError::failed_to_get_network_from_env())?;
        // Load the network from the environment.
        Ok(dotenvy::var("NETWORK").map_err(|_| CliError::failed_to_get_network_from_env())?)
    }

    /// Returns the endpoint to interact with the network.
    /// If the `--endpoint` options is not provided, it will default to the one in the `.env` file.
    pub fn get_endpoint(&self, endpoint: &Option<String>) -> Result<String> {
        match endpoint {
            Some(endpoint) => Ok(endpoint.clone()),
            None => Ok(self.dotenv_endpoint()?),
        }
    }

    /// Returns the network name.
    /// If the `--network` options is not provided, it will default to the one in the `.env` file.
    pub fn get_network(&self, network: &Option<String>) -> Result<String> {
        match network {
            Some(network) => Ok(network.clone()),
            None => Ok(self.dotenv_network()?),
        }
    }

    /// Returns the private key.
    /// If the `--private-key` options is not provided, it will default to the one in the `.env` file.
    pub fn get_private_key<N: Network>(&self, private_key: &Option<String>) -> Result<PrivateKey<N>> {
        match private_key {
            Some(private_key) => Ok(PrivateKey::<N>::from_str(private_key)?),
            None => self.dotenv_private_key(),
        }
    }
}
