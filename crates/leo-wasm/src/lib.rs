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

#![forbid(unsafe_code)]
#![cfg_attr(not(any(test, target_arch = "wasm32")), allow(dead_code))]

//! WASM bindings for the Leo compiler.
//!
//! # `leo` CLI ↔ wasm entry points
//!
//! Every entry point mirrors one [`leo` CLI command] and takes the same
//! shape: a `{ "<path>": "<contents>" }` virtual file map plus a `root`
//! pointing at the project's `program.json` directory. JS callers stage the
//! same on-disk layout `leo build` would consume — no special "single
//! source" shortcut.
//!
//! | `leo` command | wasm module          | impl                              | JS export  |
//! |---------------|----------------------|-----------------------------------|------------|
//! | `leo build`   | [`commands::build`]  | [`build_impl`][b]                 | `build`    |
//! | `leo run`     | [`commands::run`]    | [`run_impl`][r]   (wasm32)        | `run`      |
//! | `leo test`    | [`commands::test`]   | [`test_impl`][t]  (wasm32)        | `test`     |
//! | `leo fmt`     | [`commands::format`] | [`format_impl`][f] (single source) | `format`  |
//!
//! [b]: commands::build::build_impl
//! [r]: commands::run::run_impl
//! [t]: commands::test::test_impl
//! [f]: commands::format::format_impl
//! [`leo` CLI command]: https://github.com/ProvableHQ/leo/tree/master/crates/leo/src/cli/commands
//!
//! `format_impl` is the one outlier — it mirrors `leo_fmt::format_source`,
//! the same per-source primitive `leo fmt` invokes per file. Every other
//! command is project-shaped.
//!
//! Each `*_impl` returns a JSON string the JS side can parse directly. The
//! `#[wasm_bindgen]` shims in the wasm-only [`wasm_bindings`] module are
//! one-line wrappers around those `*_impl`s.
//!
//! # Internals
//!
//! - [`project`] turns a `{path: contents}` virtual file map into a loaded
//!   [`leo_package::Package`] (manifest parsing + transitive dep resolution)
//!   plus the [`leo_span::file_source::InMemoryFileSource`] the Compiler
//!   runs against.
//! - [`wire`] is the JSON plumbing shared across commands ([`wire::EnvOptions`],
//!   `error_json`, `import_summaries`).
//!
//! `leo run` and `leo test` reuse [`leo_compiler::run::run_without_ledger`]
//! for execution — the same in-memory `Process` + `FinalizeStore` path the
//! native test framework uses for non-async cases — so no wasm-only execution
//! glue lives in this crate.
//!
//! Build with:
//!   `wasm-pack build crates/leo-wasm --target web --out-dir ../../leo-playground/wasm`

pub mod commands;
pub mod project;
pub mod wire;

// ---------------------------------------------------------------------------
// WASM bindings
// ---------------------------------------------------------------------------
//
// One-liner `#[wasm_bindgen]` wrappers around each `commands::*_impl`. Gated
// to `wasm32` so a native workspace build doesn't pull `wasm-bindgen` (and
// the snarkVM subset crates) into its dependency graph.

#[cfg(target_arch = "wasm32")]
mod wasm_bindings {
    use wasm_bindgen::prelude::*;

    /// Install the panic hook so Rust panics surface as JS errors.
    #[wasm_bindgen(start)]
    pub fn init() {
        console_error_panic_hook::set_once();
    }

    /// `leo build` — compile a project. `env_json` is the JSON shape of
    /// `crate::wire::EnvOptions` (network, endpoint, …); pass `""` to default.
    #[wasm_bindgen]
    pub fn build(files_json: &str, root: &str, env_json: &str) -> String {
        crate::commands::build::build_impl(files_json, root, env_json)
    }

    /// `leo run` — compile and execute one function. `env_json` mirrors the
    /// CLI's `--network` / `--private-key` / `--endpoint` flags.
    #[wasm_bindgen]
    pub fn run(files_json: &str, root: &str, function_name: &str, inputs_json: &str, env_json: &str) -> String {
        crate::commands::run::run_impl(files_json, root, function_name, inputs_json, env_json)
    }

    /// `leo test` — compile project + test package, run every `@test` fn.
    /// `env_json` mirrors the CLI's env flags.
    #[wasm_bindgen]
    pub fn test(files_json: &str, root: &str, test_root: &str, env_json: &str) -> String {
        crate::commands::test::test_impl(files_json, root, test_root, env_json)
    }

    /// `leo fmt` — format a Leo source string. Mirrors `leo_fmt::format_source`,
    /// the per-file primitive the CLI invokes.
    #[wasm_bindgen]
    pub fn format(source: &str) -> String {
        crate::commands::format::format_impl(source)
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm_bindings::{build, format, init, run, test};
