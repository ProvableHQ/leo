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
            Some((root, targets)) => {
                // Workspace mode: build/ lives at the workspace root and is
                // shared across all members, so remove it once. Then loop
                // members to clear pre-shared-layout `<member>/build/` and
                // pre-flat-layout `<member>/outputs/` leftovers.
                remove_dir_if_present(&root.join(leo_package::BUILD_DIRECTORY), "build directory");
                for target in &targets {
                    let member_name = target.file_name().and_then(|n| n.to_str()).unwrap_or("?");
                    if targets.len() > 1 {
                        println!("\n--- workspace member '{member_name}' ---");
                    }
                    clean_member_legacy(target);
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
    remove_dir_if_present(&path.join("outputs"), "legacy outputs directory");

    // Removes the build/ directory.
    remove_dir_if_present(&path.join(leo_package::BUILD_DIRECTORY), "build directory");

    Ok(())
}

/// Per-member cleanup in workspace mode. The shared
/// `<workspace_root>/build/` is removed once by the caller. Best-effort:
/// `resolve_targets` already validated each member, so this only sweeps
/// migration leftovers and never fails the whole `leo clean`.
fn clean_member_legacy(member_dir: &std::path::Path) {
    // Migration aid: a per-member `build/` from a pre-shared-layout checkout.
    remove_dir_if_present(&member_dir.join(leo_package::BUILD_DIRECTORY), "legacy per-member build directory");
    // Migration aid: a per-member `outputs/` from a pre-flat-layout checkout.
    remove_dir_if_present(&member_dir.join("outputs"), "legacy outputs directory");
}

/// Best-effort remove `path` if it exists; log on success.
fn remove_dir_if_present(path: &std::path::Path, label: &str) {
    if std::fs::remove_dir_all(path).is_ok() {
        tracing::info!("🧹 Cleaned the {label} {}", path.display().to_string().dimmed());
    }
}
