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
#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
// On `wasm32` the run / errors helpers below are unreachable; suppress
// the dead-code lints so the wasm build stays clean.
#![cfg_attr(target_arch = "wasm32", allow(dead_code, unused_imports))]
#![doc = include_str!("../README.md")]

mod compiler;
pub use compiler::*;

mod errors;

mod options;
pub use options::*;

// Native-only: package/manifest-driven import-stub loader consumed by the LSP
// (and exposed via `pub use` below). Gated at the mod declaration so the file
// itself can stay free of `#[cfg]` attributes.
#[cfg(not(target_arch = "wasm32"))]
mod import_stubs;
#[cfg(not(target_arch = "wasm32"))]
pub use import_stubs::{
    LoadedImportStubs,
    load_import_stubs_for_package,
    load_import_stubs_for_package_with_file_source,
};

// Re-export types from leo_passes for convenience
pub use leo_passes::{Bytecode, CompiledPrograms};
pub use leo_span::file_source::{DiskFileSource, FileSource, InMemoryFileSource};

// `run` exposes the snarkVM `Process` execution surface; native-only because
// it pulls the snarkVM umbrella (ledger/synthesizer) which doesn't build on
// `wasm32`.
#[cfg(not(target_arch = "wasm32"))]
pub mod run;

#[cfg(test)]
mod test_utils;

#[cfg(test)]
mod test_compiler;

#[cfg(test)]
mod test_execution;
