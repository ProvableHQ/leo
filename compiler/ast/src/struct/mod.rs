// Copyright (C) 2019-2023 Aleo Systems Inc.
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

pub mod member;
pub use member::*;

use crate::{Identifier, Node, NodeID};
use leo_span::{Span, Symbol};

use serde::{Deserialize, Serialize};
use std::fmt;

/// A struct type definition, e.g., `struct Foo { my_field: Bar }`.
/// In some languages these are called `struct`s.
///
/// Type identity is decided by the full path including `struct_name`,
/// as the record is nominal, not structural.
/// The fields are named so `struct Foo(u8, u16)` is not allowed.
#[derive(Clone, Serialize, Deserialize)]
pub struct Struct {
    /// The name of the type in the type system in this module.
    pub identifier: Identifier,
    /// The fields, constant variables, and functions of this structure.
    pub members: Vec<Member>,
    /// Was this a `record Foo { ... }`?
    /// If so, it wasn't a struct.
    pub is_record: bool,
    /// The entire span of the struct definition.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl PartialEq for Struct {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier
    }
}

impl Eq for Struct {}

impl Struct {
    /// Returns the struct name as a Symbol.
    pub fn name(&self) -> Symbol {
        self.identifier.name
    }
}

impl fmt::Debug for Struct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

impl fmt::Display for Struct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(if self.is_record { "record" } else { "struct" })?;
        writeln!(f, " {} {{ ", self.identifier)?;
        for field in self.members.iter() {
            writeln!(f, "    {field}")?;
        }
        write!(f, "}}")
    }
}

crate::simple_node_impl!(Struct);
