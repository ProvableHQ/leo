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

//! Build-time option structs shared between the CLI and the wasm bindings.
//!
//! `BuildOptions` mirrors the CLI's `--option` flag surface, deriving both
//! `clap::Parser` (for the CLI) and `serde::Deserialize` (for the wasm JSON
//! shim). The CLI re-exports this via `crates/leo/src/cli/commands/common/options.rs`.

use clap::Parser;
use leo_ast::NetworkName;
use leo_compiler::{AstSnapshots, CompilerOptions};
use serde::Deserialize;

/// Compiler options wrapper for the `build` command.
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

impl From<BuildOptions> for CompilerOptions {
    fn from(options: BuildOptions) -> Self {
        Self {
            ast_spans_enabled: options.enable_ast_spans,
            ast_snapshots: if options.enable_all_ast_snapshots {
                AstSnapshots::All
            } else {
                AstSnapshots::Some(options.ast_snapshots.into_iter().collect())
            },
            initial_ast: options.enable_all_ast_snapshots | options.enable_initial_ast_snapshot,
        }
    }
}

/// Per-build env overrides — what the CLI reads from `--network`/`--endpoint`,
/// and what the wasm side deserializes from a JSON blob. Build only needs
/// `network`; the others travel along so callers can reuse one struct shape
/// without forcing a separate "build env" parser.
#[derive(Parser, Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct EnvOptions {
    #[clap(long, help = "Network name (`mainnet`, `testnet`, `canary`).", global = true)]
    pub network: Option<NetworkName>,
    #[clap(long, help = "Endpoint URL.", global = true)]
    pub endpoint: Option<String>,
    #[clap(
        long,
        env = "NETWORK_RETRIES",
        help = "Number of times to retry a failed network request.",
        default_value = "2"
    )]
    pub network_retries: u32,
}

impl EnvOptions {
    /// Network value defaulted to `TestnetV0` if unset (mirrors the CLI's
    /// implicit default).
    pub fn resolved_network(&self) -> NetworkName {
        self.network.unwrap_or(NetworkName::TestnetV0)
    }

    /// Parse from the JSON blob a wasm caller passes; empty / whitespace
    /// yields the default.
    pub fn from_json(env_json: &str) -> Result<Self, String> {
        if env_json.trim().is_empty() {
            return Ok(Self::default());
        }
        serde_json::from_str(env_json).map_err(|e| format!("invalid env JSON: {e}"))
    }
}
