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
//! Each `commands::*` module mirrors one [`leo` CLI command]. Pick the entry
//! point matching the CLI command you'd run:
//!
//! | `leo` command  | wasm module                | impls                                                       |
//! |----------------|----------------------------|-------------------------------------------------------------|
//! | `leo build`    | [`commands::build`]        | [`compile_impl`][b1] (single source) · [`compile_project_impl`][b2] (project) |
//! | `leo run`      | [`commands::run`] (wasm32) | [`run_impl`][r1] (single source) · [`run_project_impl`][r2] (project) |
//! | `leo test`     | [`commands::test`] (wasm32)| [`run_tests_impl`][t1] (main + tests src) · [`test_project_impl`][t2] (project) |
//! | `leo fmt`      | [`commands::format`]       | [`format_impl`][f1]                                         |
//!
//! [b1]: commands::build::compile_impl
//! [b2]: commands::build::compile_project_impl
//! [r1]: commands::run::run_impl
//! [r2]: commands::run::run_project_impl
//! [t1]: commands::test::run_tests_impl
//! [t2]: commands::test::test_project_impl
//! [f1]: commands::format::format_impl
//! [`leo` CLI command]: https://github.com/ProvableHQ/leo/tree/master/crates/leo/src/cli/commands
//!
//! Every `*_impl` returns a JSON string the JS side can parse directly. The
//! `#[wasm_bindgen]` shims in the wasm-only [`wasm_bindings`] module are
//! one-line wrappers around those `*_impl`s.
//!
//! # Internals
//!
//! - [`project`] turns a `{path: contents}` virtual file map into a loaded
//!   `Project` (manifest parsing + transitive dep resolution).
//! - [`evaluate`] is the wasm32-only snarkVM execution glue
//!   (`Process::load_web` + `FinalizeMemory`).
//! - [`wire`] is the JSON/manifest plumbing shared across commands
//!   (`parse_program_json`, `error_json`, `import_summaries`, …).
//!
//! Build with:
//!   `wasm-pack build crates/leo-wasm --target web --out-dir ../../leo-playground/wasm`

pub mod commands;
pub mod project;
pub mod wire;

// snarkVM execution glue (`Process::load_web`, `FinalizeMemory`, …) — only
// builds under the wasm-compatible snarkVM subset crates.
#[cfg(target_arch = "wasm32")]
pub mod evaluate;

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

    // `leo build` ----------------------------------------------------------

    #[wasm_bindgen]
    pub fn compile(source: &str, program_json: &str) -> String {
        crate::commands::build::compile_impl(source, program_json)
    }

    #[wasm_bindgen]
    pub fn compile_project(files_json: &str, root: &str) -> String {
        crate::commands::build::compile_project_impl(files_json, root)
    }

    // `leo run` ------------------------------------------------------------

    #[wasm_bindgen]
    pub fn run(source: &str, function_name: &str, inputs_json: &str, program_json: &str) -> String {
        crate::commands::run::run_impl(source, function_name, inputs_json, program_json)
    }

    #[wasm_bindgen]
    pub fn run_project(files_json: &str, root: &str, function_name: &str, inputs_json: &str) -> String {
        crate::commands::run::run_project_impl(files_json, root, function_name, inputs_json)
    }

    // `leo test` -----------------------------------------------------------

    #[wasm_bindgen]
    pub fn run_tests(main_source: &str, test_source: &str, program_json: &str) -> String {
        crate::commands::test::run_tests_impl(main_source, test_source, program_json)
    }

    #[wasm_bindgen]
    pub fn test_project(files_json: &str, root: &str, test_root: &str) -> String {
        crate::commands::test::test_project_impl(files_json, root, test_root)
    }

    // `leo fmt` ------------------------------------------------------------

    #[wasm_bindgen]
    pub fn format(source: &str) -> String {
        crate::commands::format::format_impl(source)
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm_bindings::{compile, compile_project, format, init, run, run_project, run_tests, test_project};
