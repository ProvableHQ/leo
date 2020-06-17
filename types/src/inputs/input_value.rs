use leo_inputs::{
    errors::InputParserError,
    expressions::{ArrayInitializerExpression, ArrayInlineExpression, Expression},
    types::{ArrayType, DataType, IntegerType, Type},
    values::{BooleanValue, FieldValue, GroupValue, NumberImplicitValue, Value},
};

use leo_inputs::values::IntegerValue;
use pest::Span;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InputValue<'ast> {
    Integer(IntegerValue<'ast>),
    Field(FieldValue<'ast>),
    Group(GroupValue<'ast>),
    Boolean(BooleanValue<'ast>),
    Array(Vec<InputValue<'ast>>),
}

impl<'ast> InputValue<'ast> {
    pub fn span(&self) -> &Span<'ast> {
        match self {
            InputValue::Integer(value) => &value.span,
            InputValue::Field(value) => &value.span,
            InputValue::Group(value) => &value.span,
            InputValue::Boolean(value) => &value.span,
            InputValue::Array(_) => unimplemented!(), // create new span from start and end
        }
    }

    fn from_typed_integer(
        integer_type: IntegerType<'ast>,
        integer: IntegerValue<'ast>,
    ) -> Result<Self, InputParserError> {
        if integer_type != integer.type_ {
            return Err(InputParserError::integer_type_mismatch(integer_type, integer));
        }

        Ok(InputValue::Integer(integer))
    }

    fn from_implicit(data_type: DataType<'ast>, number: NumberImplicitValue<'ast>) -> Result<Self, InputParserError> {
        match data_type {
            DataType::Integer(type_) => Ok(InputValue::Integer(IntegerValue::from_implicit(number, type_))),
            DataType::Field(type_) => Ok(InputValue::Field(FieldValue::from_implicit(number, type_))),
            DataType::Boolean(_) => unimplemented!("cannot have an implicit boolean"),
            DataType::Group(_) => unimplemented!("group inputs must be explicitly typed"),
        }
    }

    fn from_value(data_type: DataType<'ast>, value: Value<'ast>) -> Result<Self, InputParserError> {
        match (data_type, value) {
            (DataType::Boolean(_), Value::Boolean(boolean)) => Ok(InputValue::Boolean(boolean)),
            (DataType::Integer(integer_type), Value::Integer(integer)) => {
                Self::from_typed_integer(integer_type, integer)
            }
            (DataType::Group(_), Value::Group(group)) => Ok(InputValue::Group(group)),
            (DataType::Field(_), Value::Field(field)) => Ok(InputValue::Field(field)),
            (data_type, Value::Implicit(number)) => InputValue::from_implicit(data_type, number),
            (data_type, value) => Err(InputParserError::data_type_mismatch(data_type, value)),
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
            (type_, expression) => Err(InputParserError::expression_type_mismatch(type_, expression)),
        }
    }

    pub(crate) fn from_array_inline(
        mut array_type: ArrayType<'ast>,
        inline: ArrayInlineExpression<'ast>,
    ) -> Result<Self, InputParserError> {
        if let Some(number) = array_type.next_dimension() {
            let outer_dimension = number.value.parse::<usize>()?;
            if outer_dimension != inline.expressions.len() {
                return Err(InputParserError::array_inline_length(number, inline));
            }
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
        mut array_type: ArrayType<'ast>,
        initializer: ArrayInitializerExpression<'ast>,
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

impl<'ast> fmt::Display for InputValue<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputValue::Boolean(ref boolean) => write!(f, "{}", boolean),
            InputValue::Integer(ref number) => write!(f, "{}", number),
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
