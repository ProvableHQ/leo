// Copyright (C) 2019-2023 Aleo Systems Inc.
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

//! The `.env` file.
use leo_errors::{PackageError, Result};
use snarkvm::console::account::PrivateKey;

use leo_retriever::NetworkName;
use serde::Deserialize;
use snarkvm::prelude::{MainnetV0, TestnetV0};
use std::{borrow::Cow, fs::File, io::Write, path::Path};

pub static ENV_FILENAME: &str = ".env";

// TODO: Should this be generic over network?
#[derive(Deserialize, Default)]
pub struct Env {
    data: String,
}

impl Env {
    pub fn new(network: NetworkName) -> Result<Self> {
        Ok(Self { data: Self::template(network)? })
    }

    pub fn from(data: String) -> Self {
        Self { data }
    }

    pub fn exists_at(path: &Path) -> bool {
        let mut path = Cow::from(path);
        if path.is_dir() {
            path.to_mut().push(ENV_FILENAME);
        }
        path.exists()
    }

    pub fn write_to(self, path: &Path) -> Result<()> {
        let mut path = Cow::from(path);
        if path.is_dir() {
            path.to_mut().push(ENV_FILENAME);
        }

        let mut file = File::create(&path).map_err(PackageError::io_error_env_file)?;
        file.write_all(self.data.as_bytes()).map_err(PackageError::io_error_env_file)?;
        Ok(())
    }

    fn template(network: NetworkName) -> Result<String> {
        // Initialize an RNG.
        let rng = &mut rand::thread_rng();

        // Initialize a new development private key.
        let private_key = match network {
            NetworkName::MainnetV0 => PrivateKey::<MainnetV0>::new(rng)?.to_string(),
            NetworkName::TestnetV0 => PrivateKey::<TestnetV0>::new(rng)?.to_string(),
        };

        Ok(format!("NETWORK={network}\nPRIVATE_KEY={private_key}\n"))
    }
}
