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
// use crate::{CircuitVariableDefinition, Expression, ExpressionError, ExpressionValue, ResolvedNode};
// use leo_static_check::{SymbolTable, Type};
// use leo_typed::{CircuitVariableDefinition as UnresolvedCircuitVariableDefinition, Identifier, Span};
//
// impl Expression {
//     ///
//     /// Resolves an inline circuit expression.
//     ///
//     pub(crate) fn circuit(
//         table: &mut SymbolTable,
//         expected_type: Option<Type>,
//         identifier: Identifier,
//         variables: Vec<UnresolvedCircuitVariableDefinition>,
//         span: Span,
//     ) -> Result<Self, ExpressionError> {
//         // Check expected type
//         let type_ = Type::Circuit(identifier.clone());
//         Type::check_type(&expected_type, &type_, span.clone())?;
//
//         // Lookup circuit in symbol table
//         let circuit = table
//             .get_circuit(&identifier.name)
//             .ok_or(ExpressionError::undefined_circuit(identifier.clone()))?;
//
//         // Check the number of variables given
//         let expected_variables = circuit.variables.clone();
//
//         if variables.len() != expected_variables.len() {
//             return Err(ExpressionError::invalid_length_circuit_members(
//                 expected_variables.len(),
//                 variables.len(),
//                 span,
//             ));
//         }
//
//         // Check the name and type for each circuit variable
//         let mut variables_resolved = vec![];
//
//         for variable in variables {
//             // Find variable by name
//             let matched_variable = expected_variables
//                 .iter()
//                 .find(|expected| expected.identifier.eq(&variable.identifier));
//
//             let variable_type = match matched_variable {
//                 Some(variable_type) => variable_type,
//                 None => return Err(ExpressionError::undefined_circuit_variable(variable.identifier)),
//             };
//
//             // Resolve the variable expression using the expected variable type
//             let expected_variable_type = Some(variable_type.type_.clone());
//
//             let variable_resolved = CircuitVariableDefinition::resolve(table, (expected_variable_type, variable))?;
//
//             variables_resolved.push(variable_resolved);
//         }
//
//         Ok(Expression {
//             type_,
//             value: ExpressionValue::Circuit(identifier, variables_resolved, span),
//         })
//     }
// }
