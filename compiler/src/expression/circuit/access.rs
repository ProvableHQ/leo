//! Enforces a circuit access expression in a compiled Leo program.

use crate::{
    errors::ExpressionError,
    program::{new_scope, ConstrainedProgram},
    value::ConstrainedValue,
    GroupType,
};
use leo_types::{Expression, Identifier, Span, Type};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

static SELF_KEYWORD: &'static str = "self";

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn enforce_circuit_access<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        expected_types: &Vec<Type>,
        circuit_identifier: Box<Expression>,
        circuit_member: Identifier,
        span: Span,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        // access a circuit member using the `self` keyword
        if let Expression::Identifier(ref identifier) = *circuit_identifier {
            if identifier.is_self() {
                let self_file_scope = new_scope(file_scope.clone(), identifier.name.to_string());
                let self_function_scope = new_scope(self_file_scope.clone(), identifier.name.to_string());

                let member_value =
                    self.evaluate_identifier(self_file_scope, self_function_scope, &vec![], circuit_member.clone())?;

                return Ok(member_value);
            }
        }

        let (circuit_name, members) = match self.enforce_operand(
            cs,
            file_scope.clone(),
            function_scope.clone(),
            expected_types,
            *circuit_identifier.clone(),
            span.clone(),
        )? {
            ConstrainedValue::CircuitExpression(name, members) => (name, members),
            value => return Err(ExpressionError::undefined_circuit(value.to_string(), span)),
        };

        let matched_member = members.clone().into_iter().find(|member| member.0 == circuit_member);

        match matched_member {
            Some(member) => {
                match &member.1 {
                    ConstrainedValue::Function(ref _circuit_identifier, ref _function) => {
                        // Pass circuit members into function call by value
                        for stored_member in members {
                            let circuit_scope = new_scope(file_scope.clone(), circuit_name.to_string());
                            let self_keyword = new_scope(circuit_scope, SELF_KEYWORD.to_string());
                            let field = new_scope(self_keyword, stored_member.0.to_string());

                            self.store(field, stored_member.1.clone());
                        }
                    }
                    ConstrainedValue::Static(value) => {
                        return Err(ExpressionError::invalid_static_access(value.to_string(), span));
                    }
                    _ => {}
                }
                Ok(member.1)
            }
            None => Err(ExpressionError::undefined_member_access(
                circuit_name.to_string(),
                circuit_member.to_string(),
                span,
            )),
        }
    }
}
