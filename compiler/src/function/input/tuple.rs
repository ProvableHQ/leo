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

//! Allocates an array as a main function input parameter in a compiled Leo program.

use crate::{
    errors::FunctionError,
    program::{new_scope, ConstrainedProgram},
    value::ConstrainedValue,
    GroupType,
};

use leo_typed::{InputValue, Span, Type};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn allocate_tuple<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        name: &str,
        types: Vec<Type>,
        input_value: Option<InputValue>,
        span: &Span,
    ) -> Result<ConstrainedValue<F, G>, FunctionError> {
        let mut tuple_values = vec![];

        match input_value {
            Some(InputValue::Tuple(values)) => {
                // Allocate each value in the tuple
                for (i, (value, type_)) in values.into_iter().zip(types.into_iter()).enumerate() {
                    let value_name = new_scope(name, &i.to_string());

                    tuple_values.push(self.allocate_main_function_input(cs, type_, &value_name, Some(value), span)?)
                }
            }
            None => {
                // Allocate all tuple values as none
                for (i, type_) in types.into_iter().enumerate() {
                    let value_name = new_scope(name, &i.to_string());

                    tuple_values.push(self.allocate_main_function_input(cs, type_, &value_name, None, span)?);
                }
            }
            _ => {
                return Err(FunctionError::invalid_tuple(
                    input_value.unwrap().to_string(),
                    span.to_owned(),
                ));
            }
        }

        Ok(ConstrainedValue::Tuple(tuple_values))
    }
}
