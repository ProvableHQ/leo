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

use crate::{Block, Expression, Node, Span, Statement};
use leo_grammar::statements::{ConditionalNestedOrEndStatement, ConditionalStatement as GrammarConditionalStatement};

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ConditionalStatement {
    pub condition: Expression,
    pub block: Block,
    pub next: Option<Box<Statement>>,
    pub span: Span,
}

impl<'ast> From<GrammarConditionalStatement<'ast>> for ConditionalStatement {
    fn from(statement: GrammarConditionalStatement<'ast>) -> Self {
        ConditionalStatement {
            condition: Expression::from(statement.condition),
            block: Block::from(statement.block),
            next: statement
                .next
                .map(|nested_statement| match nested_statement {
                    ConditionalNestedOrEndStatement::Nested(conditional_statement) => {
                        Statement::Conditional(ConditionalStatement::from(*conditional_statement))
                    }
                    ConditionalNestedOrEndStatement::End(block) => Statement::Block(Block::from(block)),
                })
                .map(Box::new),
            span: Span::from(statement.span),
        }
    }
}

impl fmt::Display for ConditionalStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "if ({}) {}", self.condition, self.block)?;
        match self.next.clone() {
            Some(n_or_e) => write!(f, " {}", n_or_e),
            None => write!(f, ""),
        }
    }
}

impl Node for ConditionalStatement {
    fn span(&self) -> &Span {
        &self.span
    }

    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}
