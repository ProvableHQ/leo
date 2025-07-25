// Copyright (C) 2019-2025 Provable Inc.
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

/// This is the required snarkVM constructor bytecode for a program that is only upgradable
/// if the new program's checksum matches the one declared in a pre-determined mapping.
pub fn snarkvm_checksum_constructor(mapping: impl std::fmt::Display, key: impl std::fmt::Display) -> String {
    format!(
        r"
constructor:
    branch.eq edition 0u16 to end;
    get {mapping}[{key}] into r0;
    assert.eq checksum r0;
    position end;
"
    )
}
