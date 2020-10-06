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

//! Enforces a circuit static access expression in a compiled Leo program.

use crate::{errors::ExpressionError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_typed::{CircuitMember, Expression, Identifier, Span, Type};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn enforce_circuit_static_access<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        expected_type: Option<Type>,
        circuit_identifier: Box<Expression>,
        circuit_member: Identifier,
        span: Span,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        // Get defined circuit
        let circuit = match *circuit_identifier {
            Expression::Identifier(identifier) => {
                // Use the "Self" keyword to access a static circuit function
                if identifier.is_self() {
                    let circuit = self
                        .get(&file_scope)
                        .ok_or(ExpressionError::self_keyword(identifier.span))?;

                    circuit.to_owned()
                } else {
                    self.evaluate_identifier(file_scope, function_scope, expected_type, identifier)?
                }
            }
            expression => self.enforce_expression(
                cs,
                file_scope,
                function_scope,
                expected_type,
                expression,
            )?,
        }
        .extract_circuit(span.clone())?;

        // Find static circuit function
        let matched_function = circuit.members.into_iter().find(|member| match member {
            CircuitMember::CircuitFunction(_static, function) => function.identifier == circuit_member,
            _ => false,
        });

        // Return errors if no static function exists
        let function = match matched_function {
            Some(CircuitMember::CircuitFunction(_static, function)) => {
                if _static {
                    function
                } else {
                    return Err(ExpressionError::invalid_member_access(
                        function.identifier.to_string(),
                        span,
                    ));
                }
            }
            _ => {
                return Err(ExpressionError::undefined_member_access(
                    circuit.circuit_name.to_string(),
                    circuit_member.to_string(),
                    span,
                ));
            }
        };

        Ok(ConstrainedValue::Function(Some(circuit.circuit_name), function))
    }
}
