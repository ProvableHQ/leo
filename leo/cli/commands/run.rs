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

use snarkvm::cli::Run as SnarkVMRun;

/// Build, Prove and Run Leo program with inputs
#[derive(Parser, Debug)]
pub struct LeoRun {
    #[clap(name = "NAME", help = "The name of the program to run.", default_value = "main")]
    pub(crate) name: String,
    #[clap(
        name = "INPUTS",
        help = "The program inputs e.g. `1u32` or `{ owner: ...}`. This cannot handle record ciphertexts (yet). "
    )]
    pub(crate) inputs: Vec<String>,
    #[clap(flatten)]
    pub(crate) compiler_options: BuildOptions,
    #[clap(flatten)]
    pub(crate) env_override: EnvOptions,
}

impl Command for LeoRun {
    type Input = <LeoBuild as Command>::Output;
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, context: Context) -> Result<Self::Input> {
        (LeoBuild { options: self.compiler_options.clone(), env_override: self.env_override.clone() }).execute(context)
    }

    fn apply(self, context: Context, _: Self::Input) -> Result<Self::Output> {
        handle_run(self, context)
    }
}

// A helper function to handle the run command.
fn handle_run(mut command: LeoRun, context: Context) -> Result<<LeoRun as Command>::Output> {
    // Compose the `run` command.
    // The argument "--" is the separator, used to make sure snarkvm doesn't try to parse negative
    // values as CLI flags.
    let mut arguments = vec![SNARKVM_COMMAND.to_string(), command.name, "--".to_string()];
    arguments.append(&mut command.inputs);

    // Open the Leo build/ directory
    let path = context.dir()?;
    let build_directory = path.join(leo_package::BUILD_DIRECTORY);

    // Change the cwd to the Leo build/ directory to compile aleo files.
    std::env::set_current_dir(&build_directory)
        .map_err(|err| PackageError::failed_to_set_cwd(build_directory.display(), err))?;

    // Unset the Leo panic hook
    let _ = std::panic::take_hook();

    // Call the `run` command.
    println!();
    let command = SnarkVMRun::try_parse_from(&arguments).map_err(CliError::failed_to_parse_run)?;
    let res = command.parse().map_err(CliError::failed_to_execute_run)?;

    // Log the output of the `run` command.
    tracing::info!("{}", res);

    Ok(())
}
