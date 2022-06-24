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

use crate::{Identifier, Node, RecordVariable};
use leo_span::{Span, Symbol};

use serde::{Deserialize, Serialize};
use std::fmt;

/// A record type definition.
/// `record Token { owner: address, balance: u64, balance: u64 }`.
/// Records are constructed similar to `circuit` types but with variables specific to Aleo.
#[derive(Clone, Serialize, Deserialize)]
pub struct Record {
    /// The name of the type in the type system in this module.
    pub identifier: Identifier,
    /// The owner of the program record.
    pub owner: RecordVariable,
    /// The balance of the program record.
    pub balance: RecordVariable,
    /// The program data,
    pub data: Vec<RecordVariable>,
    /// The entire span of the circuit definition.
    pub span: Span,
}

impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier
    }
}

impl Eq for Record {}

impl Record {
    /// Returns the record name as a Symbol.
    pub fn name(&self) -> Symbol {
        self.identifier.name
    }

    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "record {} {{ ", self.identifier)?;
        writeln!(f, "    {}", self.owner)?;
        writeln!(f, "    {}", self.balance)?;
        for var in self.data.iter() {
            writeln!(f, "    {}", var)?;
        }
        write!(f, "}}")
    }
}

impl fmt::Debug for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

crate::simple_node_impl!(Record);
