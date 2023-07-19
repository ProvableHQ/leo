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

/// Deploys an Aleo program.
#[derive(Parser, Debug)]
pub struct Deploy;

impl Command for Deploy {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, _: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, context: Context, _: Self::Input) -> Result<Self::Output> {
        // Open the Leo build/ directory
        let path = context.dir()?;
        let build_directory = BuildDirectory::open(&path).map_err(|_| CliError::needs_leo_build())?;

        // Change the cwd to the Leo build/ directory to deploy aleo files.
        std::env::set_current_dir(&build_directory)
            .map_err(|err| PackageError::failed_to_set_cwd(build_directory.display(), err))?;

        // Unset the Leo panic hook.
        let _ = std::panic::take_hook();

        // Call the `node` command.
        println!();
        let command = SnarkVMDeploy::try_parse_from([ALEO_CLI_COMMAND]).map_err(CliError::failed_to_parse_aleo_node)?;
        let res = command.parse().map_err(CliError::failed_to_execute_aleo_node)?;

        // Log the output of the `node` command.
        tracing::info!("{}", res);

        Ok(())
    }
}
