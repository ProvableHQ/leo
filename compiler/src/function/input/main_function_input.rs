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

// use crate::value::{Address, ConstrainedValue, Integer};

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
            _ => unimplemented!("main function input not implemented for type"),
        }
    }
}

/// Process constant inputs and return ConstrainedValue with constant value in it.
impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    pub fn parse_constant_main_function_input<CS: ConstraintSystem<F>>(
        &mut self,
        // TODO: remove unnecessary arguments
        _cs: &mut CS,
        _type_: &Type,
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
            // TODO: array and tuple values.
            // InputValue::Array(Vec<InputValue>) => ,
            // InputValue::Tuple(Vec<InputValue>),
            // TODO: rework this error to something better fitting into context.
            _ => return Err(FunctionError::input_not_found(name.to_string(), span)),
        })
    }
}
