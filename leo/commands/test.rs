// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use crate::{commands::Command, context::Context};

use anyhow::{anyhow, Result};
use structopt::StructOpt;

use leo_compiler::{compiler::Compiler, group::targets::edwards_bls12::EdwardsGroupType};
use leo_package::{
    inputs::*,
    outputs::{OutputsDirectory, OUTPUTS_DIRECTORY_NAME},
    source::{LibraryFile, MainFile, LIBRARY_FILENAME, MAIN_FILENAME, SOURCE_DIRECTORY_NAME},
};

use snarkvm_curves::edwards_bls12::Fq;

use std::{convert::TryFrom, time::Instant};
use tracing::span::Span;

/// Build program and run tests command
#[derive(StructOpt, Debug, Default)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Test {
    #[structopt(short = "f", long = "file", name = "file")]
    files: Vec<String>,
}

impl Test {
    pub fn new(files: Vec<String>) -> Test {
        Test { files }
    }
}

impl Command for Test {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Test")
    }

    fn prelude(&self) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, ctx: Context, _: Self::Input) -> Result<Self::Output> {
        let path = ctx.dir()?;

        // Get the package name
        let package_name = ctx.manifest()?.get_package_name();

        // Sanitize the package path to the root directory
        let mut package_path = path;
        if package_path.is_file() {
            package_path.pop();
        }

        let mut file_path = package_path.clone();
        file_path.push(SOURCE_DIRECTORY_NAME);

        // Verify a main or library file exists
        if MainFile::exists_at(&package_path) {
            file_path.push(MAIN_FILENAME);
        } else if LibraryFile::exists_at(&package_path) {
            file_path.push(LIBRARY_FILENAME);
        } else {
            return Err(anyhow!(
                "Program file does not exist {}",
                package_path.to_string_lossy()
            ));
        }

        // Construct the path to the output directory;
        let mut output_directory = package_path.clone();
        output_directory.push(OUTPUTS_DIRECTORY_NAME);

        // Create the output directory
        OutputsDirectory::create(&package_path)?;

        // Start the timer
        let start = Instant::now();

        // Parse the current main program file
        let program =
            Compiler::<Fq, EdwardsGroupType>::parse_program_without_input(package_name, file_path, output_directory)?;

        // Parse all inputs as input pairs
        let pairs = match InputPairs::try_from(package_path.as_path()) {
            Ok(pairs) => pairs,
            Err(_) => {
                // tracing::warn!("Could not find inputs, if it is intentional - ignore \
                //                 or use -i or --inputs option to specify inputs directory");
                tracing::warn!("Unable to find inputs, ignore this message or put them into /inputs folder");
                InputPairs::new()
            }
        };

        // Run tests
        let temporary_program = program;
        let (passed, failed) = temporary_program.compile_test_constraints(pairs)?;

        // Set the result of the test command to passed if no tests failed.
        if failed == 0 {
            // Begin "Done" context for console logging
            tracing::span!(tracing::Level::INFO, "Done").in_scope(|| {
                tracing::info!(
                    "Tests passed in {} milliseconds. {} passed; {} failed;\n",
                    start.elapsed().as_millis(),
                    passed,
                    failed
                );
            });
        } else {
            // Begin "Done" context for console logging
            tracing::span!(tracing::Level::ERROR, "Done").in_scope(|| {
                tracing::error!(
                    "Tests failed in {} milliseconds. {} passed; {} failed;\n",
                    start.elapsed().as_millis(),
                    passed,
                    failed
                );
            });
        };

        Ok(())
    }
}
