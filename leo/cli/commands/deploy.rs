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
//use snarkos_cli::commands::{Deploy as SnarkOSDeploy, Developer};

/// Deploys an Aleo program.
#[derive(Parser, Debug)]
pub struct Deploy {
    #[clap(long, help = "Custom priority fee in microcredits", default_value = "1000000")]
    pub(crate) priority_fee: String,
    #[clap(long, help = "Custom query endpoint", default_value = "http://api.explorer.aleo.org/v1")]
    pub(crate) endpoint: String,
    #[clap(long, help = "Custom network", default_value = "testnet3")]
    pub(crate) network: String,
    #[clap(long, help = "Custom private key")]
    pub(crate) private_key: Option<String>,
    #[clap(long, help = "Disables building of the project before deployment", default_value = "false")]
    pub(crate) no_build: bool,
    #[clap(long, help = "Disables recursive deployment of dependencies", default_value = "false")]
    pub(crate) non_recursive: bool,
    #[clap(long, help = "Custom wait gap between consecutive deployments", default_value = "12")]
    pub(crate) wait_gap: u64,
}

impl Command for Deploy {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, context: Context) -> Result<Self::Input> {
        if !self.no_build {
            (Build { options: BuildOptions::default() }).execute(context)?;
        }
        Ok(())
    }

    fn apply(self, _context: Context, _: Self::Input) -> Result<Self::Output> {
        // // Get the program name
        // let project_name = context.open_manifest()?.program_id().to_string();
        //
        // // Get the private key
        // let mut private_key = self.private_key;
        // if private_key.is_none() {
        //     private_key =
        //         Some(dotenv_private_key().map_err(CliError::failed_to_read_environment_private_key)?.to_string());
        // }
        //
        // let mut all_paths: Vec<(String, PathBuf)> = Vec::new();
        //
        // // Extract post-ordered list of local dependencies' paths from `leo.lock`
        // if !self.non_recursive {
        //     all_paths = context.local_dependency_paths()?;
        // }
        //
        // // Add the parent program to be deployed last
        // all_paths.push((project_name, context.dir()?.join("build")));
        //
        // for (index, (name, path)) in all_paths.iter().enumerate() {
        //     // Set deploy arguments
        //     let deploy = SnarkOSDeploy::try_parse_from([
        //         "snarkos",
        //         "--private-key",
        //         private_key.as_ref().unwrap(),
        //         "--query",
        //         self.endpoint.as_str(),
        //         "--priority-fee",
        //         self.priority_fee.as_str(),
        //         "--path",
        //         path.to_str().unwrap(),
        //         "--broadcast",
        //         format!("{}/{}/transaction/broadcast", self.endpoint, self.network).as_str(),
        //         &name,
        //     ])
        //     .unwrap();
        //
        //     // Deploy program
        //     Developer::Deploy(deploy).parse().map_err(CliError::failed_to_execute_deploy)?;
        //
        //     // Sleep for `wait_gap` seconds.
        //     // This helps avoid parents from being serialized before children.
        //     if index < all_paths.len() - 1 {
        //         std::thread::sleep(std::time::Duration::from_secs(self.wait_gap));
        //     }
        // }

        Err(PackageError::unimplemented_command("leo deploy").into())
    }
}
