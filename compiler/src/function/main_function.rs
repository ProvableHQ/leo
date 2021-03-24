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

//! Enforces constraints on the main function of a compiled Leo program.

use crate::{errors::FunctionError, program::ConstrainedProgram, GroupType, OutputBytes};

use leo_asg::{Expression, Function, FunctionQualifier};
use leo_ast::Input;
use std::cell::Cell;

use snarkvm_fields::PrimeField;
use snarkvm_r1cs::ConstraintSystem;

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    pub fn enforce_main_function<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        function: &'a Function<'a>,
        input: &Input,
    ) -> Result<OutputBytes, FunctionError> {
        let registers = input.get_registers();

        // Iterate over main function input variables and allocate new values
        if function.has_input {
            // let input_var = function.scope.
            let asg_input = function
                .scope
                .resolve_input()
                .expect("no input variable in scope when function is qualified");

            let value =
                self.allocate_input_keyword(cs, &function.name.borrow().span, &asg_input.container_circuit, input)?;

            self.store(asg_input.container.borrow().id, value);
        }

        match function.qualifier {
            FunctionQualifier::SelfRef | FunctionQualifier::MutSelfRef => {
                unimplemented!("cannot access self variable in main function")
            }
            FunctionQualifier::Static => (),
        }

        let mut arguments = vec![];

        for (_, input_variable) in function.arguments.iter() {
            {
                let input_variable = input_variable.get().borrow();
                let name = input_variable.name.name.clone();

                let input_value = match (input_variable.const_, input.get(&name), input.get_constant(&name)) {
                    // If variable is in both [main] and [constants] sections - error.
                    (_, Some(_), Some(_)) => {
                        return Err(FunctionError::double_input_declaration(
                            name.clone(),
                            &function.span.clone().unwrap_or_default(),
                        ));
                    }
                    // If input option is found in [main] section and input is not const.
                    (false, Some(input_option), _) => self.allocate_main_function_input(
                        cs,
                        &input_variable.type_.clone(),
                        &name,
                        input_option,
                        &function.span.clone().unwrap_or_default(),
                    )?,
                    // If input option is found in [constants] section and function argument is const.
                    (true, _, Some(input_option)) => self.constant_main_function_input(
                        cs,
                        &input_variable.type_.clone(),
                        &name,
                        input_option,
                        &function.span.clone().unwrap_or_default(),
                    )?,
                    // Function argument is const, input is not.
                    (true, Some(_), None) => {
                        return Err(FunctionError::expected_const_input(
                            name.clone(),
                            &function.span.clone().unwrap_or_default(),
                        ));
                    }
                    // Input is const, function argument is not.
                    (false, None, Some(_)) => {
                        return Err(FunctionError::expected_non_const_input(
                            name.clone(),
                            &function.span.clone().unwrap_or_default(),
                        ));
                    }
                    // When not found - Error out.
                    (_, _, _) => {
                        return Err(FunctionError::input_not_found(
                            name.clone(),
                            &function.span.clone().unwrap_or_default(),
                        ));
                    }
                };

                // Store a new variable for every function input.
                self.store(input_variable.id, input_value);
            }
            arguments.push(Cell::new(&*function.scope.context.alloc_expression(
                Expression::VariableRef(leo_asg::VariableRef {
                    parent: Cell::new(None),
                    span: Some(input_variable.get().borrow().name.span.clone()),
                    variable: input_variable.get(),
                }),
            )));
        }

        let span = function.span.clone().unwrap_or_default();
        let result_value = self.enforce_function(cs, function, None, &arguments)?;
        let output_bytes = OutputBytes::new_from_constrained_value(&self.asg, registers, result_value, &span)?;

        Ok(output_bytes)
    }
}
