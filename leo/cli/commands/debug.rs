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

use leo_package::{Package, ProgramData};
use leo_span::Symbol;

use snarkvm::prelude::TestnetV0;

use indexmap::IndexSet;
use std::path::PathBuf;

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

    fn apply(self, context: Context, input: Self::Input) -> Result<Self::Output> {
        handle_debug(&self, context, input)
    }
}

fn handle_debug(command: &LeoDebug, context: Context, package: Option<Package>) -> Result<()> {
    if command.paths.is_empty() {
        let package = package.unwrap();

        // Get the private key.
        let private_key = context.get_private_key(&None)?;
        let address = Address::try_from(&private_key)?;

        // Get the paths of all local Leo dependencies.
        let local_dependency_paths: Vec<PathBuf> = package
            .programs
            .iter()
            .flat_map(|program| match &program.data {
                ProgramData::SourcePath(path) => Some(path.clone()),
                ProgramData::Bytecode(..) => None,
            })
            .collect();

        let local_dependency_symbols: IndexSet<Symbol> = package
            .programs
            .iter()
            .flat_map(|program| match &program.data {
                ProgramData::SourcePath(..) => {
                    // It's a local Leo dependency.
                    Some(program.name)
                }
                ProgramData::Bytecode(..) => {
                    // It's a network dependency or local .aleo dependency.
                    None
                }
            })
            .collect();

        let imports_directory = package.imports_directory();

        // Get the paths to .aleo files in `imports` - but filter out the ones corresponding to local dependencies.
        let aleo_paths: Vec<PathBuf> = imports_directory
            .read_dir()
            .ok()
            .into_iter()
            .flatten()
            .flat_map(|maybe_filename| maybe_filename.ok())
            .filter(|entry| entry.file_type().ok().map(|filetype| filetype.is_file()).unwrap_or(false))
            .flat_map(|entry| {
                let path = entry.path();
                if let Some(filename) = leo_package::filename_no_aleo_extension(&path) {
                    let symbol = Symbol::intern(filename);
                    if local_dependency_symbols.contains(&symbol) { None } else { Some(path) }
                } else {
                    None
                }
            })
            .collect();

        // No need to keep this around while the interpreter runs.
        std::mem::drop(package);

        leo_interpreter::interpret(&local_dependency_paths, &aleo_paths, address, command.block_height, command.tui)
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
