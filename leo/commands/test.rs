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

use super::build::BuildOptions;
use crate::{commands::Command, context::Context};
use leo_compiler::compiler::{thread_leaked_context, Compiler};
use leo_errors::{CliError, Result};
use leo_package::{
    inputs::*,
    outputs::{OutputsDirectory, OUTPUTS_DIRECTORY_NAME},
    source::{MainFile, MAIN_FILENAME, SOURCE_DIRECTORY_NAME},
};

use indexmap::IndexMap;
use std::{convert::TryFrom, path::PathBuf, time::Instant};
use structopt::StructOpt;
use tracing::span::Span;

/// Build program and run tests command
#[derive(StructOpt, Debug)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Test {
    #[structopt(short = "f", long = "file", name = "file")]
    pub(crate) files: Vec<PathBuf>,

    #[structopt(flatten)]
    pub(crate) compiler_options: BuildOptions,
}

impl Command for Test {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Test")
    }

    fn prelude(&self, _: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, context: Context, _: Self::Input) -> Result<Self::Output> {
        // Get the package name
        let package_name = context.manifest()?.get_package_name();

        // Sanitize the package path to the root directory
        let mut package_path = context.dir()?;
        if package_path.is_file() {
            package_path.pop();
        }

        let mut to_test: Vec<PathBuf> = Vec::new();

        // if -f flag was used, then we'll test only this files
        if !self.files.is_empty() {
            to_test.extend(self.files.iter().cloned());

        // if args were not passed - try main file
        } else if MainFile::exists_at(&package_path) {
            let mut file_path = package_path.clone();
            file_path.push(SOURCE_DIRECTORY_NAME);
            file_path.push(MAIN_FILENAME);
            to_test.push(file_path);

        // when no main file and no files marked - error
        } else {
            return Err(CliError::program_file_does_not_exist(package_path.to_string_lossy()).into());
        }

        // Construct the path to the output directory;
        let mut output_directory = package_path.clone();
        output_directory.push(OUTPUTS_DIRECTORY_NAME);

        // Create the output directory
        OutputsDirectory::create(&package_path)?;

        // Finally test every passed file
        for file_path in to_test {
            tracing::info!("Running tests in file {:?}", file_path);

            let input_pairs = match InputPairs::try_from(package_path.as_path()) {
                Ok(pairs) => pairs,
                Err(_) => {
                    tracing::warn!("Unable to find inputs, ignore this message or put them into /inputs folder");
                    InputPairs::new()
                }
            };

            let timer = Instant::now();
            let program = Compiler::parse_program_without_input(
                package_name.clone(),
                file_path,
                output_directory.clone(),
                thread_leaked_context(),
                Some(self.compiler_options.clone().into()),
                IndexMap::new(),
                Some(self.compiler_options.clone().into()),
            )?;

            let temporary_program = program;
            let (passed, failed) = temporary_program.compile_test(input_pairs)?;
            let time_taken = timer.elapsed().as_millis();

            if failed == 0 {
                tracing::info!(
                    "Tests passed in {} milliseconds. {} passed; {} failed;\n",
                    time_taken,
                    passed,
                    failed
                );
            } else {
                tracing::error!(
                    "Tests failed in {} milliseconds. {} passed; {} failed;\n",
                    time_taken,
                    passed,
                    failed
                );
            }
        }

        Ok(())
    }
}
