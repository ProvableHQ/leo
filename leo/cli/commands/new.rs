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

use super::*;

use leo_ast::NetworkName;
use leo_package::Package;

/// Create new Leo project
#[derive(Parser, Debug)]
pub struct LeoNew {
    #[clap(name = "NAME", help = "Set package name")]
    pub(crate) name: String,
    #[clap(short = 'n', long, help = "Name of the network to use", default_value = "testnet")]
    pub(crate) network: String,
    #[clap(
        short = 'e',
        long,
        help = "Endpoint to retrieve network state from.",
        default_value = "http://localhost:3030"
    )]
    pub(crate) endpoint: String,
}

impl Command for LeoNew {
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
        let network: NetworkName = self.network.parse()?;

        // Derive the location of the parent directory to the project.
        let package_path = context.parent_dir()?;

        // Change the cwd to the Leo package directory to initialize all files.
        std::env::set_current_dir(&package_path)
            .map_err(|err| PackageError::failed_to_set_cwd(package_path.display(), err))?;

        let full_path = Package::initialize(&self.name, &package_path, network, &self.endpoint)?;

        println!("Created program {} at `{}`.", self.name.bold(), full_path.display());

        Ok(())
    }
}
