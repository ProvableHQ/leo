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

use anyhow::{Context, Result, ensure};
use fs2::FileExt;
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
    let install_root = snarkos_path.parent().context("`--snarkos` must have a parent directory")?;
    fs::create_dir_all(install_root)?; // mkdir -p

    // File-based advisory lock → parallel runs wait or fail fast.
    let lock_file = install_root.join(".leo-devnet.lock");
    let lock = fs::OpenOptions::new().create(true).truncate(true).write(true).open(&lock_file)?;
    lock.try_lock_exclusive().context("another snarkOS install is already in progress")?;

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
    let built_bin = install_root.join("bin").join(if cfg!(windows) { "snarkos.exe" } else { "snarkos" });
    if built_bin != snarkos_path {
        if snarkos_path.exists() {
            fs::remove_file(snarkos_path)?; // overwrite consistently
        }

        let link_ok = cfg!(windows)
            // Hard-links on FAT/FAT32 are unreliable → skip.
            .then(|| false)
            .unwrap_or_else(|| fs::hard_link(&built_bin, snarkos_path).is_ok());

        if !link_ok {
            fs::copy(&built_bin, snarkos_path)
                .with_context(|| format!("failed to copy {} → {}", built_bin.display(), snarkos_path.display()))?;
            #[cfg(unix)]
            fs::set_permissions(snarkos_path, fs::Permissions::from_mode(0o755))?;
        }
    }

    ensure!(snarkos_path.is_file(), "snarkOS binary not produced at {}", snarkos_path.display());
    println!("✅  Installed snarkOS ⇒ {}", snarkos_path.display());
    Ok(snarkos_path.to_path_buf())
}

/// Looks for `snarkos` in the $PATH, or exits with an error message.
pub fn default_snarkos() -> PathBuf {
    which("snarkos").unwrap_or_else(|_| {
        eprintln!(
            "❌  Could not find `snarkos` in your $PATH.  \
             Provide one with --snarkos or use --install."
        );
        std::process::exit(1);
    })
}

/// Installs a signal handler that listens for SIGINT, SIGTERM, SIGQUIT, and SIGHUP.
/// This is only needed on Unix-like systems, as Windows shutdown is handled by the Job Object.
#[cfg(unix)]
pub fn install_signal_handler(manager: Arc<Mutex<ChildManager>>, ready: Arc<AtomicBool>) -> AnyhowResult<()> {
    let mut signals = Signals::new([SIGINT, SIGTERM, SIGQUIT, SIGHUP])?;
    thread::spawn(move || {
        for _sig in signals.forever() {
            if !ready.load(Ordering::SeqCst) {
                // Ignore very early signals (before children).
                continue;
            }
            eprintln!("\n⏹  Signal received – shutting down devnet …");
            manager.lock().unwrap().shutdown_all(Duration::from_secs(30));
            std::process::exit(0);
        }
    });
    Ok(())
}

/// Cleans a ledger associated with a snarkOS node.
pub fn clean_snarkos<S: AsRef<OsStr>>(alias: S, network: usize, idx: usize, _storage: &Path) -> std::io::Result<Child> {
    StdCommand::new(alias)
        .arg("clean")
        .arg("--network")
        .arg(network.to_string())
        .arg("--dev")
        .arg(idx.to_string())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
}
