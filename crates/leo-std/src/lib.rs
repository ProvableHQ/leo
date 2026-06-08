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

//! Embedded source for the Leo standard library.
//!
//! Compiler and tooling crates depend on this to inject `std` into every
//! Leo build without requiring users to declare it in `program.json`.

/// The Leo identifier under which the standard library is exposed
/// (`std::foo(...)`).
pub const LIBRARY_NAME: &str = "std";

const LIB_LEO: &str = include_str!("leo/lib.leo");
const DUMMY_LEO: &str = include_str!("leo/dummy.leo");

/// Entry source of the standard library (contents of `lib.leo`).
pub fn entry_source() -> &'static str {
    LIB_LEO
}

/// Submodule sources, returned as `(virtual_path, source)` pairs.
///
/// The format matches what `leo_compiler::Compiler::build_library` expects:
/// the first element of each tuple is a label used in span/error reporting,
/// and the second is the Leo source for that submodule.
pub fn modules() -> &'static [(&'static str, &'static str)] {
    &[("dummy.leo", DUMMY_LEO)]
}

/// The Leo identifier under which the standard library is exposed.
pub fn library_name() -> &'static str {
    LIBRARY_NAME
}
