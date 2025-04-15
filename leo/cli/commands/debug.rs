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

use snarkvm::prelude::TestnetV0;

use std::{fs, path::PathBuf};

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
    type Input = Option<Package>;
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, context: Context) -> Result<Self::Input> {
        if self.paths.is_empty() {
            let package = LeoBuild { options: self.compiler_options.clone() }.execute(context)?;
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

        // Retrieve all local dependencies in post order
        let local_dependency_paths: Vec<PathBuf> = package
            .programs
            .iter()
            .flat_map(|program| match &program.data {
                ProgramData::SourcePath(path) => Some(path.clone()),
                ProgramData::Bytecode(..) => None,
            })
            .collect();

        let imports_directory = package.imports_directory();

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
