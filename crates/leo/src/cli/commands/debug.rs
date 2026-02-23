// Copyright (C) 2019-2026 Provable Inc.
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

use leo_package::Package;

use snarkvm::prelude::TestnetV0;

use std::path::PathBuf;

use super::*;

/// Debugs an Aleo program through the interpreter.
#[derive(Parser, Debug)]
pub struct LeoDebug {
    #[arg(long, help = "Use these source files instead of finding source files through the project structure. Program submodules aren't supported here.", num_args = 1..)]
    pub(crate) paths: Vec<String>,
    #[arg(long, help = "The block height, accessible via block.height.", default_value = "0")]
    pub(crate) block_height: u32,
    #[arg(
        long,
        help = "The block timestamp, accessible via block.timestamp.",
        default_value = "chrono::Utc::now().timestamp()"
    )]
    pub(crate) block_timestamp: i64,
    #[arg(long, action, help = "Use the text user interface.")]
    pub(crate) tui: bool,
    #[clap(flatten)]
    pub(crate) compiler_options: BuildOptions,
    #[clap(flatten)]
    pub(crate) env_override: EnvOptions,
}

impl Command for LeoDebug {
    type Input = Option<Package>;
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, context: Context) -> Result<Self::Input> {
        if self.paths.is_empty() {
            let package = LeoBuild { options: self.compiler_options.clone(), env_override: self.env_override.clone() }
                .execute(context)?;
            Ok(Some(package))
        } else {
            Ok(None)
        }
    }

    fn apply(self, _: Context, input: Self::Input) -> Result<Self::Output> {
        handle_debug(&self, input)
    }
}

fn handle_debug(command: &LeoDebug, package: Option<Package>) -> Result<()> {
    // Get the network.
    let network_name = get_network(&command.env_override.network)?;

    if command.paths.is_empty() {
        let package = package.unwrap();

        // Get the private key.
        let private_key = get_private_key::<TestnetV0>(&Some(leo_ast::TEST_PRIVATE_KEY.to_string()))?;

        // Get the paths of all local Leo dependencies.
        let local_dependency_paths = collect_leo_paths(&package);
        let aleo_paths = collect_aleo_paths(&package);

        // No need to keep this around while the interpreter runs.
        std::mem::drop(package);

        leo_interpreter::interpret(
            &local_dependency_paths,
            &aleo_paths,
            private_key.to_string(),
            command.block_height,
            command.block_timestamp,
            command.tui,
            network_name,
        )
    } else {
        // Program that have submodules aren't supported in this mode.
        let private_key: PrivateKey<TestnetV0> = PrivateKey::from_str(leo_ast::TEST_PRIVATE_KEY)?;

        let leo_paths: Vec<(PathBuf, Vec<PathBuf>)> = command
            .paths
            .iter()
            .filter(|path_str| path_str.ends_with(".leo"))
            .map(|path_str| (path_str.into(), vec![]))
            .collect();
        let aleo_paths: Vec<PathBuf> = command
            .paths
            .iter()
            .filter(|path_str| !path_str.ends_with(".leo"))
            .map(|path_str| path_str.into())
            .collect();

        leo_interpreter::interpret(
            &leo_paths,
            &aleo_paths,
            private_key.to_string(),
            command.block_height,
            command.block_timestamp,
            command.tui,
            network_name,
        )
    }
}
