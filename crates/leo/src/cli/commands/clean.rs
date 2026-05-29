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

use anyhow::anyhow;

use super::*;

/// Clean outputs folder command
#[derive(Parser, Debug)]
pub struct LeoClean {}

impl Command for LeoClean {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, _: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, context: Context, _: Self::Input) -> Result<Self::Output> {
        match context.resolve_targets()? {
            Some(targets) => {
                for target in &targets {
                    let member_name = target.file_name().and_then(|n| n.to_str()).unwrap_or("?");
                    if targets.len() > 1 {
                        println!("\n--- workspace member '{member_name}' ---");
                    }
                    clean_package(context.with_path(target.clone()))?;
                }
                Ok(())
            }
            None => clean_package(context),
        }
    }
}

fn clean_package(context: Context) -> Result<()> {
    let path = context.dir()?;

    let manifest_path = path.join(leo_package::MANIFEST_FILENAME);

    if !manifest_path.exists() {
        return Err(anyhow!(
            "{} doesn't exist - this doesn't appear to be a Leo package.",
            leo_package::MANIFEST_FILENAME
        )
        .into());
    }

    // Best-effort legacy cleanup: pre-flat-layout builds created a top-level
    // `outputs/` directory for AST snapshots. Snapshots now live under
    // `build/<unit>/snapshots/`, so this entry only fires when wiping an
    // upgraded checkout that still has the stale directory on disk.
    let legacy_outputs = path.join("outputs");
    if std::fs::remove_dir_all(&legacy_outputs).is_ok() {
        tracing::info!("🧹 Cleaned the legacy outputs directory {}", legacy_outputs.display().to_string().dimmed());
    }

    // Removes the build/ directory.
    let build_path = path.join(leo_package::BUILD_DIRECTORY);
    if std::fs::remove_dir_all(&build_path).is_ok() {
        tracing::info!("🧹 Cleaned the build directory {}", build_path.display().to_string().dimmed());
    }

    Ok(())
}
