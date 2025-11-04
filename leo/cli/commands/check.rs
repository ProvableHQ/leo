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

use std::path::Path;

use indexmap::IndexMap;
use leo_ast::{NetworkName, Stub};
use leo_errors::{Result, UtilError};
use leo_linter::Linter;
use leo_package::Package;
use leo_span::Symbol;
use snarkvm::prelude::{CanaryV0, MainnetV0, TestnetV0};

/// Perform a check on leo programs or modules
/// for common errors, mistakes, coding practices or code quality issues etc.
#[derive(Parser, Debug)]
pub struct LeoCheck {
    #[clap(flatten)]
    pub(crate) env_override: EnvOptions,
    #[clap(flatten)]
    pub(crate) build_options: BuildOptions,
}

impl Command for LeoCheck {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, _context: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, context: Context, _input: Self::Input) -> Result<Self::Output> {
        self.check(context)
    }
}

impl LeoCheck {
    fn check(&self, context: Context) -> Result<()> {
        // Get the package path and home directory.
        let package_path = context.dir()?;
        let home_path = context.home()?;

        let command = self;

        // Get the network, defaulting to `TestnetV0` if none is specified.
        let network = match get_network(&command.env_override.network) {
            Ok(network) => network,
            Err(_) => {
                println!("⚠️ No network specified, defaulting to 'testnet'.");
                NetworkName::TestnetV0
            }
        };

        // Get the endpoint, if it is provided.
        let endpoint = get_endpoint(&command.env_override.endpoint).ok();

        let package = if command.build_options.build_tests {
            Package::from_directory_with_tests(
                &package_path,
                &home_path,
                command.build_options.no_cache,
                command.build_options.no_local,
                Some(network),
                endpoint.as_deref(),
            )?
        } else {
            Package::from_directory(
                &package_path,
                &home_path,
                command.build_options.no_cache,
                command.build_options.no_local,
                Some(network),
                endpoint.as_deref(),
            )?
        };

        // Check the manifest for the compiler version.
        // If it does not match, warn the user and continue.
        if package.manifest.leo != env!("CARGO_PKG_VERSION") {
            tracing::warn!(
                "The Leo compiler version in the manifest ({}) does not match the current version ({}).",
                package.manifest.leo,
                env!("CARGO_PKG_VERSION")
            );
        }

        let outputs_directory = package.outputs_directory();
        let build_directory = package.build_directory();
        let imports_directory = package.imports_directory();
        let source_directory = package.source_directory();
        let main_source_path = source_directory.join("main.leo");

        for dir in [&outputs_directory, &imports_directory] {
            std::fs::create_dir_all(dir).map_err(|err| {
                UtilError::util_file_io_error(format_args!("Couldn't create directory {}", dir.display()), err)
            })?;
        }

        // Initialize error handler.
        let handler = Handler::default();

        let mut stubs = IndexMap::new();

        for program in package.programs.iter() {
            let (bytecode, _build_path) = match &program.data {
                leo_package::ProgramData::Bytecode(bytecode) => {
                    // This was a network dependency or local .aleo dependency, and we have its bytecode.
                    (bytecode.clone(), imports_directory.join(format!("{}.aleo", program.name)))
                }
                leo_package::ProgramData::SourcePath { directory, source } => {
                    // This is a local dependency, so we must compile it.
                    // We would need to build the main directory also just for the tests, otherwise,
                    // it's not needed.
                    let build_path = if source == &main_source_path {
                        build_directory.join("main.aleo")
                    } else {
                        imports_directory.join(format!("{}.aleo", program.name))
                    };

                    if *source == main_source_path && !command.build_options.build_tests {
                        continue;
                    }

                    // Load the manifest in local dependency.
                    let source_dir = directory.join("src");
                    let bytecode = compile_leo_source_directory(
                        source, // entry file
                        &source_dir,
                        program.name,
                        program.is_test,
                        &outputs_directory,
                        &handler,
                        command.build_options.clone(),
                        stubs.clone(),
                        network,
                    )?;

                    (bytecode, build_path)
                }
            };

            // Track the Stub.
            let stub = match network {
                NetworkName::MainnetV0 => leo_disassembler::disassemble_from_str::<MainnetV0>(program.name, &bytecode),
                NetworkName::TestnetV0 => leo_disassembler::disassemble_from_str::<TestnetV0>(program.name, &bytecode),
                NetworkName::CanaryV0 => leo_disassembler::disassemble_from_str::<CanaryV0>(program.name, &bytecode),
            }?;

            stubs.insert(program.name, stub);
        }

        for program in &package.programs {
            if let leo_package::ProgramData::SourcePath { directory, source } = &program.data
                && (source == &main_source_path || *directory == package.tests_directory())
            {
                check_leo_source_directory(
                    if !program.is_test { Some(source_directory.as_path()) } else { None },
                    source.as_path(),
                    program.name,
                    program.is_test,
                    &handler,
                    stubs.clone(),
                    network,
                )?
            }
        }

        Ok(())
    }
}

fn check_leo_source_directory(
    source_directory: Option<&Path>,
    entry_file_path: &Path,
    program_name: Symbol,
    is_test: bool,
    handler: &Handler,
    stubs: IndexMap<Symbol, Stub>,
    network: NetworkName,
) -> Result<()> {
    // Create a new instance of the Leo linter.
    let mut linter = Linter::new(Some(program_name.to_string()), handler.clone(), is_test, stubs, network);
    linter.lint_leo_source_directory(entry_file_path, source_directory)?;
    Ok(())
}
