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

use crate::{Expression, ExpressionError, Frame, ResolvedNode};
use leo_static_check::Type;
use leo_typed::RangeOrExpression as UnresolvedRangeOrExpression;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RangeOrExpression {
    Range(Expression, Expression),
    Expression(Expression),
}

impl RangeOrExpression {
    // /// If this is a range, return an array type.
    // /// If this is an expression, return a data type.
    // pub(crate) fn type_(&self) -> &Type {
    //     match self {
    //         RangeOrExpression::Range(expresion, _expression) => expresion.type_(),
    //         RangeOrExpression::Expression(expression) => expression.type_(),
    //     }
    // }

    ///
    /// Returns a new `RangeOrExpression` from a given `UnresolvedRangeOrExpression`.
    ///
    /// Performs a lookup in the given function body's variable table if the expression contains
    /// user-defined variables.
    ///
    pub fn new(
        frame: &Frame,
        type_: &Type,
        unresolved_expression: UnresolvedRangeOrExpression,
    ) -> Result<Self, ExpressionError> {
        Ok(match unresolved_expression {
            UnresolvedRangeOrExpression::Range(from, to) => {
                let resolved_from = Expression::new(frame, type_, from.unwrap())?;
                let resolved_to = Expression::new(frame, type_, to.unwrap())?;
                // TODO: add check for range type and array type
                RangeOrExpression::Range(resolved_from, resolved_to)
            }
            UnresolvedRangeOrExpression::Expression(expression) => {
                let expression_resolved = Expression::new(frame, type_, expression)?;
                // TODO: add check for array type
                RangeOrExpression::Expression(expression_resolved)
            }
        })
    }
}
