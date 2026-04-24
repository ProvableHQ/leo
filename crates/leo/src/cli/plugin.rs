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

use leo_errors::{CliError, Result};

use colored::Colorize;
use std::{
    convert::Infallible,
    ffi::OsString,
    path::{Path, PathBuf},
    process,
};

const PLUGIN_PREFIX: &str = "leo-";

/// Scan `PATH` for an executable named `name`.
pub fn find_exe(name: &str) -> Option<PathBuf> {
    let filename = format!("{name}{}", std::env::consts::EXE_SUFFIX);
    let var = std::env::var_os("PATH")?;
    std::env::split_paths(&var).find_map(|dir| {
        let candidate = dir.join(&filename);
        if is_executable(&candidate) { Some(candidate) } else { None }
    })
}

/// Find and execute a plugin binary, forwarding `args` and optionally setting
/// the working directory to `cwd`.
///
/// On Unix this replaces the current process via `exec`. On other platforms it
/// spawns the plugin and propagates the exit code.
pub fn exec(name: &str, args: &[OsString], cwd: Option<&Path>) -> Result<Infallible> {
    let path = find_exe(name).ok_or_else(|| -> leo_errors::LeoError {
        CliError::custom(format!("'{name}' not found. Install the plugin and ensure it is available on your PATH."))
            .into()
    })?;

    let mut cmd = process::Command::new(&path);
    cmd.args(args);
    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }

    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        // Does not return on success.
        let err = cmd.exec();
        Err(CliError::custom(format!("failed to exec '{name}': {err}")).into())
    }

    #[cfg(not(unix))]
    {
        let status = cmd
            .stdin(process::Stdio::inherit())
            .stdout(process::Stdio::inherit())
            .stderr(process::Stdio::inherit())
            .status()
            .map_err(|err| -> leo_errors::LeoError {
                CliError::custom(format!("failed to spawn '{name}': {err}")).into()
            })?;
        process::exit(status.code().unwrap_or(1));
    }
}

/// Iterate over all `leo-*` plugin executables found on `PATH`.
///
/// Yields `(subcommand_name, path)` pairs, e.g. `("fmt", "/usr/local/bin/leo-fmt")`.
/// Each plugin appears at most once (first match on PATH wins).
pub fn all() -> Vec<(String, PathBuf)> {
    let Some(var) = std::env::var_os("PATH") else {
        return Vec::new();
    };
    let mut seen = std::collections::HashSet::new();
    let mut plugins = Vec::new();
    for dir in std::env::split_paths(&var) {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            let stem = name.strip_suffix(std::env::consts::EXE_SUFFIX).unwrap_or(name);
            if let Some(subcmd) = stem.strip_prefix(PLUGIN_PREFIX)
                && !subcmd.is_empty()
                && is_executable(&path)
                && seen.insert(subcmd.to_string())
            {
                plugins.push((subcmd.to_string(), path));
            }
        }
    }
    plugins.sort_by(|(a, _), (b, _)| a.cmp(b));
    plugins
}

/// Print all installed `leo-*` plugins to stdout.
pub fn print_all() {
    let plugins = all();
    if plugins.is_empty() {
        println!("No leo plugins detected on PATH.");
        return;
    }
    println!("Installed plugins:\n");
    let name_col_w = plugins.iter().map(|(cmd, _)| cmd.len()).max().unwrap_or(0);
    for (cmd, path) in &plugins {
        println!("  {:<name_col_w$}  {}", cmd.bold(), path.display());
    }
}

#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    path.is_file() && std::fs::metadata(path).map(|m| m.permissions().mode() & 0o111 != 0).unwrap_or(false)
}

#[cfg(not(unix))]
fn is_executable(path: &Path) -> bool {
    path.is_file()
}
