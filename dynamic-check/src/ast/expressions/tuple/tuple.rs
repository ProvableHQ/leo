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

use crate::{Expression, ExpressionError, ExpressionValue, ResolvedNode};
use leo_static_check::{SymbolTable, Type};
use leo_typed::{Expression as UnresolvedExpression, Span};

impl Expression {
    /// Resolves a tuple of expressions to the given tuple type
    pub(crate) fn tuple(
        table: &mut SymbolTable,
        expected_type: Option<Type>,
        expressions: Vec<UnresolvedExpression>,
        span: Span,
    ) -> Result<Self, ExpressionError> {
        // If the expected type is given, then it must be a tuple of types
        let expected_element_types = check_tuple_type(expected_type, expressions.len(), span.clone())?;

        // Check length of tuple against expected types
        if expected_element_types.len() != expressions.len() {
            return Err(ExpressionError::invalid_length_tuple(
                expected_element_types.len(),
                expressions.len(),
                span.clone(),
            ));
        }

        // Resolve all tuple elements
        let mut tuple = vec![];

        for (expression, element_type) in expressions.into_iter().zip(expected_element_types) {
            let expression_resolved = Expression::resolve(table, (element_type, expression))?;

            tuple.push(expression_resolved);
        }

        // Define tuple type for expression
        let actual_element_types = tuple
            .iter()
            .map(|expression| expression.type_().clone())
            .collect::<Vec<_>>();

        let type_ = Type::Tuple(actual_element_types);

        Ok(Expression {
            type_,
            value: ExpressionValue::Tuple(tuple, span),
        })
    }
}

/// Return a tuple of types given some expected type tuple. Otherwise return a tuple of `None` types.
pub fn check_tuple_type(
    expected_type: Option<Type>,
    length: usize,
    span: Span,
) -> Result<Vec<Option<Type>>, ExpressionError> {
    Ok(match expected_type {
        Some(type_) => {
            let types = type_.get_type_tuple(span.clone())?;
            types.iter().map(|type_| Some(type_.clone())).collect::<Vec<_>>()
        }
        None => vec![None; length],
    })
}
