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

use std::fmt::Display;

use leo_errors::Backtraced;

const CODE_PREFIX: &str = "DIS";
const CODE_MASK: i32 = 13000;

pub(crate) fn snarkvm_parsing_error(name: impl Display) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK,
        format!("failed to parse the source file for `{name}.aleo` into a valid Aleo program"),
    )
    .with_help(format!("Verify that `{name}.aleo` is valid Aleo bytecode. If it was produced by Leo, rebuild the dependency from source."))
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn snarkvm_validation_error(name: impl Display, reason: impl Display) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 1,
        format!("dependency `{name}.aleo` failed snarkVM validation: {reason}"),
    )
    .with_help(
        "This usually means the dependency is hand-crafted Aleo bytecode (or produced by a non-Leo toolchain) that uses a feature snarkVM accepts syntactically but rejects semantically. Regenerate the dependency from Leo source if possible.",
    )
}
