use leo_inputs::{
    errors::InputParserError,
    expressions::Expression,
    types::{DataType, Type},
    values::{BooleanValue, Value},
};
use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub enum InputValue {
    Integer(u128),
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

    fn from_value(data_type: DataType, value: Value<'ast>) -> Result<Self, InputParserError> {
        match (data_type, value) {
            (DataType::Boolean(_), Value::Boolean(value)) => InputValue::from_boolean(value),
            (DataType::Integer(_), Value::Integer(_)) => unimplemented!(),
            (DataType::Group(_), Value::Group(_)) => unimplemented!(),
            (DataType::Field(_), Value::Field(_)) => unimplemented!(),
            (data_type, Value::Implicit(_)) => unimplemented!(), //do something with data type
            (_, _) => unimplemented!("incompatible types"),
        }
    }

    pub(crate) fn from_expression(type_: Type<'ast>, expression: Expression<'ast>) -> Result<Self, InputParserError> {
        // evaluate expression
        match (type_, expression) {
            (Type::Basic(_), Expression::Variable(ref variable)) => unimplemented!("variable inputs not supported"),
            (Type::Basic(data_type), Expression::Value(value)) => InputValue::from_value(data_type, value),
            (Type::Array(array_type), Expression::ArrayInline(_)) => unimplemented!(),
            (Type::Array(array_type), Expression::ArrayInitializer(_)) => unimplemented!(),
            (Type::Circuit(circuit_type), Expression::CircuitInline(_)) => unimplemented!(),
            (_, _) => unimplemented!("incompatible types"),
        }

        // check type
    }
}

impl fmt::Display for InputValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputValue::Integer(ref integer) => write!(f, "{}", integer),
            InputValue::Field(ref field) => write!(f, "{}", field),
            InputValue::Group(ref group) => write!(f, "{}", group),
            InputValue::Boolean(ref bool) => write!(f, "{}", bool),
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
