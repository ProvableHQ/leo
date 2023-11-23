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
use snarkvm::console::{account::PrivateKey, prelude::Network};

use serde::Deserialize;
use std::{borrow::Cow, fs::File, io::Write, marker::PhantomData, path::Path};

pub static ENV_FILENAME: &str = ".env";

#[derive(Deserialize, Default)]
pub struct Env<N: Network> {
    data: String,
    _phantom: PhantomData<N>,
}

impl<N: Network> Env<N> {
    pub fn new() -> Result<Self> {
        Ok(Self { data: Self::template()?, _phantom: PhantomData })
    }

    pub fn from(data: String) -> Self {
        Self { data, _phantom: PhantomData }
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

    fn template() -> Result<String> {
        // Initialize an RNG.
        let rng = &mut rand::thread_rng();

        // Initialize a new development private key.
        let private_key = PrivateKey::<N>::new(rng)?;

        Ok(format!("NETWORK=testnet3\nPRIVATE_KEY={private_key}\n"))
    }
}
