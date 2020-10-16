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

use crate::{ast::expressions::array::SpreadOrExpression, Expression, ExpressionError, ExpressionValue};
use leo_symbol_table::{ResolvedNode, SymbolTable, Type};
use leo_typed::{Span, SpreadOrExpression as UnresolvedSpreadOrExpression};

impl Expression {
    /// Resolves an array expression
    pub(crate) fn array(
        table: &mut SymbolTable,
        expected_type: Option<Type>,
        expressions: Vec<Box<UnresolvedSpreadOrExpression>>,
        span: Span,
    ) -> Result<Self, ExpressionError> {
        // Expressions should evaluate to array type or array element type
        let expected_element_type = if let Some(type_) = expected_type {
            let (element_type, dimensions) = type_.get_type_array(span.clone())?;

            if dimensions[0] != expressions.len() {
                //throw array dimension mismatch error
                return Err(ExpressionError::invalid_length_array(
                    dimensions[0],
                    expressions.len(),
                    span.clone(),
                ));
            }

            Some(element_type.clone())
        } else {
            None
        };

        // Store actual array element type
        let mut actual_element_type = None;
        let mut array = vec![];

        // Resolve all array elements
        for expression in expressions {
            let expression_resolved = SpreadOrExpression::resolve(table, (expected_element_type.clone(), *expression))?;
            let expression_type = expression_resolved.type_().clone();

            array.push(Box::new(expression_resolved));
            actual_element_type = Some(expression_type);
        }

        // Define array type for expression
        let type_ = match actual_element_type {
            Some(type_) => type_,
            None => unimplemented!("ERROR: Arrays of size zero are no-op"),
        };

        Ok(Expression {
            type_,
            value: ExpressionValue::Array(array, span),
        })
    }
}
