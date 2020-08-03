use leo_input::{
    errors::InputParserError,
    expressions::{ArrayInitializerExpression, ArrayInlineExpression, Expression},
    types::{ArrayType, DataType, IntegerType, Type},
    values::{BooleanValue, FieldValue, GroupValue, NumberImplicitValue, NumberValue, Value},
};

use leo_input::values::Address;
use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub enum InputValue {
    Address(String),
    Boolean(bool),
    Field(String),
    Group(String),
    Integer(IntegerType, String),
    Array(Vec<InputValue>),
}

impl InputValue {
    fn from_address(address: Address) -> Self {
        InputValue::Address(address.value)
    }

    fn from_boolean(boolean: BooleanValue) -> Result<Self, InputParserError> {
        let boolean = boolean.value.parse::<bool>()?;
        Ok(InputValue::Boolean(boolean))
    }

    fn from_number(integer_type: IntegerType, number: NumberValue) -> Result<Self, InputParserError> {
        Ok(InputValue::Integer(integer_type, number.value))
    }

    fn from_group(group: GroupValue) -> Self {
        InputValue::Group(group.to_string())
    }

    fn from_field(field: FieldValue) -> Self {
        InputValue::Field(field.number.value)
    }

    fn from_implicit(data_type: DataType, implicit: NumberImplicitValue) -> Result<Self, InputParserError> {
        match data_type {
            DataType::Address(_) => Err(InputParserError::implicit_type(data_type, implicit)),
            DataType::Boolean(_) => Err(InputParserError::implicit_type(data_type, implicit)),
            DataType::Integer(integer_type) => InputValue::from_number(integer_type, implicit.number),
            DataType::Group(_) => Ok(InputValue::Group(implicit.number.value)),
            DataType::Field(_) => Ok(InputValue::Field(implicit.number.value)),
        }
    }

    fn from_value(data_type: DataType, value: Value) -> Result<Self, InputParserError> {
        match (data_type, value) {
            (DataType::Address(_), Value::Address(address)) => Ok(InputValue::from_address(address.address)),
            (DataType::Boolean(_), Value::Boolean(boolean)) => InputValue::from_boolean(boolean),
            (DataType::Integer(integer_type), Value::Integer(integer)) => {
                InputValue::from_number(integer_type, integer.number)
            }
            (DataType::Group(_), Value::Group(group)) => Ok(InputValue::from_group(group)),
            (DataType::Field(_), Value::Field(field)) => Ok(InputValue::from_field(field)),
            (data_type, Value::Implicit(implicit)) => InputValue::from_implicit(data_type, implicit),
            (data_type, value) => Err(InputParserError::data_type_mismatch(data_type, value)),
        }
    }

    pub(crate) fn from_expression(type_: Type, expression: Expression) -> Result<Self, InputParserError> {
        match (type_, expression) {
            (Type::Basic(DataType::Address(_)), Expression::ImplicitAddress(address)) => {
                Ok(InputValue::from_address(address))
            }
            (Type::Basic(data_type), Expression::Value(value)) => InputValue::from_value(data_type, value),
            (Type::Array(array_type), Expression::ArrayInline(inline)) => {
                InputValue::from_array_inline(array_type, inline)
            }
            (Type::Array(array_type), Expression::ArrayInitializer(initializer)) => {
                InputValue::from_array_initializer(array_type, initializer)
            }
            (type_, expression) => Err(InputParserError::expression_type_mismatch(type_, expression)),
        }
    }

    pub(crate) fn from_array_inline(
        mut array_type: ArrayType,
        inline: ArrayInlineExpression,
    ) -> Result<Self, InputParserError> {
        if let Some(number) = array_type.next_dimension() {
            let outer_dimension = number.value.parse::<usize>()?;

            if outer_dimension != inline.expressions.len() {
                return Err(InputParserError::array_inline_length(number, inline));
            }
        };

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

        if let Some(number) = array_type.next_dimension() {
            let outer_dimension = number.value.parse::<usize>()?;

            if outer_dimension != initializer_count {
                return Err(InputParserError::array_init_length(number, initializer));
            }
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
