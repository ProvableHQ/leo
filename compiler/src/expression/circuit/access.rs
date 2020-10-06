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

//! Enforces a circuit access expression in a compiled Leo program.

use crate::{
    errors::ExpressionError,
    program::{new_scope, ConstrainedProgram},
    value::ConstrainedValue,
    GroupType,
};
use leo_typed::{Expression, Identifier, Span, Type};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

static SELF_KEYWORD: &str = "self";

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn enforce_circuit_access<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        expected_type: Option<Type>,
        circuit_identifier: Box<Expression>,
        circuit_member: Identifier,
        span: Span,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        // access a circuit member using the `self` keyword
        if let Expression::Identifier(ref identifier) = *circuit_identifier {
            if identifier.is_self() {
                let self_file_scope = new_scope(file_scope, identifier.name.to_string());
                let self_function_scope = new_scope(self_file_scope.clone(), identifier.name.to_string());

                let member_value =
                    self.evaluate_identifier(self_file_scope, self_function_scope, None, circuit_member)?;

                return Ok(member_value);
            }
        }

        let (circuit_name, members) = match self.enforce_operand(
            cs,
            file_scope.clone(),
            function_scope,
            expected_type,
            *circuit_identifier,
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
                            let variable = new_scope(self_keyword, stored_member.0.to_string());

                            self.store(variable, stored_member.1.clone());
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
