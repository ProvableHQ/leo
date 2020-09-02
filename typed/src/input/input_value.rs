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

use crate::{Expression as TypedExpression, GroupValue};
use leo_input::{
    errors::InputParserError,
    expressions::{ArrayInitializerExpression, ArrayInlineExpression, Expression, TupleExpression},
    types::{ArrayElement, ArrayType, DataType, IntegerType, TupleType, Type},
    values::{Address, AddressValue, BooleanValue, FieldValue, GroupValue as InputGroupValue, NumberValue, Value},
};

use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub enum InputValue {
    Address(String),
    Boolean(bool),
    Field(String),
    Group(GroupValue),
    Integer(IntegerType, String),
    Array(Vec<InputValue>),
    Tuple(Vec<InputValue>),
}

impl InputValue {
    fn from_address(address: Address) -> Self {
        InputValue::Address(address.value)
    }

    fn from_address_value(value: AddressValue) -> Self {
        match value {
            AddressValue::Explicit(address) => Self::from_address(address.address),
            AddressValue::Implicit(address) => Self::from_address(address),
        }
    }

    fn from_boolean(boolean: BooleanValue) -> Result<Self, InputParserError> {
        let boolean = boolean.value.parse::<bool>()?;
        Ok(InputValue::Boolean(boolean))
    }

    fn from_number(integer_type: IntegerType, number: String) -> Result<Self, InputParserError> {
        Ok(InputValue::Integer(integer_type, number))
    }

    fn from_group(group: InputGroupValue) -> Self {
        InputValue::Group(GroupValue::from(group))
    }

    fn from_field(field: FieldValue) -> Self {
        InputValue::Field(field.number.to_string())
    }

    fn from_implicit(data_type: DataType, implicit: NumberValue) -> Result<Self, InputParserError> {
        match data_type {
            DataType::Address(_) => Err(InputParserError::implicit_type(data_type, implicit)),
            DataType::Boolean(_) => Err(InputParserError::implicit_type(data_type, implicit)),
            DataType::Integer(integer_type) => InputValue::from_number(integer_type, implicit.to_string()),
            DataType::Group(_) => Err(InputParserError::implicit_group(implicit)),
            DataType::Field(_) => Ok(InputValue::Field(implicit.to_string())),
        }
    }

    fn from_value(data_type: DataType, value: Value) -> Result<Self, InputParserError> {
        match (data_type, value) {
            (DataType::Address(_), Value::Address(address)) => Ok(InputValue::from_address_value(address)),
            (DataType::Boolean(_), Value::Boolean(boolean)) => InputValue::from_boolean(boolean),
            (DataType::Integer(integer_type), Value::Integer(integer)) => {
                InputValue::from_number(integer_type, integer.to_string())
            }
            (DataType::Group(_), Value::Group(group)) => Ok(InputValue::from_group(group)),
            (DataType::Field(_), Value::Field(field)) => Ok(InputValue::from_field(field)),
            (data_type, Value::Implicit(implicit)) => InputValue::from_implicit(data_type, implicit),
            (data_type, value) => Err(InputParserError::data_type_mismatch(data_type, value)),
        }
    }

    pub(crate) fn from_expression(type_: Type, expression: Expression) -> Result<Self, InputParserError> {
        match (type_, expression) {
            (Type::Basic(data_type), Expression::Value(value)) => InputValue::from_value(data_type, value),
            (Type::Array(array_type), Expression::ArrayInline(inline)) => {
                InputValue::from_array_inline(array_type, inline)
            }
            (Type::Array(array_type), Expression::ArrayInitializer(initializer)) => {
                InputValue::from_array_initializer(array_type, initializer)
            }
            (Type::Tuple(tuple_type), Expression::Tuple(tuple)) => InputValue::from_tuple(tuple_type, tuple),
            (type_, expression) => Err(InputParserError::expression_type_mismatch(type_, expression)),
        }
    }

