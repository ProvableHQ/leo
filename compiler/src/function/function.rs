// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use crate::errors::FunctionError;
use crate::program::ConstrainedProgram;
use crate::value::ConstrainedValue;
use crate::GroupType;

use leo_asg::Expression;
use leo_asg::Function;
use leo_asg::FunctionQualifier;
use std::cell::Cell;

use snarkvm_fields::PrimeField;
use snarkvm_gadgets::traits::utilities::boolean::Boolean;
use snarkvm_r1cs::ConstraintSystem;

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    pub(crate) fn enforce_function<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        function: &'a Function<'a>,
        target: Option<&'a Expression<'a>>,
        arguments: &[Cell<&'a Expression<'a>>],
    ) -> Result<ConstrainedValue<'a, F, G>, FunctionError> {
        let target_value = target.map(|target| self.enforce_expression(cs, target)).transpose()?;

        let self_var = if let Some(target) = &target_value {
            let self_var = function
                .scope
                .resolve_variable("self")
                .expect("attempted to call static function from non-static context");
            self.store(self_var.borrow().id, target.clone());
            Some(self_var)
        } else {
            None
        };

        if function.arguments.len() != arguments.len() {
            return Err(FunctionError::input_not_found(
                "arguments length invalid".to_string(),
                function.span.clone().unwrap_or_default(),
            ));
        }

        // Store input values as new variables in resolved program
        for ((_, variable), input_expression) in function.arguments.iter().zip(arguments.iter()) {
            let input_value = self.enforce_expression(cs, input_expression.get())?;
            let variable = variable.get().borrow();

            self.store(variable.id, input_value);
        }

        // Evaluate every statement in the function and save all potential results
        let mut results = vec![];
        let indicator = Boolean::constant(true);

        let output = function.output.clone();

        let mut result = self.enforce_statement(
            cs,
            &indicator,
            function.body.get().expect("attempted to call function header"),
        )?;

        results.append(&mut result);

        if function.qualifier == FunctionQualifier::MutSelfRef {
            if let (Some(self_var), Some(target)) = (self_var, target) {
                let new_self = self
                    .get(self_var.borrow().id)
                    .expect("no self variable found in mut self context")
                    .clone();
                if let Some(assignable_target) = self.resolve_mut_ref(cs, target)? {
                    if assignable_target.len() != 1 {
                        panic!("found tuple as a self assignment target");
                    }
                    let assignable_target = assignable_target.into_iter().next().unwrap();
                    *assignable_target = new_self;
                } else {
                    // todo: we should report a warning for calling a mutable function on an effectively copied self (i.e. wasn't assignable `tempStruct {x: 5}.myMutSelfFunction()`)
                }
            }
        }

        // Conditionally select a result based on returned indicators
        Self::conditionally_select_result(cs, &output, results, &function.span.clone().unwrap_or_default())
            .map_err(FunctionError::StatementError)
    }
}
