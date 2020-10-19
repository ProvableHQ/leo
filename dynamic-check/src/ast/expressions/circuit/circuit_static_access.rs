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
// use leo_static_check::{Attribute, SymbolTable, Type};
// use leo_typed::{Expression as UnresolvedExpression, Identifier, Span};
//
// impl Expression {
//     /// Resolve the type of a static circuit member
//     pub(crate) fn circuit_static_access(
//         table: &mut SymbolTable,
//         expected_type: Option<Type>,
//         circuit: Box<UnresolvedExpression>,
//         member: Identifier,
//         span: Span,
//     ) -> Result<Self, ExpressionError> {
//         // Lookup the circuit in the symbol table.
//         // We do not know the exact circuit type from this context so `expected_type = None`.
//         let circuit_resolved = Expression::resolve(table, (None, *circuit))?;
//         let circuit_name = circuit_resolved.type_().get_type_circuit(span.clone())?;
//
//         // Lookup the circuit type in the symbol table
//         let circuit_type = table
//             .get_circuit(&circuit_name.name)
//             .ok_or(ExpressionError::undefined_circuit(circuit_name.clone()))?;
//
//         // Resolve the circuit member as a circuit function
//         let matched_function = circuit_type
//             .functions
//             .iter()
//             .find(|function| function.function.identifier.eq(&member));
//
//         let type_ = match matched_function {
//             Some(function) => {
//                 // Check static method
//                 if function.attributes.contains(&Attribute::Static) {
//                     function.function.output.type_.clone()
//                 } else {
//                     return Err(ExpressionError::invalid_member_access(member.name, span));
//                 }
//             }
//             None => return Err(ExpressionError::undefined_circuit_function_static(member, span)),
//         };
//
//         // Check type of static circuit function output
//         Type::check_type(&expected_type, &type_, span.clone())?;
//
//         Ok(Expression {
//             type_,
//             value: ExpressionValue::CircuitStaticFunctionAccess(Box::new(circuit_resolved), member, span),
//         })
//     }
// }