    pub(crate) fn from_array_inline(
        mut array_type: ArrayType,
        inline: ArrayInlineExpression,
    ) -> Result<Self, InputParserError> {
        let mut array_dimensions = TypedExpression::get_input_array_dimensions(array_type.dimensions.clone());

        // Return an error if the outer array dimension does not equal the number of array elements
        if let Some(outer_dimension) = array_dimensions.pop() {
            array_type.dimensions = array_type.dimensions.next_dimension();

            if outer_dimension != inline.expressions.len() {
                return Err(InputParserError::array_inline_length(outer_dimension, inline));
            }
        };

        let inner_array_type = if array_dimensions.len() == 0 {
            // this is a single array
            match array_type.type_ {
                ArrayElement::Basic(basic) => Type::Basic(basic),
                ArrayElement::Tuple(tuple) => Type::Tuple(tuple),
            }
        } else {
            // this is a multi-dimensional array
            Type::Array(array_type)
        };

        let mut values = vec![];
        for expression in inline.expressions.into_iter() {
            let value = InputValue::from_expression(inner_array_type.clone(), expression)?;

            values.push(value)
        }

        Ok(InputValue::Array(values))
    }

    pub(crate) fn from_array_initializer(
        mut array_type: ArrayType,
        initializer: ArrayInitializerExpression,
    ) -> Result<Self, InputParserError> {
        let mut array_dimensions = TypedExpression::get_input_array_dimensions(array_type.dimensions.clone());
        let initializer_count = initializer.count.to_string().parse::<usize>()?;

        // Return an error if the outer array dimension does not equal the number of array elements
        if let Some(outer_dimension) = array_dimensions.pop() {
            array_type.dimensions = array_type.dimensions.next_dimension();

            if outer_dimension != initializer_count {
                return Err(InputParserError::array_init_length(outer_dimension, initializer));
            }
        }

        let inner_array_type = if array_dimensions.len() == 0 {
            // this is a single array
            match array_type.type_ {
                ArrayElement::Basic(basic) => Type::Basic(basic),
                ArrayElement::Tuple(tuple) => Type::Tuple(tuple),
            }
        } else {
            // this is a multi-dimensional array
            Type::Array(array_type)
        };

        let mut values = vec![];
        for _ in 0..initializer_count {
            let value = InputValue::from_expression(inner_array_type.clone(), *initializer.expression.clone())?;

            values.push(value)
        }

        Ok(InputValue::Array(values))
    }

    pub(crate) fn from_tuple(tuple_type: TupleType, tuple: TupleExpression) -> Result<Self, InputParserError> {
        let num_types = tuple_type.types_.len();
        let num_values = tuple.expressions.len();

        if num_types != num_values {
            return Err(InputParserError::tuple_length(
                num_types,
                num_values,
                tuple_type.span.clone(),
            ));
        }

        let mut values = vec![];
        for (type_, value) in tuple_type.types_.into_iter().zip(tuple.expressions.into_iter()) {
            let value = InputValue::from_expression(type_, value)?;

            values.push(value)
        }

        Ok(InputValue::Tuple(values))
    }
}

impl fmt::Display for InputValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputValue::Address(ref address) => write!(f, "{}", address),
            InputValue::Boolean(ref boolean) => write!(f, "{}", boolean),
            InputValue::Group(ref group) => write!(f, "{}", group),
            InputValue::Field(ref field) => write!(f, "{}", field),
            InputValue::Integer(ref type_, ref number) => write!(f, "{}{:?}", number, type_),
            InputValue::Array(ref array) => {
                let values = array.iter().map(|x| format!("{}", x)).collect::<Vec<_>>().join(", ");

                write!(f, "array [{}]", values)
            }
            InputValue::Tuple(ref tuple) => {
                let values = tuple.iter().map(|x| format!("{}", x)).collect::<Vec<_>>().join(", ");

                write!(f, "({})", values)
            }
        }
    }
}
