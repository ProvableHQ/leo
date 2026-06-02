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

//! Shared command implementations. Each module hosts the `handle_*` core
//! that both the native CLI (via its `clap`-derived structs) and the
//! `leo-wasm` bindings (via its JSON entry points) call into.

#![cfg(not(target_arch = "wasm32"))]

pub mod build;
pub mod query;
pub mod run;
pub mod test;
pub mod util;

/// Default `edition` value used by the CLI when a local program doesn't
/// declare one in its manifest. Lifted out of
/// `crates/leo/src/cli/commands/common/util.rs` so the shared command
/// implementations can reach it without depending on the CLI binary.
pub const LOCAL_PROGRAM_DEFAULT_EDITION: leo_package::Edition = 1;

/// Threshold (percentage of `MAX_PROGRAM_SIZE`) above which `format_program_size`
/// emits a warning to the user.
const PROGRAM_SIZE_WARNING_THRESHOLD: usize = 90;

/// Format a program-size pair for the user-facing build report.
///
/// Returns `(size_kb, max_kb, warning)` where `warning` is `Some` if size
/// exceeds 90% of `max_size`.
pub fn format_program_size(size: usize, max_size: usize) -> (f64, f64, Option<String>) {
    let size_kb = size as f64 / 1024.0;
    let max_kb = max_size as f64 / 1024.0;
    let percentage = (size as f64 / max_size as f64) * 100.0;

    let warning = if size > max_size * PROGRAM_SIZE_WARNING_THRESHOLD / 100 {
        Some(format!("approaching the size limit ({percentage:.1}% of {max_kb:.2} KB)"))
    } else {
        None
    };

    (size_kb, max_kb, warning)
}
