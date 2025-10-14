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

#![forbid(unsafe_op_in_unsafe_fn)]

mod child_manager;
use child_manager::*;

#[cfg(windows)]
mod windows_kill_tree;

mod shutdown;
use shutdown::*;

mod utilities;
use utilities::*;

use anyhow::{Context as AnyhowContext, Result as AnyhowResult, anyhow, bail, ensure};
use chrono::Local;
use clap::Parser;
use dunce::canonicalize;
use itertools::Itertools;
use parking_lot::Mutex;
use std::{
    env,
    ffi::OsStr,
    path::{Path, PathBuf},
    process::{Child, Command as StdCommand, Stdio},
    sync::Arc,
    time::Duration,
};
use tracing::{self, Span};

#[cfg(unix)]
use {libc::setsid, std::os::unix::process::CommandExt};

use super::*;
use leo_ast::NetworkName;

/// A high REST RPS (requests per second) for snarkOS devnets.
const REST_RPS: &str = "999999999";

/// Launch a local devnet (validators + clients) using snarkOS.
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
    #[clap(long, help = "Path to snarkOS binary. If it does not exist, set `--install` to build it at this path.")]
    pub(crate) snarkos: PathBuf,
    #[clap(long, help = "Required features for snarkOS (e.g. `test_network`)", value_delimiter = ',')]
    pub(crate) snarkos_features: Vec<String>,
    #[clap(long, help = "Required version for snarkOS (e.g. `4.1.0`). Defaults to latest version on `crates.io`.")]
    pub(crate) snarkos_version: Option<String>,
    #[clap(long, help = "(Re)install snarkOS at the provided `--snarkos` path with the given `--snarkos-features`")]
    pub(crate) install: bool,
    #[clap(
        long,
        help = "Optional consensus heights to use. The `test_network` feature must be enabled for this to work.",
        value_delimiter = ','
    )]
    pub(crate) consensus_heights: Option<Vec<u32>>,
    #[clap(long, help = "Run nodes in tmux (only available on Unix)")]
    pub(crate) tmux: bool,
    #[clap(long, help = "snarkOS verbosity (0-4)", default_value = "1")]
    pub(crate) verbosity: u8,
    #[clap(long, short = 'y', help = "Skip confirmation prompts and proceed with the devnet startup")]
    pub(crate) yes: bool,
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
        self.handle_apply().map_err(|e| CliError::custom(format!("Failed to start devnet: {e}")).into())
    }
}

