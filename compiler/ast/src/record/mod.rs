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

pub mod member;
pub use member::*;

use crate::{Identifier, Node};
use leo_span::{Span, Symbol};

use serde::{Deserialize, Serialize};
use std::fmt;

/// A record type definition, e.g., `record Foo { owner: <addr>, gates: 0u64, my_field: Bar }`.
#[derive(Clone, Serialize, Deserialize)]
pub struct Record {
    /// The name of the type in the type system in this module.
    pub identifier: Identifier,
    /// The fields of this record.
    pub members: Vec<RecordMember>,
    /// The entire span of the record definition.
    pub span: Span,
}

impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier
    }
}

impl Eq for Record {}

impl Record {
    /// Returns the struct name as a Symbol.
    pub fn name(&self) -> Symbol {
        self.identifier.name
    }
}

impl fmt::Debug for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "record {} {{ ", self.identifier)?;
        for field in self.members.iter() {
            writeln!(f, "    {field}")?;
        }
        write!(f, "}}")
    }
}

crate::simple_node_impl!(Record);
