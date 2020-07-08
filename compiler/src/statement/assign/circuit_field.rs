//! Enforces a circuit field assignment statement in a compiled Leo program.

use crate::{errors::StatementError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_types::{Identifier, Span};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{boolean::Boolean, select::CondSelectGadget},
    },
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn mutute_circuit_field<CS: ConstraintSystem<F>>(
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
                // Modify the circuit value.field in place
                let matched_field = members.into_iter().find(|object| object.0 == object_name);

                match matched_field {
                    Some(object) => match &object.1 {
                        ConstrainedValue::Function(_circuit_identifier, function) => {
                            return Err(StatementError::immutable_circuit_function(
                                function.function_name.to_string(),
                                span,
                            ));
                        }
                        ConstrainedValue::Static(_value) => {
                            return Err(StatementError::immutable_circuit_function("static".into(), span));
                        }
                        _ => {
                            new_value.resolve_type(&vec![object.1.to_type(span.clone())?], span.clone())?;

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
