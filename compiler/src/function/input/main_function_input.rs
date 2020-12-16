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

//! Allocates a main function input parameter in a compiled Leo program.

use crate::{
    address::Address,
    errors::FunctionError,
    program::ConstrainedProgram,
    value::{
        boolean::input::bool_from_input,
        field::input::field_from_input,
        group::input::group_from_input,
        ConstrainedValue,
    },
    GroupType,
    Integer,
};

use leo_asg::Type;
use leo_ast::{InputValue, Span};
use snarkvm_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn allocate_main_function_input<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        type_: &Type,
        name: &str,
        input_option: Option<InputValue>,
        span: &Span,
    ) -> Result<ConstrainedValue<F, G>, FunctionError> {
        match type_ {
            Type::Address => Ok(Address::from_input(cs, name, input_option, span)?),
            Type::Boolean => Ok(bool_from_input(cs, name, input_option, span)?),
            Type::Field => Ok(field_from_input(cs, name, input_option, span)?),
            Type::Group => Ok(group_from_input(cs, name, input_option, span)?),
            Type::Integer(integer_type) => Ok(ConstrainedValue::Integer(Integer::from_input(
                cs,
                integer_type,
                name,
                input_option,
                span,
            )?)),
            Type::Array(type_, len) => self.allocate_array(cs, name, &*type_, *len, input_option, span),
            Type::Tuple(types) => self.allocate_tuple(cs, &name, types, input_option, span),
            _ => unimplemented!("main function input not implemented for type"),
        }
    }
}
