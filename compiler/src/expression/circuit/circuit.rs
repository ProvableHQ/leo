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

//! Enforces a circuit expression in a compiled Leo program.

use crate::{
    errors::ExpressionError,
    program::{new_scope, ConstrainedProgram},
    value::{ConstrainedCircuitMember, ConstrainedValue},
    GroupType,
};
use leo_ast::{CircuitMember, CircuitVariableDefinition, Identifier, Span};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn enforce_circuit<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: &str,
        function_scope: &str,
        identifier: Identifier,
        members: Vec<CircuitVariableDefinition>,
        span: Span,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        // Circuit definitions are located at the minimum file scope
        let minimum_scope = file_scope.split('_').next().unwrap();
        let identifier_string = identifier.to_string();
        let mut program_identifier = new_scope(minimum_scope, &identifier_string);

        if identifier.is_self() {
            program_identifier = file_scope.to_string();
        }

        let circuit = match self.get(&program_identifier) {
            Some(value) => value.clone().extract_circuit(&span)?,
            None => return Err(ExpressionError::undefined_circuit(identifier.to_string(), span)),
        };

        let circuit_identifier = circuit.circuit_name.clone();
        let mut resolved_members = Vec::with_capacity(circuit.members.len());

        for member in circuit.members.into_iter() {
            match member {
                CircuitMember::CircuitVariable(identifier, type_) => {
                    let matched_variable = members
                        .clone()
                        .into_iter()
                        .find(|variable| variable.identifier.eq(&identifier));
                    match matched_variable {
                        Some(variable) => {
                            // Resolve and enforce circuit variable
                            let variable_value = self.enforce_expression(
                                cs,
                                file_scope,
                                function_scope,
                                Some(type_.clone()),
                                variable.expression,
                            )?;

                            resolved_members.push(ConstrainedCircuitMember(identifier, variable_value))
                        }
                        None => return Err(ExpressionError::expected_circuit_member(identifier.to_string(), span)),
                    }
                }
                CircuitMember::CircuitFunction(function) => {
                    let identifier = function.identifier.clone();
                    let constrained_function_value =
                        ConstrainedValue::Function(Some(circuit_identifier.clone()), Box::new(function));

                    resolved_members.push(ConstrainedCircuitMember(identifier, constrained_function_value));
                }
            };
        }

        Ok(ConstrainedValue::CircuitExpression(
            circuit_identifier,
            resolved_members,
        ))
    }
}
