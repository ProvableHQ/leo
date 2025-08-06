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

// Copyright (C) 2019-2025 Provable Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::{Context as AnyhowContext, Result as AnyhowResult, anyhow, bail, ensure};
use chrono::Local;
use clap::Parser;
use dunce::canonicalize;
use libc::setsid;
use signal_hook::{consts::signal::*, iterator::Signals};
use std::{
    ffi::OsStr,
    os::unix::prelude::CommandExt,
    path::{Path, PathBuf},
    process::{Child, Command as StdCommand, Stdio},
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};
use tracing::{self, Span};
use which::which;

use super::*;
use leo_ast::NetworkName;

/// Launch a local devnet (validators + clients) for snarkOS.
#[derive(Parser, Debug)]
pub struct LeoDevnet {
    #[clap(long, help = "Number of validators", default_value = "4")]
    pub(crate) num_validators: usize,
    #[clap(long, help = "Number of clients", default_value = "2")]
    pub(crate) num_clients: usize,
    #[clap(short = 'n', long, help = "Network (mainnet=0, testnet=1, canary=2)", default_value = "testnet")]
    pub(crate) network: NetworkName,
    #[clap(long, help = "Ledger / log root directory", default_value = "./")]
    pub(crate) storage: String,
    #[clap(long, help = "Clear existing ledgers before start")]
    pub(crate) clear_storage: bool,
    #[clap(
        long,
        help = "Path to snarkOS binary (defaults to `snarkos` in $PATH)",
        default_value_os_t = default_snarkos()
    )]
    pub(crate) snarkos: PathBuf,
    #[clap(long, help = "Comma-separated extra features passed to `cargo install`", value_delimiter = ',')]
    pub(crate) features: Vec<String>,
    #[clap(long, help = "Build / update snarkOS before launch")]
    pub(crate) install: bool,
    #[clap(long, help = "Run nodes in tmux windows on Unix")]
    pub(crate) tmux: bool,
    #[clap(long, help = "snarkOS verbosity (0-4)", default_value = "1")]
    pub(crate) verbosity: u8,
}

impl Command for LeoDevnet {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "LeoDevnet")
    }

    fn prelude(&self, _: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, _cx: Context, _: Self::Input) -> Result<Self::Output> {
        self.handle_apply().map_err(|e| CliError::custom(format!("Failed start devnet: {e}")).into())
    }
}

