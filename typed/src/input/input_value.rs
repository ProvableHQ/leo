use leo_input::{
    errors::InputParserError,
    expressions::{ArrayInitializerExpression, ArrayInlineExpression, Expression},
    types::{ArrayType, DataType, IntegerType, Type},
    values::{BooleanValue, FieldValue, GroupValue, NumberValue, Value},
};

use leo_input::{types::TupleType, values::Address};
use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub enum InputValue {
    Address(String),
    Boolean(bool),
    Field(String),
    Group(String),
    Integer(IntegerType, String),
    Array(Vec<InputValue>),
    Tuple(Vec<InputValue>),
}

impl InputValue {
    fn from_address(address: Address) -> Self {
        InputValue::Address(address.value)
    }

    fn from_boolean(boolean: BooleanValue) -> Result<Self, InputParserError> {
        let boolean = boolean.value.parse::<bool>()?;
        Ok(InputValue::Boolean(boolean))
    }

    fn from_number(integer_type: IntegerType, number: String) -> Result<Self, InputParserError> {
        Ok(InputValue::Integer(integer_type, number))
    }

    fn from_group(group: GroupValue) -> Self {
        InputValue::Group(group.to_string())
    }

    fn from_field(field: FieldValue) -> Self {
        InputValue::Field(field.number.to_string())
    }

    fn from_implicit(data_type: DataType, implicit: NumberValue) -> Result<Self, InputParserError> {
        match data_type {
            DataType::Address(_) => Err(InputParserError::implicit_type(data_type, implicit)),
            DataType::Boolean(_) => Err(InputParserError::implicit_type(data_type, implicit)),
            DataType::Integer(integer_type) => InputValue::from_number(integer_type, implicit.to_string()),
            DataType::Group(_) => Ok(InputValue::Group(implicit.to_string())),
            DataType::Field(_) => Ok(InputValue::Field(implicit.to_string())),
        }
    }

    fn from_value(data_type: DataType, value: Value) -> Result<Self, InputParserError> {
        match (data_type, value) {
            (DataType::Address(_), Value::Address(address)) => Ok(InputValue::from_address(address.address)),
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
        let initializer_count = initializer.count.to_string().parse::<usize>()?;

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

    pub(crate) fn from_tuple(tuple_type: TupleType, tuple: Vec<Expression>) -> Result<Self, InputParserError> {
        let num_types = tuple_type.types_.len();
        let num_values = tuple.len();

        if num_types != num_values {
            return Err(InputParserError::tuple_length(
                num_types,
                num_values,
                tuple_type.span.clone(),
            ));
        }

        let mut values = vec![];
        for (type_, value) in tuple_type.types_.into_iter().zip(tuple.into_iter()) {
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
