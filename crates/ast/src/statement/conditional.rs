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

use crate::{Block, Expression, Indent, Node, NodeID, Statement};
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
        writeln!(f, "if {} {{", self.condition)?;
        for stmt in self.then.statements.iter() {
            writeln!(f, "{}{}", Indent(stmt), stmt.semicolon())?;
        }
        match self.otherwise.as_deref() {
            None => write!(f, "}}")?,
            Some(Statement::Block(block)) => {
                writeln!(f, "}} else {{")?;
                for stmt in block.statements.iter() {
                    writeln!(f, "{}{}", Indent(stmt), stmt.semicolon())?;
                }
                write!(f, "}}")?;
            }
            Some(Statement::Conditional(cond)) => {
                write!(f, "}} else {cond}")?;
            }
            Some(_) => panic!("`otherwise` of a `ConditionalStatement` must be a block or conditional."),
        }
        Ok(())
    }
}

impl From<ConditionalStatement> for Statement {
    fn from(value: ConditionalStatement) -> Self {
        Statement::Conditional(value)
    }
}

crate::simple_node_impl!(ConditionalStatement);
