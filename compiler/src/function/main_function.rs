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

//! Enforces constraints on the main function of a compiled Leo program.

use crate::{
    errors::FunctionError,
    program::{new_scope, ConstrainedProgram},
    GroupType,
    OutputBytes,
};

use leo_asg::{Expression, FunctionBody, FunctionQualifier};
use leo_ast::Input;
use std::sync::Arc;

use snarkvm_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn enforce_main_function<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        scope: &str,
        function: &Arc<FunctionBody>,
        input: &Input,
    ) -> Result<OutputBytes, FunctionError> {
        let function_name = new_scope(scope, &function.function.name.borrow().name.clone());
        let registers = input.get_registers();

        // Iterate over main function input variables and allocate new values
        if function.function.has_input {
            // let input_var = function.scope.
            let variable = function.scope.borrow().resolve_variable("input").expect("no input variable in scope when function is qualified");

            let value = self.allocate_input_keyword(cs, function.function.name.borrow().span.clone(), input)?;

            self.store(variable.borrow().id.clone(), value);
        }


        match function.function.qualifier {
            FunctionQualifier::SelfRef | FunctionQualifier::MutSelfRef => unimplemented!("cannot access self variable in main function"),
            FunctionQualifier::Static => (),
        }

        let mut arguments = vec![];
        
        for input_variable in function.arguments.iter() {
            {
                let input_variable = input_variable.borrow();
                let name = input_variable.name.name.clone();
                let input_option = input
                    .get(&name)
                    .ok_or_else(|| FunctionError::input_not_found(name.clone(), function.span.clone().unwrap_or_default()))?;
                let input_value =
                    self.allocate_main_function_input(cs, &input_variable.type_, &name, input_option, &function.span.clone().unwrap_or_default())?;

                // Store a new variable for every allocated main function input
                self.store(input_variable.id.clone(), input_value);
            }
            arguments.push(Arc::new(Expression::VariableRef(leo_asg::VariableRef {
                parent: std::cell::RefCell::new(None),
                span: Some(input_variable.borrow().name.span.clone()),
                variable: input_variable.clone(),
            })));
        }

        let span = function.span.clone().unwrap_or_default();
        let result_value = self.enforce_function(cs, scope, &function_name, function, None, &arguments)?;
        let output_bytes = OutputBytes::new_from_constrained_value(registers, result_value, span)?;

        Ok(output_bytes)
    }
}
