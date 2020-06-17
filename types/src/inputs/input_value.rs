use leo_inputs::{
    errors::InputParserError,
    expressions::{ArrayInitializerExpression, ArrayInlineExpression, Expression},
    types::{ArrayType, DataType, IntegerType, Type},
    values::{BooleanValue, FieldValue, GroupValue, NumberImplicitValue, NumberValue, Value},
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
    Unresolved(DataType, NumberImplicitValue<'ast>),
}

impl<'ast> InputValue<'ast> {
    pub fn span(&self) -> &Span<'ast> {
        match self {
            InputValue::Integer(value) => &value.span,
            InputValue::Field(value) => &value.span,
            InputValue::Group(value) => &value.span,
            InputValue::Boolean(value) => &value.span,
            InputValue::Array(_) => unimplemented!(), // create new span from start and end
            InputValue::Unresolved(_, _) => unimplemented!(),
        }
    }

    fn from_value(data_type: DataType, value: Value<'ast>) -> Result<Self, InputParserError> {
        match (data_type, value) {
            (DataType::Boolean(_), Value::Boolean(boolean)) => Ok(InputValue::Boolean(boolean)),
            (DataType::Integer(integer_type), Value::Integer(integer)) => Ok(InputValue::Integer(integer)),
            (DataType::Group(_), Value::Group(group)) => Ok(InputValue::Group(group)),
            (DataType::Field(_), Value::Field(field)) => Ok(InputValue::Field(field)),
            (data_type, Value::Implicit(implicit)) => Ok(InputValue::Unresolved(data_type, implicit)),
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
            (type_, value) => Err(InputParserError::IncompatibleTypes(
                type_.to_string(),
                value.to_string(),
            )),
        }
    }

    pub(crate) fn from_array_inline(
        mut array_type: ArrayType<'ast>,
        inline: ArrayInlineExpression<'ast>,
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
        mut array_type: ArrayType<'ast>,
        initializer: ArrayInitializerExpression<'ast>,
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
            InputValue::Unresolved(ref type_, ref number) => write!(f, "{}{}", number, type_),
        }
    }
}
