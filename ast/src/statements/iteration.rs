// Copyright (C) 2019-2020 Aleo Systems Inc.
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

use crate::{Block, Expression, Identifier, Node, Span};

use leo_grammar::statements::ForStatement;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct IterationStatement {
    pub variable: Identifier,
    pub start: Expression,
    pub stop: Expression,
    pub block: Block,
    pub span: Span,
}

impl fmt::Display for IterationStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "for {} in {}..{} {}",
            self.variable, self.start, self.stop, self.block
        )
    }
}

impl<'ast> From<ForStatement<'ast>> for IterationStatement {
    fn from(statement: ForStatement<'ast>) -> Self {
        IterationStatement {
            variable: Identifier::from(statement.index),
            start: Expression::from(statement.start),
            stop: Expression::from(statement.stop),
            block: Block::from(statement.block),
            span: Span::from(statement.span),
        }
    }
}

impl Node for IterationStatement {
    fn span(&self) -> &Span {
        &self.span
    }

    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}
