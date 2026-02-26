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

use crate::{Identifier, Mapping, Node, NodeID, StorageVariable, Type, indent_display::Indent};
use leo_span::{Span, Symbol};
use serde::{Deserialize, Serialize};
use std::fmt;

pub use prototypes::{FunctionPrototype, RecordPrototype};

mod prototypes;

/// An interface definition.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Interface {
    /// The interface identifier, e.g., `Foo` in `interface Foo { ... }`.
    pub identifier: Identifier,
    /// The interfaces this interface inherits from (supports multiple inheritance)
    pub parents: Vec<(Span, Type)>,
    /// The entire span of the interface definition.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
    /// A vector of function prototypes.
    pub functions: Vec<(Symbol, FunctionPrototype)>,
    /// A vector of record prototypes.
    pub records: Vec<(Symbol, RecordPrototype)>,
    /// A vector of mapping declarations.
    pub mappings: Vec<Mapping>,
    /// A vector of storage declarations.
    pub storages: Vec<StorageVariable>,
}

impl Interface {
    pub fn name(&self) -> Symbol {
        self.identifier.name
    }
}

impl PartialEq for Interface {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier
    }
}

impl Eq for Interface {}

impl fmt::Debug for Interface {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl fmt::Display for Interface {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "interface {}{} {{",
            self.identifier,
            if self.parents.is_empty() {
                String::new()
            } else {
                format!(" : {}", self.parents.iter().map(|(_, p)| p.to_string()).collect::<Vec<_>>().join(" + "))
            }
        )?;
        for (_, fun_prot) in &self.functions {
            writeln!(f, "{}", Indent(fun_prot))?;
        }
        for (_, rec_prot) in &self.records {
            writeln!(f, "{}", Indent(rec_prot))?;
        }
        write!(f, "}}")
    }
}

crate::simple_node_impl!(Interface);
