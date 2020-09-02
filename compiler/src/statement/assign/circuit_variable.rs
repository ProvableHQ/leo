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
use leo_typed::{Identifier, Span};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{boolean::Boolean, select::CondSelectGadget},
    },
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn mutute_circuit_variable<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        indicator: Option<Boolean>,
        circuit_name: String,
        object_name: Identifier,
        mut new_value: ConstrainedValue<F, G>,
        span: Span,
    ) -> Result<(), StatementError> {
        let condition = indicator.unwrap_or(Boolean::Constant(true));

        match self.get_mutable_assignee(circuit_name, span.clone())? {
            ConstrainedValue::CircuitExpression(_variable, members) => {
                // Modify the circuit variable in place
                let matched_variable = members.into_iter().find(|object| object.0 == object_name);

                match matched_variable {
                    Some(object) => match &object.1 {
                        ConstrainedValue::Function(_circuit_identifier, function) => {
                            return Err(StatementError::immutable_circuit_function(
                                function.identifier.to_string(),
                                span,
                            ));
                        }
                        ConstrainedValue::Static(_value) => {
                            return Err(StatementError::immutable_circuit_function("static".into(), span));
                        }
                        _ => {
                            new_value.resolve_type(Some(object.1.to_type(span.clone())?), span.clone())?;

                            let name_unique = format!("select {} {}:{}", new_value, span.line, span.start);
                            let selected_value = ConstrainedValue::conditionally_select(
                                cs.ns(|| name_unique),
                                &condition,
                                &new_value,
                                &object.1,
                            )
                            .map_err(|_| {
                                StatementError::select_fail(new_value.to_string(), object.1.to_string(), span)
                            })?;

                            object.1 = selected_value.to_owned();
                        }
                    },
                    None => return Err(StatementError::undefined_circuit_object(object_name.to_string(), span)),
                }
            }
            _ => return Err(StatementError::undefined_circuit(object_name.to_string(), span)),
        }

        Ok(())
    }
}
