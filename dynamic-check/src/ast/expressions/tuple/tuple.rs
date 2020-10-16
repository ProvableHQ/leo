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

use crate::{Expression, ExpressionError, ExpressionValue, FunctionBody, ResolvedNode, VariableTable};
use leo_static_check::{SymbolTable, Type};
use leo_typed::{Expression as UnresolvedExpression, Span};

impl Expression {
    ///
    /// Returns a new expression that defines a tuple (single variable with multiple values).
    ///
    /// Performs a lookup in the given variable table if an `UnresolvedExpression` contains user-defined types.
    ///
    pub(crate) fn tuple(
        function_body: &FunctionBody,
        type_: &Type,
        unresolved_expressions: Vec<UnresolvedExpression>,
        span: Span,
    ) -> Result<Self, ExpressionError> {
        // Get the type of each element in the tuple.
        // If the type is not a tuple type, throw an error.
        let element_types = match type_ {
            Type::Tuple(types) => types,
            type_ => return Err(ExpressionError::invalid_type_tuple(type_, &span)),
        };

        // Create a new vector of `Expression`s from the given vector of `UnresolvedExpression`s.
        let mut tuple = vec![];

        for (unresolved_expression, element_type) in unresolved_expressions.into_iter().zip(element_types) {
            let expression = Expression::new(function_body, element_type, unresolved_expression)?;

            tuple.push(expression);
        }

        Ok(Expression {
            type_: type_.clone(),
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
