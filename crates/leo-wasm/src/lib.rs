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
// The build module is only reached via `wasm_bindings::build`, which is itself
// cfg-gated to `wasm32`. On native (incl. native tests) nothing inside calls
// it, so silence the dead-code lints there.
#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

//! WASM bindings for the Leo compiler. Single entry — `build` — mirroring
//! `leo build`. Takes a `{ "<path>": "<contents>" }` virtual file map plus
//! a `root` pointing at the main package's `program.json` directory.
//! Returns a JSON blob with the primary bytecode + ABI plus any FromLeo
//! source-dep artifacts.
//!
//! The real work lives in
//! [`leo_commands::commands::build::handle_build`]; this crate just
//! supplies the `InMemoryFileSource` + `MemorySink` and JSON-shapes the
//! artifacts the sink collected.
//!
//! Build with:
//!   `wasm-pack build crates/leo-wasm --target web --out-dir <out>`

mod build;

#[cfg(target_arch = "wasm32")]
mod wasm_bindings {
    use wasm_bindgen::prelude::*;

    /// Install the panic hook so Rust panics surface as JS errors.
    #[wasm_bindgen(start)]
    pub fn init() {
        console_error_panic_hook::set_once();
    }

    /// `leo build` — compile a project. `env_json` is the JSON shape of
    /// `leo_commands::options::EnvOptions` (network, endpoint, ...); pass
    /// `""` to default the network to testnet. `network_deps_json` is a
    /// `{"<name>.aleo": "<bytecode>"}` map of pre-fetched network deps
    /// (pass `""` for projects with no network deps); each entry gets
    /// staged into the virtual file map and the project's manifests are
    /// rewritten so the build sees them as local deps.
    #[wasm_bindgen]
    pub fn build(files_json: &str, root: &str, env_json: &str, network_deps_json: &str) -> String {
        crate::build::build_impl(files_json, root, env_json, network_deps_json)
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm_bindings::{build, init};
