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

//! Enforces a circuit variable assignment statement in a compiled Leo program.

use crate::{errors::StatementError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_core_ast::{Identifier, Span};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{boolean::Boolean, select::CondSelectGadget},
    },
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn mutate_circuit_variable<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        indicator: Option<Boolean>,
        circuit_name: &str,
        variable_name: Identifier,
        mut new_value: ConstrainedValue<F, G>,
        span: &Span,
    ) -> Result<ConstrainedValue<F, G>, StatementError> {
        let condition = indicator.unwrap_or(Boolean::Constant(true));

        // Get the mutable circuit by name
        match self.get_mutable_assignee(circuit_name, span)? {
            ConstrainedValue::CircuitExpression(_variable, members) => {
                // Modify the circuit variable in place
                let matched_variable = members.iter_mut().find(|member| member.0 == variable_name);

                match matched_variable {
                    Some(member) => match &member.1 {
                        ConstrainedValue::Function(_circuit_identifier, function) => {
                            // Throw an error if we try to mutate a circuit function
                            Err(StatementError::immutable_circuit_function(
                                function.identifier.to_string(),
                                span.to_owned(),
                            ))
                        }
                        ConstrainedValue::Static(_circuit_function) => {
                            // Throw an error if we try to mutate a static circuit function
                            Err(StatementError::immutable_circuit_function(
                                "static".into(),
                                span.to_owned(),
                            ))
                        }
                        ConstrainedValue::Mutable(value) => {
                            // Mutate the circuit variable's value in place

                            // Check that the new value type == old value type
                            new_value.resolve_type(Some(value.to_type(span)?), span)?;

                            // Conditionally select the value if this branch is executed.
                            let mut selected_value = ConstrainedValue::conditionally_select(
                                cs.ns(|| format!("select {} {}:{}", new_value, span.line, span.start)),
                                &condition,
                                &new_value,
                                &member.1,
                            )
                            .map_err(|_| {
                                StatementError::select_fail(
                                    new_value.to_string(),
                                    member.1.to_string(),
                                    span.to_owned(),
                                )
                            })?;

                            // Make sure the new value is still mutable
                            selected_value = ConstrainedValue::Mutable(Box::new(selected_value));

                            member.1 = selected_value.to_owned();

                            Ok(selected_value)
                        }
                        _ => {
                            // Throw an error if we try to mutate an immutable circuit variable
                            Err(StatementError::immutable_circuit_variable(
                                variable_name.name,
                                span.to_owned(),
                            ))
                        }
                    },
                    None => {
                        // Throw an error if the circuit variable does not exist in the circuit
                        Err(StatementError::undefined_circuit_variable(
                            variable_name.to_string(),
                            span.to_owned(),
                        ))
                    }
                }
            }
            // Throw an error if the circuit definition does not exist in the file
            _ => Err(StatementError::undefined_circuit(
                variable_name.to_string(),
                span.to_owned(),
            )),
        }
    }
}
