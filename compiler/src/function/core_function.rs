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
use crate::{program::ConstrainedProgram, value::ConstrainedValue, GroupType, Integer};

use crate::errors::ExpressionError;
use leo_core::{call_core_function, CoreFunctionArgument};
use leo_typed::{Expression, Type};
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn enforce_core_function_call_expression<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        _expected_type: Option<Type>,
        function: String,
        arguments: Vec<Expression>,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        // Get the value of each core function argument
        let mut argument_values = vec![];
        for argument in arguments.into_iter() {
            let argument_value =
                self.enforce_expression(cs, file_scope.clone(), function_scope.clone(), None, argument)?;
            let core_function_argument = CoreFunctionArgument(argument_value.to_value());

            argument_values.push(core_function_argument);
        }

        // Call the core function in `leo-core`
        let res = call_core_function(cs, function, argument_values);

        let array = res
            .into_iter()
            .map(|uint| ConstrainedValue::Integer(Integer::U8(uint)))
            .collect();

        return Ok(ConstrainedValue::Array(array));
    }
}
