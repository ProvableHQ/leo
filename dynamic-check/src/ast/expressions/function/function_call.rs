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
//
// use crate::{Expression, ExpressionError, ExpressionValue, ResolvedNode};
// use leo_static_check::{SymbolTable, Type};
// use leo_typed::{Expression as UnresolvedExpression, Span};
//
// impl Expression {
//     /// Resolves an inline function call
//     /// Checks for errors in function name, inputs, and output.
//     pub(crate) fn function_call(
//         table: &mut SymbolTable,
//         expected_type: Option<Type>,
//         function: Box<UnresolvedExpression>,
//         inputs: Vec<UnresolvedExpression>,
//         span: Span,
//     ) -> Result<Self, ExpressionError> {
//         // Lookup function in symbol table.
//         // We do not know the exact function type from this context so `expected_type = None`.
//         let function_resolved = Expression::resolve(table, (None, *function))?;
//         let function_name = function_resolved.type_().get_type_function(span.clone())?;
//
//         // Lookup the function type in the symbol table
//         let function_type = table
//             .get_function(&function_name.name)
//             .ok_or(ExpressionError::undefined_function(function_name.clone()))?;
//
//         let type_ = function_type.output.type_.clone();
//         let expected_inputs = function_type.inputs.clone();
//
//         // Check the number of inputs given
//         if inputs.len() != expected_inputs.len() {
//             return Err(ExpressionError::invalid_length_function_inputs(
//                 expected_inputs.len(),
//                 inputs.len(),
//                 span,
//             ));
//         }
//
//         // Check the type for each function input
//         let mut inputs_resolved = vec![];
//
//         for (input, function_input_type) in inputs.into_iter().zip(expected_inputs) {
//             let input_type = function_input_type.type_().clone();
//             let input_resolved = Expression::resolve(table, (Some(input_type), input))?;
//
//             inputs_resolved.push(input_resolved)
//         }
//
//         // Check the function output type
//         Type::check_type(&expected_type, &type_, span.clone())?;
//
//         Ok(Expression {
//             type_,
//             value: ExpressionValue::FunctionCall(Box::new(function_resolved), inputs_resolved, span),
//         })
//     }
// }
