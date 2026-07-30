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

use crate::{Function, Identifier, Indent, Node, NodeID};

use leo_span::{Span, Symbol};
use serde::{Deserialize, Serialize};
use std::fmt;

/// An `impl` block associating a set of methods with a type.
///
/// Methods are ordinary functions namespaced under the target type: a method `m` in
/// `impl Point { .. }` is registered at the symbol-table location `[Point, m]`, so a static
/// call `Point::m(..)` resolves through the normal path machinery. An instance method takes a
/// `self` first parameter and is called as `value.m(..)`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Impl {
    /// The type this block implements methods for, e.g. `Point` in `impl Point { .. }`.
    pub type_: Identifier,
    /// The methods defined in this block. Multiple `impl` blocks for the same type merge.
    pub functions: Vec<(Symbol, Function)>,
    /// The entire span of the `impl` block.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl Impl {
    /// The name of the type this block implements.
    pub fn type_name(&self) -> Symbol {
        self.type_.name
    }
}

impl fmt::Display for Impl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "impl {} {{", self.type_)?;
        for (_, function) in self.functions.iter() {
            writeln!(f, "{}", Indent(function))?;
        }
        write!(f, "}}")
    }
}

crate::simple_node_impl!(Impl);
