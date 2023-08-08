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

use snarkvm::{cli::New as SnarkVMNew, file::AleoFile};

/// Create new Leo project
#[derive(Parser, Debug)]
pub struct New {
    #[clap(name = "NAME", help = "Set package name")]
    pub(crate) name: String,
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
        // Call the `aleo new` command from the Aleo SDK.
        let command =
            SnarkVMNew::try_parse_from([SNARKVM_COMMAND, &self.name]).map_err(CliError::failed_to_parse_new)?;
        let result = command.parse().map_err(CliError::failed_to_execute_new)?;

        // Log the output of the `aleo new` command.
        tracing::info!("{}", result);

        // Derive the program directory path.
        let mut package_path = context.dir()?;
        package_path.push(&self.name);

        // Initialize the Leo package in the directory created by `aleo new`.
        Package::<CurrentNetwork>::initialize(&self.name, &package_path)?;

        // Change the cwd to the Leo package directory to compile aleo files.
        std::env::set_current_dir(&package_path)
            .map_err(|err| PackageError::failed_to_set_cwd(package_path.display(), err))?;

        // Open the program manifest.
        let manifest = context.open_manifest()?;

        // Create a path to the build directory.
        let mut build_directory = package_path.clone();
        build_directory.push(BUILD_DIRECTORY_NAME);

        // Write the Aleo file into the build directory.
        AleoFile::create(&build_directory, manifest.program_id(), true)
            .map_err(PackageError::failed_to_create_aleo_file)?;

        // build_aleo_file.push(AleoFile::<Network>::main_file_name());
        //
        // println!("{}", build_aleo_file.display());
        //
        //
        // std::fs::File::create(build_aleo_file).map_err()
        // aleo_file.write_to(&build_aleo_file).map_err(PackageError::failed_to_write_aleo_file)?;

        // Open the `main.aleo` file path.
        let aleo_file = AleoFile::open(&package_path, manifest.program_id(), true)
            .map_err(PackageError::failed_to_open_aleo_file)?;

        let mut aleo_file_path = package_path.clone();
        aleo_file_path.push(AleoFile::<CurrentNetwork>::main_file_name());

        // Remove the Aleo file from the package directory.
        aleo_file.remove(&aleo_file_path).map_err(PackageError::failed_to_remove_aleo_file)?;

        Ok(())
    }
}
