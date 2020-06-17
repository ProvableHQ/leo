use crate::{ast::Rule, errors::SyntaxError as InputSyntaxError};

use crate::{
    expressions::{ArrayInitializerExpression, ArrayInlineExpression, Expression},
    types::{DataType, IntegerType, Type},
    values::{IntegerValue, NumberValue, Value},
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
    fn syntax_error_span<'ast>(message: String, span: Span<'ast>) -> Self {
        let error = Error::new_from_span(ErrorVariant::CustomError { message }, span);

        InputParserError::SyntaxError(InputSyntaxError::from(error))
    }

    pub fn integer_type_mismatch<'ast>(integer_type: IntegerType<'ast>, integer_value: IntegerValue<'ast>) -> Self {
        let message = format!(
            "expected `{}`, found `{}`",
            integer_type.to_string(),
            integer_value.type_.to_string()
        );
        let span = integer_value.type_.span().to_owned();

        InputParserError::syntax_error_span(message, span)
    }

    pub fn data_type_mismatch<'ast>(data_type: DataType<'ast>, value: Value<'ast>) -> Self {
        let message = format!("expected `{}`, found `{}`", data_type.to_string(), value.to_string());
        let span = value.span().to_owned();

        InputParserError::syntax_error_span(message, span)
    }

    pub fn expression_type_mismatch<'ast>(type_: Type<'ast>, expression: Expression<'ast>) -> Self {
        let message = format!("expected `{}`, found `{}`", type_.to_string(), expression.to_string());
        let span = expression.span().to_owned();

        InputParserError::syntax_error_span(message, span)
    }

    pub fn array_inline_length<'ast>(number: NumberValue<'ast>, array: ArrayInlineExpression<'ast>) -> Self {
        let message = format!(
            "expected an array with a fixed size of {} elements, found one with {} elements",
            number.to_string(),
            array.expressions.len()
        );
        let span = array.span.to_owned();

        InputParserError::syntax_error_span(message, span)
    }

    pub fn array_init_length<'ast>(number: NumberValue<'ast>, array: ArrayInitializerExpression<'ast>) -> Self {
        let message = format!(
            "expected an array with a fixed size of {} elements, found one with {} elements",
            number.to_string(),
            array.count
        );
        let span = array.span.to_owned();

        InputParserError::syntax_error_span(message, span)
    }
}

impl From<Error<Rule>> for InputParserError {
    fn from(error: Error<Rule>) -> Self {
        InputParserError::SyntaxError(InputSyntaxError::from(error))
    }
}
