// Copyright (C) 2019-2024 Aleo Systems Inc.
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

use crate::{ Expression, Node, NodeID};
use crate::cst::{ Block, Statement, };
use leo_span::Span;
use serde::{Deserialize, Serialize};
use std::fmt;

/// An `if condition block (else next)?` statement.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ConditionalStatement {
    /// The `bool`-typed condition deciding what to evaluate.
    pub condition: Expression,
    /// The block to evaluate in case `condition` yields `true`.
    pub then: Block,
    /// The statement, if any, to evaluate when `condition` yields `false`.
    pub otherwise: Option<Box<Statement>>,
    /// The span from `if` to `next` or to `block`.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl fmt::Display for ConditionalStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "if ({}) {}", self.condition, self.then)?;
        match self.otherwise.as_ref() {
            Some(n_or_e) => write!(f, " else {n_or_e}"),
            None => write!(f, ""),
        }
    }
}

crate::simple_node_impl!(ConditionalStatement);
