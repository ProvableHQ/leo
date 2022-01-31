// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use crate::{CircuitMember, Identifier};

use serde::{Deserialize, Serialize};
use std::fmt;

/// A record type definition, e.g., `circuit Foo { my_field: Bar }`.
/// In some languages these are called `struct`s.
///
/// Type identity is decided by the full path including `circuit_name`,
/// as the record is nominal, not structural.
/// The fields are named so `circuit Foo(u8, u16)` is not allowed.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Circuit {
    /// The name of the type in the type system in this module.
    pub circuit_name: Identifier,
    /// The fields, constant variables, and functions of this structure.
    pub members: Vec<CircuitMember>,
}

impl Circuit {
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "circuit {} {{ ", self.circuit_name)?;
        for field in self.members.iter() {
            writeln!(f, "    {}", field)?;
        }
        write!(f, "}}")
    }
}

impl fmt::Debug for Circuit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl fmt::Display for Circuit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}
