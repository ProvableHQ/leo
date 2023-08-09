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

use crate::{Block, Expression, Identifier, Node, NodeID, Type, Value};

use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::{cell::RefCell, fmt};

/// A bounded `for` loop statement `for variable in start .. =? stop block`.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct IterationStatement {
    /// The binding / variable to introduce in the body `block`.
    pub variable: Identifier,
    /// The type of the iteration.
    pub type_: Type,
    /// The start of the iteration.
    pub start: Expression,
    /// The concrete value of `start`.
    #[serde(skip)]
    pub start_value: RefCell<Option<Value>>,
    /// The end of the iteration, possibly `inclusive`.
    pub stop: Expression,
    /// The concrete value of `stop`.
    #[serde(skip)]
    pub stop_value: RefCell<Option<Value>>,
    /// Whether `stop` is inclusive or not.
    /// Signified with `=` when parsing.
    pub inclusive: bool,
    /// The block to run on each iteration.
    pub block: Block,
    /// The span from `for` to `block`.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl fmt::Display for IterationStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let eq = if self.inclusive { "=" } else { "" };
        write!(f, "for {} in {}..{eq}{} {}", self.variable, self.start, self.stop, self.block)
    }
}

crate::simple_node_impl!(IterationStatement);
