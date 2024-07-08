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

use super::*;
use snarkvm::prelude::{CanaryV0, MainnetV0, TestnetV0};

use leo_retriever::NetworkName;

/// Create new Leo project
#[derive(Parser, Debug)]
pub struct New {
    #[clap(name = "NAME", help = "Set package name")]
    pub(crate) name: String,
    #[clap(short = 'n', long, help = "Name of the network to use", default_value = "testnet")]
    pub(crate) network: String,
    #[clap(
        short = 'e',
        long,
        help = "Endpoint to retrieve network state from.",
        default_value = "https://api.explorer.aleo.org/v1"
    )]
    pub(crate) endpoint: String,
}

impl Command for New {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, _: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, context: Context, _: Self::Input) -> Result<Self::Output> {
        // Parse the network.
        let network = NetworkName::try_from(self.network.as_str())?;

        // Derive the location of the parent directory to the project.
        let package_path = context.parent_dir()?;

        // Change the cwd to the Leo package directory to initialize all files.
        std::env::set_current_dir(&package_path)
            .map_err(|err| PackageError::failed_to_set_cwd(package_path.display(), err))?;

        // Initialize the package.
        match network {
            NetworkName::MainnetV0 => Package::initialize::<MainnetV0>(&self.name, &package_path, self.endpoint),
            NetworkName::TestnetV0 => Package::initialize::<TestnetV0>(&self.name, &package_path, self.endpoint),
            NetworkName::CanaryV0 => Package::initialize::<CanaryV0>(&self.name, &package_path, self.endpoint),
        }?;

        Ok(())
    }
}
