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

#![deny(unsafe_code)]
#![allow(clippy::module_inception)]

// `cli` is the native binary's command surface — uses snarkVM's umbrella,
// HTTP clients, on-disk artefact writes, and the rest of the native toolbelt.
// Gated out of wasm so consumers (e.g. `leo-wasm`) can depend on `leo-lang`
// for shared, wasm-safe surfaces (`options`, …) without dragging the whole
// native command tree into the wasm dep graph.
#[cfg(not(target_arch = "wasm32"))]
pub mod cli;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod errors;

/// Wasm-buildable command options (`EnvOptions`, `BuildOptions`, …).
/// Shared between the native CLI (which `clap`-parses them) and `leo-wasm`
/// (which `serde`-parses them from a JSON blob).
pub mod options;
