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
use crate::{errors::FunctionError, program::ConstrainedProgram, value::ConstrainedValue, GroupType, Integer};

use crate::errors::ExpressionError;
use leo_core::{blake2s::unstable::hash::Blake2sFunction, call_core_function, CoreFunctionArgument};
use leo_typed::{Expression, Span, Type};
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{r1cs::ConstraintSystem, utilities::uint::UInt8},
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn enforce_core_function_call_expression<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        expected_type: Option<Type>,
        function: String,
        arguments: Vec<Expression>,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        println!("function call {}", function);
        println!("argument names {:?}", arguments);

        // Get the value of each core function argument
        let mut argument_values = vec![];
        for argument in arguments.into_iter() {
            let argument_value =
                self.enforce_expression(cs, file_scope.clone(), function_scope.clone(), None, argument)?;
            let core_function_argument = CoreFunctionArgument(argument_value.to_value());

            argument_values.push(core_function_argument);
        }
        // println!("argument values {:?}", argument_values);

        // Call the core function in `leo-core`
        let res = call_core_function(cs, function, argument_values);

        // Temporarily return empty array
        let empty = vec![ConstrainedValue::Integer(Integer::U8(UInt8::constant(0))); 32];

        return Ok(ConstrainedValue::Array(empty));
    }
}

// fn enforce_blake2s_function<F: Field + PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
//     cs: CS,
//     file_scope: String,
//     caller_scope: String,
//     arguments: Vec<ConstrainedValue<F, G>>,
//     span: Span,
// ) -> Result<ConstrainedValue<F, G>, FunctionError> {
//
//     // length of input to hash function must be 1
//     // if arguments.len() != 1 {
//     //     return Err(FunctionError::)
//     // }
//
//     let argument_expression = arguments[0].clone();
//
//     let argument_value =
//
//
//     return Ok(ConstrainedValue::Array(vec![]));
// }