impl LeoDevnet {
    /// Handle the actual devnet startup logic.
    fn handle_apply(&self) -> AnyhowResult<()> {
        //───────────────────────────────────────────────────────────────────
        // 0. Guard rails
        //───────────────────────────────────────────────────────────────────
        if cfg!(windows) && self.tmux {
            bail!("tmux mode is not available on Windows – remove `--tmux`.");
        }
        if self.tmux && std::env::var("TMUX").is_ok() {
            bail!("Nested tmux session detected.  Unset $TMUX and retry.");
        }

        // If the devnet heights are provided, ensure the `test_network` feature is enabled, and validate the heights.
        if let Some(ref heights) = self.consensus_heights {
            if !self.snarkos_features.contains(&"test_network".to_string()) {
                bail!("The `test_network` feature must be enabled on snarkOS to use `--consensus-heights`.");
            }
            validate_consensus_heights(heights.as_slice())?;
        }

        // Resolve the snarkOS path to its canonical form.

        if self.install {
            // If installing, make sure we can write to a file at the path.
            if let Some(parent) = self.snarkos.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(parent)
                        .with_context(|| format!("Failed to create directory for binary: {}", parent.display()))?;
                }
            }
            std::fs::write(&self.snarkos, [0u8]).with_context(|| {
                format!("Failed to write to path {} for snarkos installation", self.snarkos.display())
            })?;
        } else {
            // If not installing, ensure the snarkOS binary exists at the provided path.
            if !self.snarkos.exists() {
                bail!(
                    "The snarkOS binary at `{}` does not exist. Please provide a valid path or use `--install`.",
                    self.snarkos.display()
                );
            }
        };
        let snarkos = canonicalize(&self.snarkos)
            .with_context(|| format!("Failed to resolve snarkOS path: {}", self.snarkos.display()))?;

        // Confirm with the user the options they provided.
        println!("🔧  Starting devnet with the following options:");
        println!("  • Network: {}", self.network);
        println!("  • Validators: {}", self.num_validators);
        println!("  • Clients: {}", self.num_clients);
        println!("  • Storage: {}", self.storage);
        if self.install {
            println!("  • Installing snarkOS at: {}", snarkos.display());
            if let Some(ref version) = self.snarkos_version {
                println!("  • version: {version}");
            }
            if !self.snarkos_features.is_empty() {
                println!("  • features: {}", self.snarkos_features.iter().format(","));
            }
        } else {
            println!("  • Using snarkOS binary at: {}", snarkos.display());
        }
        if let Some(heights) = &self.consensus_heights {
            println!("  • Consensus heights: {}", heights.iter().format(","));
        } else {
            println!("  • Consensus heights: default (based on your snarkOS binary)");
        }
        println!("  • Clear storage: {}", if self.clear_storage { "yes" } else { "no" });
        println!("  • Verbosity: {}", self.verbosity);
        println!("  • tmux: {}", if self.tmux { "yes" } else { "no" });

        //───────────────────────────────────────────────────────────────────
        // 1. Child-manager & shutdown listener (no race!)
        //───────────────────────────────────────────────────────────────────
        let manager = Arc::new(Mutex::new(ChildManager::new()));

        // Install the listener to catch any early shutdown signals.
        let (tx_shutdown, rx_shutdown) = crossbeam_channel::bounded::<()>(1);
        let _signal_thread =
            install_shutdown_listener(tx_shutdown.clone()).context("Failed to install shutdown listener")?;

        //───────────────────────────────────────────────────────────────────
        // 2. snarkOS binary  (+ optional build)
        //───────────────────────────────────────────────────────────────────
        let snarkos = if self.install {
            if !confirm("\nProceed with snarkOS installation?", self.yes)? {
                println!("❌ Installation aborted.");
                return Ok(());
            }
            install_snarkos(&snarkos, self.snarkos_version.as_deref(), &self.snarkos_features)?
        } else {
            snarkos
        };

        // Run `snarkOS --version` and confirm with the user that they'd like to proceed.
        let version_output = StdCommand::new(&snarkos)
            .arg("--version")
            .output()
            .context(format!("Failed to run `{}`", snarkos.display()))?;
        if !version_output.status.success() {
            bail!("Failed to run `{}`: {}", snarkos.display(), String::from_utf8_lossy(&version_output.stderr));
        }

        // Print the version output.
        let version_str = String::from_utf8_lossy(&version_output.stdout);
        println!("🔍  Detected: {version_str}");

        // The version string has the following form:
        // "snarkos refs/heads/staging ace765a42551092fbb47799c2651d6b6df30e49a features=[default,snarkos_node_metrics,test_network]"
        // Parse the features and see if it matches the expected features.
        let features_str = version_str
            .trim()
            .split("features=[")
            .nth(1)
            .and_then(|s| s.split(']').next())
            .ok_or_else(|| anyhow!("Failed to parse snarkOS features from version string: {version_str}"))?;
        let found_features: Vec<String> = features_str.split(',').map(|s| s.trim().to_string()).collect();
        for feature in &self.snarkos_features {
            if !found_features.contains(feature) {
                println!("⚠️  Warning: snarkOS does not have the required feature `{feature}` enabled.");
            }
        }

        if !confirm("\nProceed with devnet startup?", self.yes)? {
            println!("❌ Devnet aborted.");
            return Ok(());
        }

        //───────────────────────────────────────────────────────────────────
        // 3. Resolve storage & create log dir
        //───────────────────────────────────────────────────────────────────
        // Create the storage directory if it does not exist.
        let storage = PathBuf::from(&self.storage);
        if !storage.exists() {
            std::fs::create_dir_all(&storage)
                .context(format!("Failed to create storage directory: {}", self.storage))?;
        } else if !storage.is_dir() {
            bail!("The storage path `{}` is not a directory.", self.storage);
        }
        // Resolve the storage directory to its canonical form.
        let storage =
            canonicalize(&storage).with_context(|| format!("Failed to resolve storage path: {}", self.storage))?;
        // Create the log directory inside the storage directory.
        let log_dir = {
            let ts = Local::now().format(".logs-%Y-%m-%d-%H-%M-%S").to_string();
            let p = storage.join(ts);
            std::fs::create_dir_all(&p)?;
            p
        };

        //───────────────────────────────────────────────────────────────────
        // 4. (Optional) ledger cleanup
        //───────────────────────────────────────────────────────────────────
        if self.clear_storage {
            println!("🧹  Cleaning ledgers …");
            let mut cleaners = Vec::new();
            for idx in 0..self.num_validators {
                cleaners.push(clean_snarkos(&snarkos, self.network as usize, "validator", idx, storage.as_path())?);
            }
            for idx in 0..self.num_clients {
                cleaners.push(clean_snarkos(
                    &snarkos,
                    self.network as usize,
                    "client",
                    idx + self.num_validators,
                    storage.as_path(),
                )?);
            }
            for mut c in cleaners {
                c.wait()?;
            }
        }

        //───────────────────────────────────────────────────────────────────
        // 5. Spawn nodes (tmux **or** background)
        //───────────────────────────────────────────────────────────────────

        #[allow(clippy::too_many_arguments)]
        fn build_args(
            role: &str,
            verbosity: u8,
            network: usize,
            num_validators: usize,
            idx: usize,
            log_file: &Path,
            metrics_port: Option<u16>,
        ) -> Vec<String> {
            let mut base = vec![
                "start".to_string(),
                "--nodisplay".to_string(),
                "--network".to_string(),
                network.to_string(),
                "--dev".to_string(),
                idx.to_string(),
                "--dev-num-validators".to_string(),
                num_validators.to_string(),
                "--rest-rps".to_string(),
                REST_RPS.to_string(),
                "--logfile".to_string(),
                log_file.to_str().unwrap().to_string(),
                "--verbosity".to_string(),
                verbosity.to_string(),
            ];
            match role {
                "validator" => {
                    base.extend(
                        ["--allow-external-peers", "--validator", "--no-dev-txs"].into_iter().map(String::from),
                    );
                    if let Some(p) = metrics_port {
                        base.extend(["--metrics".into(), "--metrics-ip".into(), format!("0.0.0.0:{p}")]);
                    }
                }
                "client" => base.push("--client".into()),
                _ => unreachable!(),
            }
            base
        }

        // Set the environment variable for the consensus heights if provided.
        // These are used by all child processes.
        if let Some(ref heights) = self.consensus_heights {
            let heights = heights.iter().join(",");
            println!("🔧  Setting consensus heights: {heights}");
            #[allow(unsafe_code)]
            unsafe {
                // SAFETY:
                //  - `CONSENSUS_VERSION_HEIGHTS` is only set once and is only read in `snarkvm::prelude::load_consensus_heights`.
                //  - There are no concurrent threads running at this point in the execution.
                // WHY:
                //  - This is needed because there is no way to set the desired consensus heights for a particular `VM` instance in a node
                //    without using the environment variable `CONSENSUS_VERSION_HEIGHTS`. Which is itself read once, and stored in a `OnceLock`.
                env::set_var("CONSENSUS_VERSION_HEIGHTS", heights);
            }
        }

        //────────────── tmux branch ──────────────
        if self.tmux {
            // Create session.
            let mut args: Vec<String> =
                vec!["new-session", "-d", "-s", "devnet", "-n", "validator-0"].into_iter().map(Into::into).collect();

            // If a tmux server is already running, the new session will inherit the environment
            // variables of the server. As such, we need to explicitly set the CONSENSUS_VERSION_HEIGHTS
            // env var in the new session we are creatomg.
            if let Some(ref heights) = self.consensus_heights {
                let heights = heights.iter().join(",");
                args.push("-e".to_string());
                args.push(format!("CONSENSUS_VERSION_HEIGHTS={heights}"));
            }

            ensure!(StdCommand::new("tmux").args(args).status()?.success(), "tmux failed to create session");

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
                let cmd = std::iter::once(snarkos.to_string_lossy().into_owned())
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
                let cmd = std::iter::once(snarkos.to_string_lossy().into_owned())
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

        //──────────── background branch ──────────
        println!("⚙️  Spawning nodes as background tasks …");

        // Helper: setsid() on Unix, Job-object attach on Windows.
        let spawn_with_group = |mut cmd: StdCommand, log_file: &Path| -> AnyhowResult<Child> {
            let log_handle = std::fs::OpenOptions::new().create(true).append(true).open(log_file)?;
            cmd.stdout(Stdio::from(log_handle.try_clone()?));
            cmd.stderr(Stdio::from(log_handle));

            #[cfg(unix)]
            #[allow(unsafe_code)]
            unsafe {
                // SAFETY: We are in the child just before exec; setsid() only
                // affects the child and cannot violate Rust invariants.
                cmd.pre_exec(|| {
                    setsid();
                    Ok(())
                });
            }

            let child = cmd.spawn().map_err(|e| anyhow!("spawn {e}"))?;

            #[cfg(windows)]
            windows_kill_tree::attach_to_global_job(child.id())?;

            Ok(child)
        };

        {
            // This should be safe since only the current thread will write to the manager.
            let mut guard = manager.lock();

            // Validators
            for idx in 0..self.num_validators {
                let log_file = log_dir.join(format!("validator-{idx}.log"));
                let child = spawn_with_group(
                    {
                        let mut c = StdCommand::new(&snarkos);
                        c.args(build_args(
                            "validator",
                            self.verbosity,
                            self.network as usize,
                            self.num_validators,
                            idx,
                            &log_file,
                            Some(9000 + idx as u16),
                        ));
                        c
                    },
                    &log_file,
                )?;
                println!("  • validator {idx}  (pid = {})", child.id());
                guard.push(child);
            }

            // Clients
            for idx in 0..self.num_clients {
                let dev_idx = idx + self.num_validators;
                let log_file = log_dir.join(format!("client-{idx}.log"));
                let child = spawn_with_group(
                    {
                        let mut c = StdCommand::new(&snarkos);
                        c.args(build_args(
                            "client",
                            self.verbosity,
                            self.network as usize,
                            self.num_validators,
                            dev_idx,
                            &log_file,
                            None,
                        ));
                        c
                    },
                    &log_file,
                )?;
                println!("  • client    {idx}  (pid = {})", child.id());
                guard.push(child);
            }
        }

        // Print the main process ID.
        println!("📌  Main process ID: {}", std::process::id());
        println!("\nDevnet running – Ctrl+C, SIGTERM, or terminal close to stop.");

        // Block here until the first (coalesced) shutdown request
        let _ = rx_shutdown.recv();
        manager.lock().shutdown_all(Duration::from_secs(30));

        Ok(())
    }
}
