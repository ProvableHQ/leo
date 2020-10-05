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

use crate::Expression;
use leo_ast::common::RangeOrExpression as AstRangeOrExpression;

use serde::{Deserialize, Serialize};
use std::fmt;

/// Range or expression enum
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RangeOrExpression {
    Range(Option<Expression>, Option<Expression>),
    Expression(Expression),
}

impl<'ast> From<AstRangeOrExpression<'ast>> for RangeOrExpression {
    fn from(range_or_expression: AstRangeOrExpression<'ast>) -> Self {
        match range_or_expression {
            AstRangeOrExpression::Range(range) => RangeOrExpression::Range(
                range.from.map(|expression| Expression::from(expression)),
                range.to.map(|expression| Expression::from(expression)),
            ),
            AstRangeOrExpression::Expression(expression) => RangeOrExpression::Expression(Expression::from(expression)),
        }
    }
}

impl fmt::Display for RangeOrExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RangeOrExpression::Range(ref from, ref to) => write!(
                f,
                "{}..{}",
                from.as_ref().map(|e| e.to_string()).unwrap_or("".to_string()),
                to.as_ref().map(|e| e.to_string()).unwrap_or("".to_string())
            ),
            RangeOrExpression::Expression(ref e) => write!(f, "{}", e),
        }
    }
}
