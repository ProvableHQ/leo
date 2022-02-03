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

//! Allocates an array as a main function input parameter in a compiled Leo program.

use crate::{program::ConstrainedProgram, value::ConstrainedValue, GroupType};

use leo_asg::Type;
use leo_ast::InputValue;
use leo_errors::{CompilerError, Result, Span};

use snarkvm_fields::PrimeField;
use snarkvm_r1cs::ConstraintSystem;

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    pub fn allocate_array<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        name: &str,
        array_type: &Type,
        array_len: usize,
        input_value: Option<InputValue>,
        span: &Span,
    ) -> Result<ConstrainedValue<'a, F, G>> {
        // Build the array value using the expected types.
        let mut array_value = vec![];

        match input_value {
            Some(InputValue::Array(arr)) => {
                if array_len != arr.len() {
                    return Err(CompilerError::invalid_input_array_dimensions(arr.len(), array_len, span).into());
                }

                // Allocate each value in the current row
                for (i, value) in arr.into_iter().enumerate() {
                    let value_name = format!("{}_{}", &name, &i.to_string());

                    array_value.push(self.allocate_main_function_input(
                        cs,
                        array_type,
                        &value_name,
                        Some(value),
                        span,
                    )?)
                }
            }
            None => {
                // Allocate all row values as none
                for i in 0..array_len {
                    let value_name = format!("{}_{}", &name, &i.to_string());

                    array_value.push(self.allocate_main_function_input(cs, array_type, &value_name, None, span)?);
                }
            }
            _ => {
                return Err(CompilerError::invalid_function_input_array(input_value.unwrap(), span).into());
            }
        }

        Ok(ConstrainedValue::Array(array_value))
    }
}
