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

use leo_ast::{Expression, Function, FunctionInput, Input};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn enforce_main_function<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        scope: &str,
        function: Function,
        input: Input,
    ) -> Result<OutputBytes, FunctionError> {
        let function_name = new_scope(scope, function.get_name());
        let registers = input.get_registers();

        // Iterate over main function input variables and allocate new values
        let mut input_variables = Vec::with_capacity(function.input.len());
        for input_model in function.input.clone().into_iter() {
            let (identifier, value) = match input_model {
                FunctionInput::InputKeyword(identifier) => {
                    let value = self.allocate_input_keyword(cs, identifier.clone(), &input)?;

                    (identifier, value)
                }
                FunctionInput::Variable(input_model) => {
                    let name = input_model.identifier.name.clone();
                    let input_option = input
                        .get(&name)
                        .ok_or_else(|| FunctionError::input_not_found(name.clone(), function.span.clone()))?;
                    let input_value =
                        self.allocate_main_function_input(cs, input_model.type_, &name, input_option, &function.span)?;

                    (input_model.identifier, input_value)
                }
            };

            // Store input as variable with {function_name}_{identifier_name}
            let input_name = new_scope(&function_name, &identifier.name);

            // Store a new variable for every allocated main function input
            self.store(input_name, value);

            input_variables.push(Expression::Identifier(identifier));
        }

        let span = function.span.clone();
        let result_value = self.enforce_function(cs, scope, &function_name, function, input_variables, "")?;
        let output_bytes = OutputBytes::new_from_constrained_value(registers, result_value, span)?;

        Ok(output_bytes)
    }
}
