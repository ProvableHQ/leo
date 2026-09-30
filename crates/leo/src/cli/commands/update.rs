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

use super::*;
use leo_ast::NetworkName;
use leo_package::Package;

/// Update dependencies within the manifest constraints.
#[derive(Debug, Parser)]
pub struct LeoUpdate {
    #[clap(value_name = "NAME", help = "Update only this dependency; otherwise update all dependencies.")]
    name: Option<String>,
    #[clap(long, help = "Show dependency updates without changing leo.lock.")]
    dry_run: bool,
    #[clap(flatten)]
    env_override: EnvOptions,
}

impl Command for LeoUpdate {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, _: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, context: Context, _: Self::Input) -> Result<Self::Output> {
        if context.package_filter.is_some() {
            return Err(crate::errors::custom(
                "Dependency updates use the shared workspace lock. Use `leo update NAME` to select a dependency.",
            )
            .into());
        }
        let network = get_network(&self.env_override.network).unwrap_or_else(|_| {
            tracing::warn!("No network specified, defaulting to 'testnet'.");
            NetworkName::TestnetV0
        });
        let endpoint = get_endpoint(&self.env_override.endpoint).unwrap_or_else(|_| DEFAULT_ENDPOINT.to_string());
        let (old, updated) = Package::update_dependencies(
            &context.dir()?,
            &context.home()?,
            self.name.as_deref(),
            self.dry_run,
            network,
            &endpoint,
            self.env_override.network_retries,
        )?;

        let action = if self.dry_run { "Would update" } else { "Updated" };
        let mut changed = false;
        for pin in updated.network_entries() {
            let previous = old.network_entries().iter().find(|old| old.name == pin.name && old.network == pin.network);
            if previous == Some(pin) {
                continue;
            }
            changed = true;
            let previous = previous.map(|pin| pin.edition.to_string()).unwrap_or_else(|| "unlocked".to_string());
            tracing::info!("{action} {} ({}) edition {previous} -> {}", pin.name, pin.network, pin.edition);
        }
        for pin in updated.git_entries() {
            let previous = old.commit_for(&pin.name, &pin.git, &pin.reference);
            if previous == Some(pin.commit.as_str()) {
                continue;
            }
            changed = true;
            tracing::info!(
                "{action} {} ({}) {:.12} -> {:.12}",
                pin.name,
                pin.reference,
                previous.unwrap_or("unlocked"),
                pin.commit
            );
        }
        if !changed {
            tracing::info!("No dependency updates.");
        }
        if self.dry_run {
            tracing::info!("Dry run: leo.lock was not changed.");
        }
        Ok(())
    }
}
