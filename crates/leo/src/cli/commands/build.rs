// Copyright (C) 2019-2026 Provable Inc.
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

//! `leo build` clap surface. The real work lives in
//! [`leo_commands::commands::build::handle_build`]; this file collects
//! the CLI flags, resolves the network / endpoint, and forwards.

use super::*;

use leo_ast::NetworkName;
use leo_commands::commands::build::{DiskSink, handle_build};
use leo_package::Package;
use leo_span::file_source::DiskFileSource;

/// Compile and build program command.
#[derive(Parser, Debug)]
pub struct LeoBuild {
    #[clap(flatten)]
    pub(crate) options: BuildOptions,
    #[clap(flatten)]
    pub(crate) env_override: EnvOptions,
}

impl Command for LeoBuild {
    type Input = ();
    type Output = Package;

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, _: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, context: Context, _: Self::Input) -> Result<Self::Output> {
        match context.resolve_targets()? {
            Some(targets) => {
                let mut last_package = None;
                for target in &targets {
                    let member_name = target.file_name().and_then(|n| n.to_str()).unwrap_or("?");
                    if targets.len() > 1 {
                        println!("\n--- workspace member '{member_name}' ---");
                    }
                    let member_ctx = context.with_path(target.clone());
                    last_package = Some(run_build(&self, member_ctx)?);
                }
                last_package.ok_or_else(|| crate::errors::custom("No workspace members found.").into())
            }
            None => run_build(&self, context),
        }
    }
}

fn run_build(command: &LeoBuild, context: Context) -> Result<Package> {
    let package_path = context.dir()?;
    let home_path = context.home()?;

    let network = match get_network(&command.env_override.network) {
        Ok(n) => n,
        Err(_) => {
            println!("⚠️ No network specified, defaulting to 'testnet'.");
            NetworkName::TestnetV0
        }
    };
    let endpoint = match get_endpoint(&command.env_override.endpoint) {
        Ok(e) => e,
        Err(_) => {
            println!("⚠️ No endpoint specified, defaulting to '{}'.", DEFAULT_ENDPOINT);
            DEFAULT_ENDPOINT.to_string()
        }
    };

    handle_build(
        &command.options,
        network,
        &package_path,
        &DiskFileSource,
        &DiskSink,
        Some(&home_path),
        Some(&endpoint),
        command.env_override.network_retries,
    )
}
