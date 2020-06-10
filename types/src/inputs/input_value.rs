use crate::InputFields;
use leo_inputs::{
    errors::InputParserError,
    expressions::{ArrayInitializerExpression, ArrayInlineExpression, Expression},
    types::{ArrayType, DataType, IntegerType, Type},
    values::{BooleanValue, FieldValue, GroupValue, NumberImplicitValue, NumberValue, Value},
};
use snarkos_models::curves::PairingEngine;
use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub enum InputValue {
    Integer(IntegerType, u128),
    Field(String),
    Group(String),
    Boolean(bool),
    Array(Vec<InputValue>),
}

impl<'ast> InputValue {
    fn from_boolean(boolean: BooleanValue<'ast>) -> Result<Self, InputParserError> {
        let boolean = boolean.value.parse::<bool>()?;
        Ok(InputValue::Boolean(boolean))
    }

    fn from_number(integer_type: IntegerType, number: NumberValue<'ast>) -> Result<Self, InputParserError> {
        let integer = number.value.parse::<u128>()?;
        Ok(InputValue::Integer(integer_type, integer))
    }

    fn from_group(group: GroupValue<'ast>) -> Self {
        InputValue::Group(group.to_string())
    }

    fn from_field(field: FieldValue<'ast>) -> Self {
        InputValue::Field(field.number.value)
    }

    fn from_implicit(data_type: DataType, implicit: NumberImplicitValue<'ast>) -> Result<Self, InputParserError> {
        match data_type {
            DataType::Boolean(_) => Err(InputParserError::IncompatibleTypes(
                "bool".to_string(),
                "implicit number".to_string(),
            )),
            DataType::Integer(integer_type) => InputValue::from_number(integer_type, implicit.number),
            DataType::Group(_) => Ok(InputValue::Group(implicit.number.value)),
            DataType::Field(_) => Ok(InputValue::Field(implicit.number.value)),
        }
    }

    fn from_value(data_type: DataType, value: Value<'ast>) -> Result<Self, InputParserError> {
        match (data_type, value) {
            (DataType::Boolean(_), Value::Boolean(boolean)) => InputValue::from_boolean(boolean),
            (DataType::Integer(integer_type), Value::Integer(integer)) => {
                InputValue::from_number(integer_type, integer.number)
            }
            (DataType::Group(_), Value::Group(group)) => Ok(InputValue::from_group(group)),
            (DataType::Field(_), Value::Field(field)) => Ok(InputValue::from_field(field)),
            (data_type, Value::Implicit(implicit)) => InputValue::from_implicit(data_type, implicit),
            (data_type, value) => Err(InputParserError::IncompatibleTypes(
                data_type.to_string(),
                value.to_string(),
            )),
        }
    }

    pub(crate) fn from_expression(type_: Type<'ast>, expression: Expression<'ast>) -> Result<Self, InputParserError> {
        match (type_, expression) {
            (Type::Basic(data_type), Expression::Value(value)) => InputValue::from_value(data_type, value),
            (Type::Array(array_type), Expression::ArrayInline(inline)) => {
                InputValue::from_array_inline(array_type, inline)
            }
            (Type::Array(array_type), Expression::ArrayInitializer(initializer)) => {
                InputValue::from_array_initializer(array_type, initializer)
            }
            (Type::Circuit(_), Expression::CircuitInline(_)) => unimplemented!("circuit input values not supported"),
            (Type::Basic(_), Expression::Variable(_)) => unimplemented!("variable input values not supported"),
            (type_, value) => Err(InputParserError::IncompatibleTypes(
                type_.to_string(),
                value.to_string(),
            )),
        }
    }

    pub(crate) fn from_array_inline(
        mut array_type: ArrayType,
        inline: ArrayInlineExpression,
    ) -> Result<Self, InputParserError> {
        match array_type.next_dimension() {
            Some(number) => {
                let outer_dimension = number.value.parse::<usize>()?;

                if outer_dimension != inline.expressions.len() {
                    return Err(InputParserError::InvalidArrayLength(
                        outer_dimension,
                        inline.expressions.len(),
                    ));
                }
            }
            None => return Err(InputParserError::UndefinedArrayDimension),
        }

        let inner_array_type = if array_type.dimensions.len() == 0 {
            // this is a single array
            Type::Basic(array_type._type)
        } else {
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
        let initializer_count = initializer.count.value.parse::<usize>()?;

        match array_type.next_dimension() {
            Some(number) => {
                let outer_dimension = number.value.parse::<usize>()?;

                if outer_dimension != initializer_count {
                    return Err(InputParserError::InvalidArrayLength(outer_dimension, initializer_count));
                }
            }
            None => return Err(InputParserError::UndefinedArrayDimension),
        }

        let inner_array_type = if array_type.dimensions.len() == 0 {
            // this is a single array
            Type::Basic(array_type._type)
        } else {
            Type::Array(array_type)
        };

        let mut values = vec![];
        for _ in 0..initializer_count {
            let value = InputValue::from_expression(inner_array_type.clone(), *initializer.expression.clone())?;

            values.push(value)
        }

        Ok(InputValue::Array(values))
    }

    pub(crate) fn to_input_fields<E: PairingEngine>(&self) -> Result<InputFields<E>, InputParserError> {
        match self {
            InputValue::Boolean(boolean) => Ok(InputFields::from_boolean(boolean)),
            InputValue::Integer(type_, number) => InputFields::from_integer(type_, number),
            InputValue::Group(_) => unimplemented!(),
            InputValue::Field(_) => unimplemented!(),
            InputValue::Array(_) => unimplemented!(),
        }
    }
}

impl fmt::Display for InputValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputValue::Boolean(ref boolean) => write!(f, "{}", boolean),
            InputValue::Integer(ref type_, ref number) => write!(f, "{}{:?}", number, type_),
            InputValue::Group(ref group) => write!(f, "{}", group),
            InputValue::Field(ref field) => write!(f, "{}", field),
            InputValue::Array(ref array) => {
                write!(f, "[")?;
                for (i, e) in array.iter().enumerate() {
                    write!(f, "{}", e)?;
                    if i < array.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
        }
    }
}