impl LeoDevnet {
    /// Handle the actual devnet startup logic.
    fn handle_apply(&self) -> AnyhowResult<()> {
        //───────────────────────────────────────────────────────────────────
        // 0. Guard rails
        //───────────────────────────────────────────────────────────────────
        if cfg!(target_os = "windows") && self.tmux {
            bail!("tmux mode is not available on Windows – remove `--tmux`.");
        }
        if self.tmux && std::env::var("TMUX").is_ok() {
            bail!("Nested tmux session detected.  Unset $TMUX and retry.");
        }

        //───────────────────────────────────────────────────────────────────
        // 1. Optional: build snarkOS
        //───────────────────────────────────────────────────────────────────
        if self.install {
            // Ensure parent/bin directory exists
            let snarkos_dir = self.snarkos.parent().expect("snarkos path must have a parent directory");
            let root_dir = snarkos_dir.parent().unwrap_or(snarkos_dir);
            std::fs::create_dir_all(root_dir)?;

            // Tell Cargo to install into <root_dir>, so binary ends up in <root_dir>/bin/snarkos
            let mut cmd = StdCommand::new("cargo");
            cmd.args(["install", "--locked", "--force", "--path", ".", "--root", root_dir.to_str().unwrap()]);
            if !self.features.is_empty() {
                cmd.arg("--features").arg(self.features.join(","));
            }

            println!("🔧  Building snarkOS into {} … ({cmd:?})", root_dir.display());
            ensure!(cmd.status()?.success(), "`cargo install` failed");

            println!("✅  Installed snarkOS ⇒ {}", self.snarkos.display());
        }

        // Check if snarkos binary exists
        let snarkos_bin = if self.snarkos.is_absolute() {
            self.snarkos.clone()
        } else {
            // Relative path, resolve against current working directory
            std::env::current_dir().context("Failed to get current working directory")?.join(self.snarkos.clone())
        };

        ensure!(snarkos_bin.exists(), "snarkOS binary not found at {}", snarkos_bin.display());

        //───────────────────────────────────────────────────────────────────
        // 2. Resolve storage & create log dir
        //───────────────────────────────────────────────────────────────────
        let storage_dir = canonicalize(&self.storage).with_context(|| format!("Cannot access {}", self.storage))?;
        let log_dir = {
            let ts = Local::now().format(".logs-%Y%m%d%H%M%S").to_string();
            let p = storage_dir.join(ts);
            std::fs::create_dir_all(&p)?;
            p
        };

        //───────────────────────────────────────────────────────────────────
        // 3. (Optional) ledger cleanup
        //───────────────────────────────────────────────────────────────────
        if self.clear_storage {
            println!("🧹  Cleaning ledgers …");
            let mut cleaners = Vec::new();
            for idx in 0..(self.num_validators + self.num_clients) {
                cleaners.push(clean_snarkos(snarkos_bin.clone(), self.network as usize, idx, storage_dir.as_path())?);
            }
            for mut c in cleaners {
                c.wait()?;
            }
        }

        //───────────────────────────────────────────────────────────────────
        // 4. Signal handling & manager setup
        //───────────────────────────────────────────────────────────────────
        let manager = Arc::new(Mutex::new(ChildManager::new()));
        install_signal_handler(manager.clone())?;

        //───────────────────────────────────────────────────────────────────
        // 5. Spawn nodes (tmux OR background mode)
        //───────────────────────────────────────────────────────────────────
        const REST_RPS: &str = "999999999";

        /// Build the arg list common to both spawn paths.
        fn build_args(
            role: &str,
            verbosity: u8,
            network: usize,
            num_validators: usize,
            idx: usize,
            log_file: &Path,
            metrics_port: Option<u16>,
        ) -> Vec<String> {
            let mut args = vec![
                "start",
                "--nodisplay",
                "--network",
                &network.to_string(),
                "--dev",
                &idx.to_string(),
                "--dev-num-validators",
                &num_validators.to_string(),
                "--rest-rps",
                REST_RPS,
                "--logfile",
                log_file.to_str().expect("utf-8 path"),
                "--verbosity",
                &verbosity.to_string(),
            ]
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>();

            match role {
                "validator" => {
                    args.extend(
                        ["--allow-external-peers", "--validator", "--no-dev-txs"].iter().map(|s| s.to_string()),
                    );
                    if let Some(p) = metrics_port {
                        args.extend(["--metrics".to_string(), "--metrics-ip".to_string(), format!("0.0.0.0:{p}")]);
                    }
                }
                "client" => args.push("--client".to_string()),
                _ => unreachable!(),
            }
            args
        }

        //──────────────── tmux branch ────────────────
        if self.tmux {
            // Create session.
            ensure!(
                StdCommand::new("tmux")
                    .args(["new-session", "-d", "-s", "devnet", "-n", "validator-0"])
                    .status()?
                    .success(),
                "tmux failed to create session"
            );

            // Determine base-index.
            let base_index = {
                let out = StdCommand::new("tmux").args(["show-option", "-gv", "base-index"]).output()?;
                String::from_utf8_lossy(&out.stdout).trim().parse::<usize>().unwrap_or(0)
            };

            // Validators
            for idx in 0..self.num_validators {
                let win_idx = idx + base_index;
                let window_name = format!("validator-{idx}");
                if idx != 0 {
                    StdCommand::new("tmux")
                        .args(["new-window", "-t", &format!("devnet:{win_idx}"), "-n", &window_name])
                        .status()?;
                }
                let log_file = log_dir.join(format!("{window_name}.log"));
                let metrics_port = 9000 + idx as u16;
                let cmd = std::iter::once(snarkos_bin.to_string_lossy().into_owned())
                    .chain(build_args(
                        "validator",
                        self.verbosity,
                        self.network as usize,
                        self.num_validators,
                        idx,
                        log_file.as_path(),
                        Some(metrics_port),
                    ))
                    .collect::<Vec<_>>()
                    .join(" ");
                StdCommand::new("tmux")
                    .args(["send-keys", "-t", &format!("devnet:{win_idx}"), &cmd, "C-m"])
                    .status()?;
            }

            // Clients
            for idx in 0..self.num_clients {
                let dev_idx = idx + self.num_validators;
                let win_idx = dev_idx + base_index;
                let window_name = format!("client-{idx}");
                StdCommand::new("tmux")
                    .args(["new-window", "-t", &format!("devnet:{win_idx}"), "-n", &window_name])
                    .status()?;
                let log_file = log_dir.join(format!("{window_name}.log"));
                let cmd = std::iter::once(snarkos_bin.to_string_lossy().into_owned())
                    .chain(build_args(
                        "client",
                        self.verbosity,
                        self.network as usize,
                        self.num_validators,
                        dev_idx,
                        log_file.as_path(),
                        None,
                    ))
                    .collect::<Vec<_>>()
                    .join(" ");
                StdCommand::new("tmux")
                    .args(["send-keys", "-t", &format!("devnet:{win_idx}"), &cmd, "C-m"])
                    .status()?;
            }

            println!("✅  tmux session \"devnet\" is ready – attaching …");
            StdCommand::new("tmux").args(["attach-session", "-t", "devnet"]).status()?;
            return Ok(()); // tmux will hold the terminal
        }

        //──────────────── background branch ──────────
        println!("⚙️  Spawning nodes as background tasks …");

        // Helper that attaches proper pre-exec on Unix (setsid).
        let spawn_with_group = |mut cmd: StdCommand| -> Result<Child> {
            #[cfg(unix)]
            #[allow(unsafe_code)]
            unsafe {
                cmd.pre_exec(|| {
                    // TODO (@d0cd) Validate this approach
                    setsid();
                    Ok(())
                });
            }
            cmd.stdout(Stdio::null()).stderr(Stdio::null());
            Ok(cmd.spawn().map_err(|e| anyhow!("spawn {e}"))?)
        };

        let mut guard = manager.lock().unwrap();

        // Validators
        for idx in 0..self.num_validators {
            let log_file = log_dir.join(format!("validator-{idx}.log"));
            let child = spawn_with_group({
                let mut command = StdCommand::new(&snarkos_bin);
                command.args(build_args(
                    "validator",
                    self.verbosity,
                    self.network as usize,
                    self.num_validators,
                    idx,
                    log_file.as_path(),
                    Some(9000 + idx as u16),
                ));
                command
            })?;
            println!("  • validator {idx}  (pid = {})", child.id());
            guard.push(child);
        }

        // Clients
        for idx in 0..self.num_clients {
            let dev_idx = idx + self.num_validators;
            let log_file = log_dir.join(format!("client-{idx}.log"));
            let child = spawn_with_group({
                let mut command = StdCommand::new(snarkos_bin.clone());
                command.args(build_args(
                    "client",
                    self.verbosity,
                    self.network as usize,
                    self.num_validators,
                    dev_idx,
                    log_file.as_path(),
                    None,
                ));
                command
            })?;
            println!("  • client    {idx}  (pid = {})", child.id());
            guard.push(child);
        }
        drop(guard); // release mutex

        println!("\nDevnet running – Ctrl+C or SIGTERM to stop.");

        // Block forever – ChildManager::Drop + signal handler cover shutdown.
        thread::park();
        unreachable!()
    }
}

