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

use crate::{program::ConstrainedProgram, GroupType, Output};

use leo_asg::{Expression, Function, FunctionQualifier};
use leo_ast::Input;
use leo_errors::{CompilerError, LeoError};
use std::cell::Cell;

use snarkvm_fields::PrimeField;
use snarkvm_r1cs::ConstraintSystem;

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    pub fn enforce_main_function<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        function: &'a Function<'a>,
        input: &Input,
    ) -> Result<Output, LeoError> {
        let registers = input.get_registers();

        // Iterate over main function input variables and allocate new values
        let asg_input = function.scope.resolve_input();

        if let Some(asg_input) = asg_input {
            let value =
                self.allocate_input_keyword(cs, &function.name.borrow().span, &asg_input.container_circuit, input)?;

            self.store(asg_input.container.borrow().id, value);
        }

        match function.qualifier {
            FunctionQualifier::SelfRef | FunctionQualifier::ConstSelfRef | FunctionQualifier::MutSelfRef => {
                unimplemented!("cannot access self variable in main function")
            }
            FunctionQualifier::Static => (),
        }

        let mut arguments = vec![];

        for (_, input_variable) in function.arguments.iter() {
            {
                let input_variable = input_variable.get().borrow();
                let name = input_variable.name.name.clone();

                let input_value = match (
                    input_variable.const_,
                    input.get(&name),
                    input.get_constant(name.as_ref()),
                ) {
                    // If variable is in both [main] and [constants] sections - error.
                    (_, Some(_), Some(_)) => {
                        return Err(LeoError::from(CompilerError::double_input_declaration(
                            name.to_string(),
                            &input_variable.name.span,
                        )));
                    }
                    // If input option is found in [main] section and input is not const.
                    (false, Some(input_option), _) => self.allocate_main_function_input(
                        cs,
                        &input_variable.type_.clone(),
                        &name,
                        input_option,
                        &input_variable.name.span,
                    )?,
                    // If input option is found in [constants] section and function argument is const.
                    (true, _, Some(input_option)) => self.constant_main_function_input(
                        cs,
                        &input_variable.type_.clone(),
                        &name,
                        input_option,
                        &input_variable.name.span,
                    )?,
                    // Function argument is const, input is not.
                    (true, Some(_), None) => {
                        return Err(LeoError::from(CompilerError::expected_const_input_variable(
                            name.to_string(),
                            &input_variable.name.span,
                        )));
                    }
                    // Input is const, function argument is not.
                    (false, None, Some(_)) => {
                        return Err(LeoError::from(CompilerError::expected_non_const_input_variable(
                            name.to_string(),
                            &input_variable.name.span,
                        )));
                    }
                    // When not found - Error out.
                    (_, _, _) => {
                        return Err(LeoError::from(CompilerError::function_input_not_found(
                            function.name.borrow().name.to_string(),
                            name.to_string(),
                            &input_variable.name.span,
                        )));
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
        let output = Output::new(&self.asg, registers, result_value, &span)?;

        Ok(output)
    }
}
