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
use snarkos_cli::commands::{Deploy as SnarkOSDeploy, Developer};
use snarkvm::cli::helpers::dotenv_private_key;
use std::path::PathBuf;

/// Deploys an Aleo program.
#[derive(Parser, Debug)]
pub struct Deploy {
    #[clap(flatten)]
    pub(crate) fee_options: FeeOptions,
    #[clap(long, help = "Disables building of the project before deployment.", default_value = "false")]
    pub(crate) no_build: bool,
    #[clap(long, help = "Enables recursive deployment of dependencies.", default_value = "false")]
    pub(crate) recursive: bool,
    #[clap(
        long,
        help = "Time in seconds to wait between consecutive deployments. This is to help prevent a program from trying to be included in an earlier block than its dependency program.",
        default_value = "12"
    )]
    pub(crate) wait: u64,
    #[clap(flatten)]
    pub(crate) compiler_options: BuildOptions,
}

impl Command for Deploy {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, context: Context) -> Result<Self::Input> {
        if !self.no_build {
            (Build { options: self.compiler_options.clone() }).execute(context)?;
        }
        Ok(())
    }

    fn apply(self, context: Context, _: Self::Input) -> Result<Self::Output> {
        // Get the program name.
        let project_name = context.open_manifest()?.program_id().to_string();

        // Get the private key.
        let mut private_key = self.fee_options.private_key;
        if private_key.is_none() {
            private_key =
                Some(dotenv_private_key().map_err(CliError::failed_to_read_environment_private_key)?.to_string());
        }

        let mut all_paths: Vec<(String, PathBuf)> = Vec::new();

        // Extract post-ordered list of local dependencies' paths from `leo.lock`.
        if self.recursive {
            // Cannot combine with private fee.
            if self.fee_options.record.is_some() {
                return Err(CliError::recursive_deploy_with_record().into());
            }
            all_paths = context.local_dependency_paths()?;
        }

        // Add the parent program to be deployed last.
        all_paths.push((project_name, context.dir()?.join("build")));

        for (index, (name, path)) in all_paths.iter().enumerate() {
            // Set the deploy arguments.
            let mut deploy_args = vec![
                "snarkos".to_string(),
                "--private-key".to_string(),
                private_key.as_ref().unwrap().clone(),
                "--query".to_string(),
                self.compiler_options.endpoint.clone(),
                "--priority-fee".to_string(),
                self.fee_options.priority_fee.to_string(),
                "--path".to_string(),
                path.to_str().unwrap().parse().unwrap(),
                "--broadcast".to_string(),
                format!("{}/{}/transaction/broadcast", self.compiler_options.endpoint, self.fee_options.network)
                    .to_string(),
                name.clone(),
            ];

            // Use record as payment option if it is provided.
            if let Some(record) = self.fee_options.record.clone() {
                deploy_args.push("--record".to_string());
                deploy_args.push(record);
            };

            let deploy = SnarkOSDeploy::try_parse_from(deploy_args).unwrap();

            // Deploy program.
            Developer::Deploy(deploy).parse().map_err(CliError::failed_to_execute_deploy)?;

            // Sleep for `wait_gap` seconds.
            // This helps avoid parents from being serialized before children.
            if index < all_paths.len() - 1 {
                std::thread::sleep(std::time::Duration::from_secs(self.wait));
            }
        }

        Ok(())
    }
}
