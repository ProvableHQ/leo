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

use crate::{Expression, Node, Span};

use leo_grammar::statements::ExpressionStatement as GrammarExpressionStatement;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ExpressionStatement {
    pub expression: Expression,
    pub span: Span,
}

impl fmt::Display for ExpressionStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{};", self.expression)
    }
}

impl<'ast> From<GrammarExpressionStatement<'ast>> for ExpressionStatement {
    fn from(statement: GrammarExpressionStatement<'ast>) -> Self {
        // why do we have this span-setting logic?
        let span = Span::from(statement.span);
        let mut expression = Expression::from(statement.expression);
        expression.set_span(span.clone());
        ExpressionStatement { expression, span }
    }
}

impl Node for ExpressionStatement {
    fn span(&self) -> &Span {
        &self.span
    }

    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}
