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

//! Enforces an array expression in a compiled Leo program.

use crate::{
    errors::ExpressionError,
    program::{new_scope, ConstrainedProgram},
    value::ConstrainedValue,
    GroupType,
};
use leo_ast::{ArrayDimensions, Expression, PositiveNumber, Span, SpreadOrExpression, Type};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    /// Enforce array expressions
    pub fn enforce_array<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: &str,
        function_scope: &str,
        mut expected_type: Option<Type>,
        array: Vec<SpreadOrExpression>,
        span: Span,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        let mut expected_dimension = None;

        // Check explicit array type dimension if given
        if let Some(type_) = expected_type {
            match type_ {
                Type::Array(type_, mut dimensions) => {
                    // Remove the first dimension of the array.
                    let first = match dimensions.remove_first() {
                        Some(number) => {
                            // Parse the array dimension into a `usize`.
                            parse_index(&number, &span)?
                        }
                        None => return Err(ExpressionError::unexpected_array(type_.to_string(), span)),
                    };

                    // Update the expected dimension to the first dimension.
                    expected_dimension = Some(first);

                    // Update the expected type to a new array type with the first dimension removed.
                    expected_type = Some(inner_array_type(*type_, dimensions));
                }
                ref type_ => {
                    // Return an error if the expected type is not an array.
                    return Err(ExpressionError::unexpected_array(type_.to_string(), span));
                }
            }
        }

        let mut result = vec![];
        for element in array.into_iter() {
            match element {
                SpreadOrExpression::Spread(spread) => match spread {
                    Expression::Identifier(identifier) => {
                        let array_name = new_scope(&function_scope, &identifier.name);
                        match self.get(&array_name) {
                            Some(value) => match value {
                                ConstrainedValue::Array(array) => result.extend(array.clone()),
                                value => return Err(ExpressionError::invalid_spread(value.to_string(), span)),
                            },
                            None => return Err(ExpressionError::undefined_array(identifier.name, span)),
                        }
                    }
                    value => return Err(ExpressionError::invalid_spread(value.to_string(), span)),
                },
                SpreadOrExpression::Expression(expression) => {
                    result.push(self.enforce_expression(
                        cs,
                        file_scope,
                        function_scope,
                        expected_type.clone(),
                        expression,
                    )?);
                }
            }
        }

        // Check expected_dimension if given.
        if let Some(dimension) = expected_dimension {
            // Return an error if the expected dimension != the actual dimension.
            if dimension != result.len() {
                return Err(ExpressionError::invalid_length(dimension, result.len(), span));
            }
        }

        Ok(ConstrainedValue::Array(result))
    }

    /// Enforce array expressions
    pub fn enforce_array_initializer<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: &str,
        function_scope: &str,
        mut expected_type: Option<Type>,
        element_expression: Expression,
        mut actual_dimensions: ArrayDimensions,
        span: Span,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        let mut expected_dimensions = None;

        // Check explicit array type dimension if given
        if let Some(type_) = expected_type {
            match type_ {
                Type::Array(element_type, dimensions) => {
                    // Update the expected type to the element type.
                    expected_type = Some(*element_type);

                    // Update the expected dimensions to the first dimension.
                    expected_dimensions = Some(dimensions);
                }
                ref type_ => {
                    // Return an error if the expected type is not an array.
                    return Err(ExpressionError::unexpected_array(type_.to_string(), span));
                }
            }
        }

        // Evaluate array element expression.
        let mut value = self.enforce_expression(cs, file_scope, function_scope, expected_type, element_expression)?;

        // Check array dimensions.
        if let Some(dimensions) = expected_dimensions {
            if dimensions.ne(&actual_dimensions) {
                return Err(ExpressionError::invalid_dimensions(
                    &dimensions,
                    &actual_dimensions,
                    span,
                ));
            }
        }

        // Allocate the array.
        while let Some(dimension) = actual_dimensions.remove_first() {
            // Parse the dimension into a `usize`.
            let dimension_usize = parse_index(&dimension, &span)?;

            // Allocate the array dimension.
            let array = vec![value; dimension_usize];

            // Set the array value.
            value = ConstrainedValue::Array(array);
        }

        Ok(value)
    }
}

///
/// Returns the index as a usize.
///
pub fn parse_index(number: &PositiveNumber, span: &Span) -> Result<usize, ExpressionError> {
    number
        .value
        .parse::<usize>()
        .map_err(|_| ExpressionError::invalid_index(number.value.to_owned(), span))
}

///
/// Returns the type of the inner array given an array element and array dimensions.
///
/// If the array has no dimensions, then an inner array does not exist. Simply return the given
/// element type.
///
/// If the array has dimensions, then an inner array exists. Create a new type for the
/// inner array. The element type of the new array should be the same as the old array. The
/// dimensions of the new array should be the old array dimensions with the first dimension removed.
///
pub fn inner_array_type(element_type: Type, dimensions: ArrayDimensions) -> Type {
    if dimensions.is_empty() {
        // The array has one dimension.
        element_type
    } else {
        // The array has multiple dimensions.
        Type::Array(Box::new(element_type), dimensions)
    }
}
