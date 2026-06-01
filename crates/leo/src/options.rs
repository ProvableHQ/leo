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

//! Wasm-buildable command options shared between the native CLI and the
//! `leo-wasm` bindings.
//!
//! The structs live here (rather than in `cli/commands/common/options.rs`)
//! so they remain available when the rest of the `cli` module is gated out
//! on `wasm32-unknown-unknown`. The CLI re-exports them via
//! [`crate::cli::commands::common::options`] for backward compatibility.
//!
//! Every field is `pub` so the wasm side can populate the same struct shape
//! the CLI parses from `clap` flags.

use clap::Parser;
use leo_ast::NetworkName;
use serde::Deserialize;

/// Default network endpoint used by the CLI when neither `--endpoint` nor the
/// `ENDPOINT` environment variable is set.
pub const DEFAULT_ENDPOINT: &str = "https://api.explorer.provable.com/v1";

/// Compiler options wrapper for the `build` command. Also used by other
/// commands which require build output as their input.
#[derive(Parser, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct BuildOptions {
    #[clap(long, help = "Enables offline mode.")]
    pub offline: bool,
    #[clap(long, help = "Enable spans in AST snapshots.")]
    pub enable_ast_spans: bool,
    #[clap(long, help = "Enables dead code elimination in the compiler.", default_value = "true")]
    pub enable_dce: bool,
    #[clap(long, help = "Max depth to type check nested conditionals.", default_value = "10")]
    pub conditional_block_max_depth: usize,
    #[clap(long, help = "Disable type checking of nested conditional branches in finalize scope.")]
    pub disable_conditional_branch_type_checking: bool,
    #[clap(long, help = "Write an AST snapshot immediately after parsing.")]
    pub enable_initial_ast_snapshot: bool,
    #[clap(long, help = "Writes all AST snapshots for the different compiler phases.")]
    pub enable_all_ast_snapshots: bool,
    #[clap(long, help = "Comma separated list of passes whose AST snapshots to capture.", value_delimiter = ',', num_args = 1..)]
    pub ast_snapshots: Vec<String>,
    #[clap(long, help = "Build tests along with the main program and dependencies.")]
    pub build_tests: bool,
    #[clap(long, help = "Don't use the dependency cache.")]
    pub no_cache: bool,
    #[clap(long, help = "Don't use the local source code.")]
    pub no_local: bool,
}

impl Default for BuildOptions {
    fn default() -> Self {
        Self {
            offline: false,
            enable_ast_spans: false,
            enable_dce: true,
            conditional_block_max_depth: 10,
            disable_conditional_branch_type_checking: false,
            enable_initial_ast_snapshot: false,
            enable_all_ast_snapshots: false,
            ast_snapshots: Vec::new(),
            build_tests: false,
            no_cache: false,
            no_local: false,
        }
    }
}

/// Per-invocation environment overrides — what the CLI reads from
/// `--network`/`--endpoint`/`--private-key`/`.env`, and what the wasm side
/// deserializes from the JSON blob each entry point receives.
#[derive(Parser, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct EnvOptions {
    #[clap(
        long,
        help = "The private key to use for the deployment. Overrides the `PRIVATE_KEY` environment variable in your shell or `.env` file. We recommend using `APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH` for local devnets. This key should NEVER be used in production.",
        global = true
    )]
    pub private_key: Option<String>,
    #[clap(
        long,
        help = "The network type to use. e.g `mainnet`, `testnet, and `canary`. Overrides the `NETWORK` environment variable in your shell or `.env` file.",
        global = true
    )]
    pub network: Option<NetworkName>,
    #[clap(
        long,
        help = "The endpoint to deploy to. Overrides the `ENDPOINT` environment variable. We recommend using `https://api.explorer.provable.com/v1` for live networks and `http://localhost:3030` for local devnets.",
        global = true
    )]
    pub endpoint: Option<String>,
    #[clap(
        long,
        help = "Whether the network is a devnet. If not set, defaults to the `DEVNET` environment variable in your shell.",
        global = true
    )]
    pub devnet: bool,
    #[clap(
        long,
        help = "Optional consensus heights to use. This should only be set if you are using a custom devnet.",
        value_delimiter = ',',
        global = true
    )]
    pub consensus_heights: Option<Vec<u32>>,
    #[clap(
        long,
        env = "NETWORK_RETRIES",
        help = "Number of times to retry a failed network request before giving up.",
        default_value = "2"
    )]
    pub network_retries: u32,
}

impl Default for EnvOptions {
    fn default() -> Self {
        Self {
            private_key: None,
            network: None,
            endpoint: None,
            devnet: false,
            consensus_heights: None,
            network_retries: 2,
        }
    }
}

impl EnvOptions {
    /// Resolved network, defaulting to `TestnetV0` when nothing was supplied
    /// (mirrors the CLI's implicit default).
    pub fn resolved_network(&self) -> NetworkName {
        self.network.unwrap_or(NetworkName::TestnetV0)
    }

    /// Parse from the JSON blob a wasm caller passes. An empty / whitespace
    /// blob yields `Self::default()` so callers can pass `""` when they have
    /// no overrides.
    pub fn from_json(env_json: &str) -> Result<Self, String> {
        if env_json.trim().is_empty() {
            return Ok(Self::default());
        }
        serde_json::from_str(env_json).map_err(|e| format!("invalid env JSON: {e}"))
    }
}
