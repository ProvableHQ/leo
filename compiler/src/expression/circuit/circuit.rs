//! Enforces a circuit expression in a compiled Leo program.

use crate::{
    errors::ExpressionError,
    program::{new_scope, ConstrainedProgram},
    value::{ConstrainedCircuitMember, ConstrainedValue},
    GroupType,
};
use leo_typed::{CircuitFieldDefinition, CircuitMember, Identifier, Span};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn enforce_circuit<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        identifier: Identifier,
        members: Vec<CircuitFieldDefinition>,
        span: Span,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        let mut program_identifier = new_scope(file_scope.clone(), identifier.to_string());

        if identifier.is_self() {
            program_identifier = file_scope.clone();
        }

        let circuit = match self.get(&program_identifier) {
            Some(value) => value.clone().extract_circuit(span.clone())?,
            None => return Err(ExpressionError::undefined_circuit(identifier.to_string(), span)),
        };

        let circuit_identifier = circuit.circuit_name.clone();
        let mut resolved_members = vec![];

        for member in circuit.members.clone().into_iter() {
            match member {
                CircuitMember::CircuitField(identifier, _type) => {
                    let matched_field = members
                        .clone()
                        .into_iter()
                        .find(|field| field.identifier.eq(&identifier));
                    match matched_field {
                        Some(field) => {
                            // Resolve and enforce circuit object
                            let field_value = self.enforce_expression(
                                cs,
                                file_scope.clone(),
                                function_scope.clone(),
                                &vec![_type.clone()],
                                field.expression,
                            )?;

                            resolved_members.push(ConstrainedCircuitMember(identifier, field_value))
                        }
                        None => return Err(ExpressionError::expected_circuit_member(identifier.to_string(), span)),
                    }
                }
                CircuitMember::CircuitFunction(_static, function) => {
                    let identifier = function.identifier.clone();
                    let mut constrained_function_value =
                        ConstrainedValue::Function(Some(circuit_identifier.clone()), function);

                    if _static {
                        constrained_function_value = ConstrainedValue::Static(Box::new(constrained_function_value));
                    }

                    resolved_members.push(ConstrainedCircuitMember(identifier, constrained_function_value));
                }
            };
        }

        Ok(ConstrainedValue::CircuitExpression(
            circuit_identifier.clone(),
            resolved_members,
        ))
    }
}
