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

//! One module per `leo` CLI command, mirroring `crates/leo/src/cli/commands/*.rs`.
//!
//! Each module exposes target-neutral `*_impl` entry points returning a JSON
//! string. The thin `#[wasm_bindgen]` shims in [`crate::wasm_bindings`]
//! forward verbatim, and the leo CLI's equivalent commands will (Phase 2)
//! call into the same target-neutral logic in `leo-compiler::commands`.

pub mod build;
pub mod format;
#[cfg(target_arch = "wasm32")]
pub mod run;
#[cfg(target_arch = "wasm32")]
pub mod test;
