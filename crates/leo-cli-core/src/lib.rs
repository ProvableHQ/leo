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

//! Wasm-buildable command core for the Leo toolchain.
//!
//! This crate is the single home for the data structures and
//! command-core logic both [`crates/leo`] (the CLI binary) and
//! [`crates/leo-wasm`] (the wasm-bindgen layer) consume. Each surface
//! becomes a thin wrapper:
//!
//! * The CLI's `LeoBuild::handle_build`/`LeoRun::handle_run`/
//!   `LeoTest::handle_test` lift their core out of the `clap`-driven
//!   structs and call into [`project`] for parsing, dep-graph walking,
//!   compilation, and `@test` discovery.
//! * The `leo-wasm` `commands::*_impl` entry points do the same — they
//!   parse [`options::EnvOptions`] from a JSON blob and call into
//!   [`project`] for the actual work.
//!
//! Everything reachable through this crate compiles cleanly on
//! `wasm32-unknown-unknown`. Anything that genuinely needs the native
//! toolbelt (snarkVM's umbrella, HTTP, terminal UI, disk writes) stays
//! in `crates/leo`.
//!
//! [`crates/leo`]: https://github.com/ProvableHQ/leo/tree/master/crates/leo
//! [`crates/leo-wasm`]: https://github.com/ProvableHQ/leo/tree/master/crates/leo-wasm

#![forbid(unsafe_code)]

pub mod options;
pub mod project;

// Native-only modules — disk + HTTP + snarkVM-umbrella helpers lifted out
// of `crates/leo-package` so that crate stays purely wasm-buildable. Each
// module is `#[cfg(not(target_arch = "wasm32"))]`-gated internally.
pub mod network;
pub mod validation;
