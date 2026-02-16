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

/// Returns true if `name` matches a known devnet artifact directory prefix.
fn is_devnet_artifact(name: &str) -> bool {
    name.starts_with("ledger-")
        || name.starts_with("node-data-")
        || name.starts_with(".logs-")
        || name.starts_with(".ledger-")
        || name.starts_with(".node-data-")
}

/// Discover devnet artifact directories under `storage`.
pub fn find_devnet_artifacts(storage: &Path) -> AnyhowResult<Vec<PathBuf>> {
    let mut dirs = Vec::new();
    for entry in std::fs::read_dir(storage)? {
        let entry = entry?;
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if entry.file_type()?.is_dir() && is_devnet_artifact(&name) {
            dirs.push(entry.path());
        }
    }
    dirs.sort();
    Ok(dirs)
}

/// Clean devnet storage (ledgers, node data, logs).
#[derive(Parser, Debug)]
pub struct LeoDevnetClean {
    #[clap(long, help = "Ledger / log root directory", default_value = "./")]
    pub(crate) storage: String,
    #[clap(short = 'y', long, help = "Skip confirmation prompts")]
    pub(crate) yes: bool,
}

impl Command for LeoDevnetClean {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "LeoDevnetClean")
    }

    fn prelude(&self, _: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, _cx: Context, _: Self::Input) -> Result<Self::Output> {
        self.handle_clean().map_err(|e| CliError::custom(format!("Failed to clean devnet storage: {e}")).into())
    }
}

impl LeoDevnetClean {
    fn handle_clean(&self) -> AnyhowResult<()> {
        let storage = PathBuf::from(&self.storage);
        if !storage.exists() {
            println!("Storage path `{}` does not exist. Nothing to clean.", self.storage);
            return Ok(());
        }
        if !storage.is_dir() {
            bail!("Storage path `{}` is not a directory.", self.storage);
        }

        let dirs_to_remove = find_devnet_artifacts(&storage)?;

        if dirs_to_remove.is_empty() {
            println!("No devnet storage found in `{}`.", self.storage);
            return Ok(());
        }

        println!("Found {} devnet storage directories in `{}`:", dirs_to_remove.len(), self.storage);
        for dir in &dirs_to_remove {
            println!("  • {}", dir.display());
        }

        if !confirm("\nRemove these directories?", self.yes)? {
            println!("Aborted.");
            return Ok(());
        }

        for dir in &dirs_to_remove {
            std::fs::remove_dir_all(dir)?;
            println!("  Removed {}", dir.display());
        }
        println!("Cleaned {} directories.", dirs_to_remove.len());

        Ok(())
    }
}
