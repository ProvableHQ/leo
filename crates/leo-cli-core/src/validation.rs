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

//! Native-only program-name validation helpers (snarkVM keyword tables +
//! Aleo identifier rules).
//!
//! Re-exports the implementations from `leo-package`. As with [`crate::network`],
//! `crates/package` still hosts the bodies behind a
//! `#[cfg(not(target_arch = "wasm32"))]` gate; a follow-up PR moves them
//! here and drops the gates.

#![cfg(not(target_arch = "wasm32"))]

pub use leo_package::{is_valid_library_name, is_valid_program_name, reserved_keywords, verify_valid_program};
