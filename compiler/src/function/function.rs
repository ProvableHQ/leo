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

//! Enforces constraints on a function in a compiled Leo program.

use crate::{
    errors::FunctionError,
    program::{new_scope, ConstrainedProgram},
    value::ConstrainedValue,
    GroupType,
};

use leo_asg::{Expression, FunctionBody, FunctionQualifier};
use std::sync::Arc;

use snarkvm_models::{
    curves::{Field, PrimeField},
    gadgets::{r1cs::ConstraintSystem, utilities::boolean::Boolean},
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub(crate) fn enforce_function<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        scope: &str,
        caller_scope: &str,
        function: &Arc<FunctionBody>,
        self_: Option<ConstrainedValue<F, G>>,
        arguments: &Vec<Arc<Expression>>,
    ) -> Result<ConstrainedValue<F, G>, FunctionError> {
        let function_name = new_scope(scope, &function.function.name.borrow().name.clone());

        // Store if function contains input `mut self`.
        let mut_self = function.function.qualifier == FunctionQualifier::MutSelfRef;

        // if function.function.has_input {
        //     // let input_var = function.scope.
        //     if let Some(input) = input {
        //         let variable = function.scope.borrow().resolve_variable("input").expect("no input variable in scope when function is qualified");

        //         let value =
        //             self.enforce_function_input(cs, scope, caller_scope, &function_name, None, input)?;

        //         self.store(variable.borrow().id.clone(), value);
        //     } else {
        //         return Err(FunctionError::input_not_found("input".to_string(), function.span.unwrap_or_default()));
        //     }
        // }

        // match function.function.qualifier {
        //     FunctionQualifier::SelfRef | FunctionQualifier::MutSelfRef => {
        //         if let Some(value) = self_ {
        //             // todo: enforce self & input
        //             let variable = function.scope.borrow().resolve_variable("self").expect("no self variable in scope when function is qualified");
        //             self.store(variable.borrow().id.clone(), value);
        //         } else {
        //             return Err(FunctionError::input_not_found("self".to_string(), function.span.unwrap_or_default()));
        //         }
        //     },
        //     FunctionQualifier::Static => (),
        // }
        if function.arguments.len() != arguments.len() {
            return Err(FunctionError::input_not_found("arguments length invalid".to_string(), function.span.unwrap_or_default()));
        }

        // Store input values as new variables in resolved program
        for (variable, input_expression) in function.arguments.iter().zip(arguments.into_iter()) {
            let variable = variable.borrow();

            let mut input_value = self.enforce_function_input(
                cs,
                scope,
                caller_scope,
                &function_name,
                Some(variable.type_.clone()),
                input_expression,
            )?;

            if variable.mutable {
                input_value = ConstrainedValue::Mutable(Box::new(input_value))
            }

            self.store(variable.id.clone(), input_value);
        }

        // Evaluate every statement in the function and save all potential results
        let mut results = vec![];
        let indicator = Boolean::constant(true);

        let output = function.function.output.clone().strong();

        let mut result = self.enforce_statement(
            cs,
            scope,
            &function_name,
            &indicator,
            &function.body,
            mut_self,
        )?;

        results.append(&mut result);

        // Conditionally select a result based on returned indicators
        Self::conditionally_select_result(cs, &output, results, &function.span.unwrap_or_default())
            .map_err(FunctionError::StatementError)
    }
}
