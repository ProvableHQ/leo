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
use leo_grammar::{
    common::SpreadOrExpression as GrammarSpreadOrExpression,
    expressions::Expression as GrammarExpression,
};

use serde::{Deserialize, Serialize};
use std::fmt;

/// Spread or expression
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpreadOrExpression {
    Spread(Expression),
    Expression(Expression),
}

impl<'ast> From<GrammarSpreadOrExpression<'ast>> for SpreadOrExpression {
    fn from(s_or_e: GrammarSpreadOrExpression<'ast>) -> Self {
        match s_or_e {
            GrammarSpreadOrExpression::Spread(spread) => {
                SpreadOrExpression::Spread(Expression::from(spread.expression))
            }
            GrammarSpreadOrExpression::Expression(expression) => {
                SpreadOrExpression::Expression(Expression::from(expression))
            }
        }
    }
}

impl<'ast> From<GrammarExpression<'ast>> for SpreadOrExpression {
    fn from(expression: GrammarExpression<'ast>) -> Self {
        SpreadOrExpression::Expression(Expression::from(expression))
    }
}

impl fmt::Display for SpreadOrExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SpreadOrExpression::Spread(ref spread) => write!(f, "...{}", spread),
            SpreadOrExpression::Expression(ref expression) => write!(f, "{}", expression),
        }
    }
}

impl Node for SpreadOrExpression {
    fn span(&self) -> &Span {
        use SpreadOrExpression::*;
        match self {
            Spread(expression) | Expression(expression) => expression.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        use SpreadOrExpression::*;
        match self {
            Spread(expression) | Expression(expression) => expression.set_span(span),
        }
    }
}
