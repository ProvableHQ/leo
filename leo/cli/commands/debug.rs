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

use std::{fs, path::PathBuf};

use snarkvm::prelude::{Network, ProgramID, TestnetV0};

#[cfg(not(feature = "only_testnet"))]
use snarkvm::prelude::{CanaryV0, MainnetV0};

use leo_errors::UtilError;
use leo_retriever::{Manifest, NetworkName, Retriever};
use leo_span::Symbol;

use super::*;

/// Debugs an Aleo program through the interpreter.
#[derive(Parser, Debug)]
pub struct LeoDebug {
    #[arg(long, help = "Use these source files instead of finding source files through the project structure.", num_args = 1..)]
    pub(crate) paths: Vec<String>,

    #[arg(long, help = "The block height, accessible via block.height.", default_value = "0")]
    pub(crate) block_height: u32,

    #[arg(long, action, help = "Use the text user interface.")]
    pub(crate) tui: bool,

    #[clap(flatten)]
    pub(crate) compiler_options: BuildOptions,
}

impl Command for LeoDebug {
    type Input = <LeoBuild as Command>::Output;
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, context: Context) -> Result<Self::Input> {
        if self.paths.is_empty() {
            (LeoBuild { options: self.compiler_options.clone() }).execute(context)
        } else {
            Ok(())
        }
    }

    fn apply(self, context: Context, _: Self::Input) -> Result<Self::Output> {
        // Parse the network.
        let network = NetworkName::try_from(context.get_network(&self.compiler_options.network)?)?;
        match network {
            NetworkName::TestnetV0 => handle_debug::<TestnetV0>(&self, context),
            NetworkName::MainnetV0 => {
                #[cfg(feature = "only_testnet")]
                panic!("Mainnet chosen with only_testnet feature");
                #[cfg(not(feature = "only_testnet"))]
                return handle_debug::<MainnetV0>(&self, context);
            }
            NetworkName::CanaryV0 => {
                #[cfg(feature = "only_testnet")]
                panic!("Canary chosen with only_testnet feature");
                #[cfg(not(feature = "only_testnet"))]
                return handle_debug::<CanaryV0>(&self, context);
            }
        }
    }
}

fn handle_debug<N: Network>(command: &LeoDebug, context: Context) -> Result<()> {
    if command.paths.is_empty() {
        // Get the package path.
        let package_path = context.dir()?;
        let home_path = context.home()?;

        // Get the program id.
        let manifest = Manifest::read_from_dir(&package_path)?;
        let program_id = ProgramID::<N>::from_str(manifest.program())?;

        // Get the private key.
        let private_key = context.get_private_key(&None)?;
        let address = Address::try_from(&private_key)?;

        // Retrieve all local dependencies in post order
        let main_sym = Symbol::intern(&program_id.name().to_string());
        let mut retriever = Retriever::<N>::new(
            main_sym,
            &package_path,
            &home_path,
            context.get_endpoint(&command.compiler_options.endpoint)?.to_string(),
        )
        .map_err(|err| UtilError::failed_to_retrieve_dependencies(err, Default::default()))?;
        let mut local_dependencies =
            retriever.retrieve().map_err(|err| UtilError::failed_to_retrieve_dependencies(err, Default::default()))?;

        // Push the main program at the end of the list.
        local_dependencies.push(main_sym);

        let paths: Vec<PathBuf> = local_dependencies
            .into_iter()
            .map(|dependency| {
                let base_path = retriever.get_context(&dependency).full_path();
                base_path.join("src/main.leo")
            })
            .collect();

        let imports_directory = package_path.join("build/imports");

        let aleo_paths: Vec<PathBuf> = if let Ok(dir) = fs::read_dir(imports_directory) {
            dir.flat_map(|maybe_filename| maybe_filename.ok())
                .filter(|entry| entry.file_type().ok().map(|filetype| filetype.is_file()).unwrap_or(false))
                .flat_map(|entry| {
                    let path = entry.path();
                    if path.extension().map(|e| e == "aleo").unwrap_or(false) { Some(path) } else { None }
                })
                .collect()
        } else {
            Vec::new()
        };

        leo_interpreter::interpret(&paths, &aleo_paths, address, command.block_height, command.tui)
    } else {
        let private_key: PrivateKey<TestnetV0> = PrivateKey::from_str(leo_package::TEST_PRIVATE_KEY)?;
        let address = Address::try_from(&private_key)?;

        let leo_paths: Vec<PathBuf> = command
            .paths
            .iter()
            .filter(|path_str| path_str.ends_with(".leo"))
            .map(|path_str| path_str.into())
            .collect();
        let aleo_paths: Vec<PathBuf> = command
            .paths
            .iter()
            .filter(|path_str| !path_str.ends_with(".leo"))
            .map(|path_str| path_str.into())
            .collect();

        leo_interpreter::interpret(&leo_paths, &aleo_paths, address, command.block_height, command.tui)
    }
}