//──────────────────────────────────────────────────────────────────────────────
//  Child-process manager  (RAII guard + graceful shutdown)
//──────────────────────────────────────────────────────────────────────────────

/// Owns all spawned snarkOS processes and ensures they are terminated.
///
/// *  On **Unix** we send `SIGTERM` to the *process group* (negative PID)
///    so that any snarkOS helper processes are included.
/// *  On **Windows** we fall back to `child.kill()` for each PID.
struct ChildManager {
    children: Vec<Child>,
}

impl ChildManager {
    fn new() -> Self {
        Self { children: Vec::new() }
    }

    fn push(&mut self, child: Child) {
        self.children.push(child);
    }

    /// Politely ask each child to exit, wait `timeout`, then hard-kill leftovers.
    fn shutdown_all(&mut self, timeout: Duration) {
        for child in &mut self.children {
            #[cfg(unix)]
            #[allow(unsafe_code)]
            unsafe {
                // negative PID → process-group
                libc::kill(-(child.id() as i32), libc::SIGTERM);
            }
            #[cfg(windows)]
            {
                let _ = child.kill(); // still “polite” on Windows
            }
        }

        let start = Instant::now();
        while start.elapsed() < timeout {
            if self.children.iter_mut().all(|c| matches!(c.try_wait(), Ok(Some(_)))) {
                return; // everyone exited
            }
            thread::sleep(Duration::from_millis(200));
        }

        // escalate
        for child in &mut self.children {
            let _ = child.kill();
        }
    }
}

impl Drop for ChildManager {
    fn drop(&mut self) {
        // Best-effort cleanup even during unwinding.
        self.shutdown_all(Duration::from_secs(5));
    }
}

//──────────────────────────────────────────────────────────────────────────────
//  Helpers
//──────────────────────────────────────────────────────────────────────────────

/// Locate `snarkos` in $PATH or exit with helpful message.
fn default_snarkos() -> PathBuf {
    which("snarkos").unwrap_or_else(|_| {
        eprintln!(
            "❌  Could not find `snarkos` in your $PATH.  \
             Provide one with --snarkos or use --install."
        );
        std::process::exit(1);
    })
}

/// Install a signal handler thread that forwards termination events
/// to the `ChildManager`.
fn install_signal_handler(manager: Arc<Mutex<ChildManager>>) -> AnyhowResult<()> {
    let mut signals = Signals::new([SIGINT, SIGTERM, SIGQUIT, SIGHUP])?;
    thread::spawn(move || {
        if signals.forever().next().is_some() {
            eprintln!("\n⏹  Signal received – shutting down devnet …");
            manager.lock().unwrap().shutdown_all(Duration::from_secs(10));
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
