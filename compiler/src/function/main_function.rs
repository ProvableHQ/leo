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
    Output,
};

use leo_typed::{Expression, Function, Input, InputVariable};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::{ConstraintSystem, Index},
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn enforce_main_function<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        scope: &str,
        function: Function,
        input: Input,
    ) -> Result<Output, FunctionError> {
        let function_name = new_scope(scope, function.get_name());
        let registers = input.get_registers();

        // Iterate over main function input variables and allocate new values
        let mut input_variables = Vec::with_capacity(function.input.len());
        let mut cs_input_indices: Vec<Index> = Vec::with_capacity(0);
        for (i, input_model) in function.input.clone().into_iter().enumerate() {
            let (identifier, value) = match input_model {
                InputVariable::InputKeyword(identifier) => {
                    let value = self.allocate_input_keyword(cs, identifier.clone(), &input)?;

                    (identifier, value)
                }
                InputVariable::FunctionInput(input_model) => {
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

            // Store constraint system input variable indices for serialization.
            let mut indices = value.get_constraint_system_indices(cs.ns(|| format!("input index {}", i)));
            cs_input_indices.append(&mut indices);

            // Store a new variable for every allocated main function input
            self.store(input_name, value);

            input_variables.push(Expression::Identifier(identifier));
        }

        let span = function.span.clone();
        let result_value = self.enforce_function(cs, scope, &function_name, function, input_variables, "")?;

        // Lookup result value constraint variable indices.
        let cs_output_indices = result_value.get_constraint_system_indices(cs);

        let output_bytes = Output::new(registers, result_value, cs_input_indices, cs_output_indices, span)?;

        Ok(output_bytes)
    }
}
