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

use anyhow::{Result, ensure};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

/// Compiles & installs snarkOS and returns the **final binary path**.
///
/// * `snarkos_path` – the exact `--snarkos` value (may not exist yet)  
/// * `version`      – optional `--version` (`"4.1.0"` etc.)  
/// * `features`     – `--features ..` list (may be empty)
pub fn install_snarkos(snarkos_path: &Path, version: Option<&str>, features: &[String]) -> Result<PathBuf> {
    //───────────────── 1. resolve & prepare directories ─────────────────
    // Create a temporary install directory.
    let tempdir = tempfile::tempdir()?;
    let install_root = tempdir.path();

    //───────────────── 2. `cargo install` into <root> ───────────────────
    let mut cmd = Command::new("cargo");
    cmd.args(["install", "--locked", "--force", "snarkos", "--root", install_root.to_str().unwrap()]);
    if let Some(v) = version {
        cmd.arg("--version").arg(v);
    }
    if !features.is_empty() {
        cmd.arg("--features").arg(features.join(","));
    }

    println!("🔧  Building snarkOS into {} …", install_root.display());
    ensure!(cmd.status()?.success(), "`cargo install` failed");

    //───────────────── 3. link / copy to requested path ─────────────────
    // Get the path to the built `snarkos` binary.
    let built_bin = install_root.join("bin").join(if cfg!(windows) { "snarkos.exe" } else { "snarkos" });

    // Remove the existing file if it exists, to ensure we overwrite it.
    if snarkos_path.exists() {
        fs::remove_file(snarkos_path)?; // overwrite consistently
    }

    // Copy the built binary to the requested path, ensuring the parent directory exists.
    fs::create_dir_all(snarkos_path.parent().unwrap())?;
    fs::copy(&built_bin, snarkos_path)?;

    // Set permissions to be executable (if on Unix).
    #[cfg(unix)]
    {
        let mut perms = fs::metadata(snarkos_path)?.permissions();
        perms.set_mode(0o755); // rwxr-xr-x
        fs::set_permissions(snarkos_path, perms)?;
    }

    ensure!(snarkos_path.is_file(), "snarkOS binary not produced at {}", snarkos_path.display());
    println!("✅  Installed snarkOS ⇒ {}", snarkos_path.display());
    Ok(snarkos_path.to_path_buf())
}

/// Cleans a ledger associated with a snarkOS node.
pub fn clean_snarkos<S: AsRef<OsStr>>(
    snarkos: S,
    network: usize,
    _role: &str,
    idx: usize,
    _storage: &Path,
) -> std::io::Result<Child> {
    StdCommand::new(snarkos)
        .arg("clean")
        .arg("--network")
        .arg(network.to_string())
        .arg("--dev")
        .arg(idx.to_string())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
}
