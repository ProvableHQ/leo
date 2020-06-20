use crate::{
    ast::Rule,
    errors::SyntaxError as InputSyntaxError,
    expressions::{ArrayInitializerExpression, ArrayInlineExpression, Expression},
    types::{DataType, Type},
    values::{NumberImplicitValue, NumberValue, Value},
};

use pest::{
    error::{Error, ErrorVariant},
    Span,
};
use std::{num::ParseIntError, path::PathBuf, str::ParseBoolError};

#[derive(Debug, Error)]
pub enum InputParserError {
    #[error("Program input value {} not found", _0)]
    InputNotFound(String),

    #[error("Cannot read from the provided file path - {:?}", _0)]
    FileReadError(PathBuf),

    #[error("{}", _0)]
    ParseIntError(#[from] ParseIntError),

    #[error("{}", _0)]
    ParseBoolError(#[from] ParseBoolError),

    #[error("{}", _0)]
    SyntaxError(#[from] InputSyntaxError),

    #[error("Unable to construct abstract syntax tree")]
    SyntaxTreeError,
}

impl InputParserError {
    fn new_from_span(message: String, span: Span) -> Self {
        let error = Error::new_from_span(ErrorVariant::CustomError { message }, span);

        InputParserError::SyntaxError(InputSyntaxError::from(error))
    }

    pub fn implicit_boolean(data_type: DataType, implicit: NumberImplicitValue) -> Self {
        let message = format!("expected `{}`, found `{}`", data_type.to_string(), implicit.to_string());

        Self::new_from_span(message, implicit.span)
    }

    pub fn data_type_mismatch(data_type: DataType, value: Value) -> Self {
        let message = format!("expected `{}`, found `{}`", data_type.to_string(), value.to_string());
        let span = value.span().to_owned();

        Self::new_from_span(message, span)
    }

    pub fn expression_type_mismatch(type_: Type, expression: Expression) -> Self {
        let message = format!("expected `{}`, found `{}`", type_.to_string(), expression.to_string());
        let span = expression.span().to_owned();

        Self::new_from_span(message, span)
    }

    pub fn array_inline_length(number: NumberValue, array: ArrayInlineExpression) -> Self {
        let message = format!(
            "expected an array with a fixed size of {} elements, found one with {} elements",
            number.to_string(),
            array.expressions.len()
        );
        let span = array.span.to_owned();

        Self::new_from_span(message, span)
    }

    pub fn array_init_length(number: NumberValue, array: ArrayInitializerExpression) -> Self {
        let message = format!(
            "expected an array with a fixed size of {} elements, found one with {} elements",
            number.to_string(),
            array.count
        );
        let span = array.span.to_owned();

        Self::new_from_span(message, span)
    }
}

impl From<Error<Rule>> for InputParserError {
    fn from(error: Error<Rule>) -> Self {
        InputParserError::SyntaxError(InputSyntaxError::from(error))
    }
}
