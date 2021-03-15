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
    FieldType,
    GroupType,
    Integer,
};
use leo_asg::{ConstInt, Type};
use leo_ast::{InputValue, IntegerType, Span};

use snarkvm_fields::PrimeField;
use snarkvm_gadgets::traits::utilities::boolean::Boolean;
use snarkvm_r1cs::ConstraintSystem;

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    pub fn allocate_main_function_input<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        type_: &Type,
        name: &str,
        input_option: Option<InputValue>,
        span: &Span,
    ) -> Result<ConstrainedValue<'a, F, G>, FunctionError> {
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
            _ => unimplemented!("main function input not implemented for type {}", type_),
        }
    }
}

/// Process constant inputs and return ConstrainedValue with constant value in it.
impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    pub fn constant_main_function_input<CS: ConstraintSystem<F>>(
        &mut self,
        // TODO: remove unnecessary arguments
        _cs: &mut CS,
        type_: &Type,
        name: &str,
        input_option: Option<InputValue>,
        span: &Span,
    ) -> Result<ConstrainedValue<'a, F, G>, FunctionError> {
        let value = input_option.unwrap();

        Ok(match value {
            InputValue::Address(value) => ConstrainedValue::Address(Address::constant(value, span)?),
            InputValue::Boolean(value) => ConstrainedValue::Boolean(Boolean::constant(value)),
            InputValue::Field(value) => ConstrainedValue::Field(FieldType::constant(value, span)?),
            InputValue::Group(value) => ConstrainedValue::Group(G::constant(&value.into(), span)?),
            InputValue::Integer(integer_type, value) => {
                let integer = IntegerType::from(integer_type);
                let const_int = match integer {
                    IntegerType::U8 => ConstInt::U8(value.parse::<u8>().unwrap()),
                    IntegerType::U16 => ConstInt::U16(value.parse::<u16>().unwrap()),
                    IntegerType::U32 => ConstInt::U32(value.parse::<u32>().unwrap()),
                    IntegerType::U64 => ConstInt::U64(value.parse::<u64>().unwrap()),
                    IntegerType::U128 => ConstInt::U128(value.parse::<u128>().unwrap()),

                    IntegerType::I8 => ConstInt::I8(value.parse::<i8>().unwrap()),
                    IntegerType::I16 => ConstInt::I16(value.parse::<i16>().unwrap()),
                    IntegerType::I32 => ConstInt::I32(value.parse::<i32>().unwrap()),
                    IntegerType::I64 => ConstInt::I64(value.parse::<i64>().unwrap()),
                    IntegerType::I128 => ConstInt::I128(value.parse::<i128>().unwrap()),
                };

                ConstrainedValue::Integer(Integer::new(&const_int))
            }
            InputValue::Array(values) => {
                // Get ASG type and array length to compare with provided inputs.
                let (type_, arr_len) = if let Type::Array(type_, len) = type_ {
                    (type_, *len)
                } else {
                    return Err(FunctionError::input_not_found("expected".to_string(), &span));
                };

                if arr_len != values.len() {
                    return Err(FunctionError::invalid_input_array_dimensions(
                        arr_len,
                        values.len(),
                        span,
                    ));
                }

                ConstrainedValue::Array(
                    values
                        .iter()
                        .map(|x| self.constant_main_function_input(_cs, type_, name, Some(x.clone()), span))
                        .collect::<Result<Vec<_>, _>>()?,
                )
            }
            InputValue::Tuple(values) => {
                // Get ASG tuple size and compare it to input tuple size.
                let tuple_len = if let Type::Tuple(types) = type_ {
                    types.len()
                } else {
                    return Err(FunctionError::tuple_size_mismatch(0, values.len(), span));
                };

                if values.len() != tuple_len {
                    return Err(FunctionError::tuple_size_mismatch(tuple_len, values.len(), span));
                }

                ConstrainedValue::Tuple(
                    values
                        .iter()
                        .map(|x| self.constant_main_function_input(_cs, type_, name, Some(x.clone()), span))
                        .collect::<Result<Vec<_>, _>>()?,
                )
            }
        })
    }
}
