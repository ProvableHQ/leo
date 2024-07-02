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
use leo_retriever::NetworkName;
use snarkvm::console::account::PrivateKey;

use snarkvm::prelude::{MainnetV0, Network, TestnetV0};

use serde::Deserialize;
use std::{borrow::Cow, fs::File, io::Write, path::Path};

pub static ENV_FILENAME: &str = ".env";

#[derive(Deserialize)]
pub struct Env<N: Network> {
    #[serde(bound(deserialize = ""))]
    private_key: PrivateKey<N>,
    endpoint: String,
}

impl<N: Network> Env<N> {
    pub fn new(private_key: Option<PrivateKey<N>>, endpoint: String) -> Result<Self> {
        // Initialize an RNG.
        let rng = &mut rand::thread_rng();

        // Generate a development private key.
        let private_key = match private_key {
            Some(private_key) => private_key,
            None => PrivateKey::<N>::new(rng)?,
        };

        Ok(Self { private_key, endpoint })
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
        file.write_all(self.to_string().as_bytes()).map_err(PackageError::io_error_env_file)?;
        Ok(())
    }
}

impl<N: Network> ToString for Env<N> {
    fn to_string(&self) -> String {
        // Get the network name.
        let network = match N::ID {
            MainnetV0::ID => NetworkName::MainnetV0,
            TestnetV0::ID => NetworkName::TestnetV0,
            _ => unimplemented!("Unsupported network"),
        };
        // Return the formatted string.
        format!("NETWORK={network}\nPRIVATE_KEY={}\nENDPOINT={}\n", self.private_key, self.endpoint)
    }
}
