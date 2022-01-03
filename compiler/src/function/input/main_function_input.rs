// Copyright (C) 2019-2022 Aleo Systems Inc.
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
    program::ConstrainedProgram,
    value::{
        boolean::input::bool_from_input, char::char_from_input, field::input::field_from_input,
        group::input::group_from_input, ConstrainedValue,
    },
    CharType, FieldType, GroupType, Integer,
};
use leo_asg::{ConstInt, Type};
use leo_ast::{Char, InputValue};
use leo_errors::{CompilerError, Result, Span};

use snarkvm_fields::PrimeField;
use snarkvm_gadgets::boolean::Boolean;
use snarkvm_r1cs::ConstraintSystem;

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    pub fn allocate_main_function_input<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        type_: &Type,
        name: &str,
        input_option: Option<InputValue>,
        span: &Span,
    ) -> Result<ConstrainedValue<'a, F, G>> {
        match type_ {
            Type::Address => Ok(Address::from_input(cs, name, input_option, span)?),
            Type::Boolean => Ok(bool_from_input(cs, name, input_option, span)?),
            Type::Char => Ok(char_from_input(cs, name, input_option, span)?),
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
            Type::Tuple(types) => self.allocate_tuple(cs, name, types, input_option, span),
            _ => unimplemented!("main function input not implemented for type {}", type_), // Should not happen.
        }
    }
}

/// Process constant inputs and return ConstrainedValue with constant value in it.
impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    pub fn constant_main_function_input<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        type_: &Type,
        name: &str,
        input_option: Option<InputValue>,
        span: &Span,
    ) -> Result<ConstrainedValue<'a, F, G>> {
        let input = input_option.ok_or_else(|| CompilerError::function_input_not_found("main", name, span))?;

        match (type_, input) {
            (Type::Address, InputValue::Address(addr)) => Ok(ConstrainedValue::Address(Address::constant(addr, span)?)),
            (Type::Boolean, InputValue::Boolean(value)) => Ok(ConstrainedValue::Boolean(Boolean::constant(value))),
            (Type::Char, InputValue::Char(character)) => match character.character {
                Char::Scalar(scalar) => Ok(ConstrainedValue::Char(crate::Char::constant(
                    cs,
                    CharType::Scalar(scalar),
                    format!("{}", scalar as u32),
                    span,
                )?)),
                Char::NonScalar(non_scalar) => Ok(ConstrainedValue::Char(crate::Char::constant(
                    cs,
                    CharType::NonScalar(non_scalar),
                    format!("{}", non_scalar),
                    span,
                )?)),
            },
            (Type::Field, InputValue::Field(value)) => {
                Ok(ConstrainedValue::Field(FieldType::constant(cs, value, span)?))
            }
            (Type::Group, InputValue::Group(value)) => Ok(ConstrainedValue::Group(G::constant(&value.into(), span)?)),
            (Type::Integer(integer_type), InputValue::Integer(input_type, value)) => {
                let parsed = ConstInt::parse(integer_type, &value, span)?;
                let parsed_type = parsed.get_int_type();
                let input_type = input_type.into();
                if std::mem::discriminant(&parsed_type) != std::mem::discriminant(&input_type) {
                    return Err(
                        CompilerError::integer_value_integer_type_mismatch(input_type, parsed_type, span).into(),
                    );
                }
                Ok(ConstrainedValue::Integer(Integer::new(&parsed)))
            }
            (Type::Array(type_, arr_len), InputValue::Array(values)) => {
                if *arr_len != values.len() {
                    return Err(CompilerError::invalid_input_array_dimensions(*arr_len, values.len(), span).into());
                }

                Ok(ConstrainedValue::Array(
                    values
                        .iter()
                        .map(|x| self.constant_main_function_input(cs, type_, name, Some(x.clone()), span))
                        .collect::<Result<Vec<_>, _>>()?,
                ))
            }
            (Type::Tuple(types), InputValue::Tuple(values)) => {
                if values.len() != types.len() {
                    return Err(CompilerError::input_tuple_size_mismatch(types.len(), values.len(), span).into());
                }

                Ok(ConstrainedValue::Tuple(
                    values
                        .iter()
                        .enumerate()
                        .map(|(i, x)| {
                            self.constant_main_function_input(cs, types.get(i).unwrap(), name, Some(x.clone()), span)
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                ))
            }
            (Type::Circuit(_), _) => unimplemented!("main function input not implemented for type {}", type_), // Should not happen.

            // Return an error if the input type and input value do not match.
            (_, input) => Err(CompilerError::input_variable_type_mismatch(type_, input, name, span).into()),
        }
    }
}
