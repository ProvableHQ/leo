use crate::{
    ast::Rule,
    errors::SyntaxError as InputSyntaxError,
    expressions::{ArrayInitializerExpression, ArrayInlineExpression, Expression},
    sections::Header,
    tables::Table,
    types::{DataType, Type},
    values::{NumberValue, PositiveNumber, Value},
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

    #[error("Unable to construct program input abstract syntax tree")]
    SyntaxTreeError,
}

impl InputParserError {
    fn new_from_span(message: String, span: Span) -> Self {
        let error = Error::new_from_span(ErrorVariant::CustomError { message }, span);

        InputParserError::SyntaxError(InputSyntaxError::from(error))
    }

    pub fn implicit_type(data_type: DataType, implicit: NumberValue) -> Self {
        let message = format!("expected `{}`, found `{}`", data_type.to_string(), implicit.to_string());

        Self::new_from_span(message, implicit.span().clone())
    }

    pub fn implicit_group(number: NumberValue) -> Self {
        let message = format!("group coordinates should be in (x, y)group format, found `{}`", number);

        Self::new_from_span(message, number.span().clone())
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

    pub fn array_inline_length(number: PositiveNumber, array: ArrayInlineExpression) -> Self {
        let message = format!(
            "expected an array with a fixed size of {} elements, found one with {} elements",
            number.to_string(),
            array.expressions.len()
        );
        let span = array.span.to_owned();

        Self::new_from_span(message, span)
    }

    pub fn array_init_length(number: PositiveNumber, array: ArrayInitializerExpression) -> Self {
        let message = format!(
            "expected an array with a fixed size of {} elements, found one with {} elements",
            number.to_string(),
            array.count
        );
        let span = array.span.to_owned();

        Self::new_from_span(message, span)
    }

    pub fn input_section_header(header: Header) -> Self {
        let message = format!("the section header `{}` is not valid in an input `.in` file", header);
        let span = header.span();

        Self::new_from_span(message, span)
    }

    pub fn public_section(header: Header) -> Self {
        let message = format!("the section header `{}` is not a public section", header);
        let span = header.span();

        Self::new_from_span(message, span)
    }

    pub fn private_section(header: Header) -> Self {
        let message = format!("the section header `{}` is not a private section", header);
        let span = header.span();

        Self::new_from_span(message, span)
    }

    pub fn table(table: Table) -> Self {
        let message = format!(
            "the double bracket section `{}` is not valid in an input `.in` file",
            table
        );

        Self::new_from_span(message, table.span)
    }

    pub fn tuple_length(expected: usize, actual: usize, span: Span) -> Self {
        let message = format!(
            "expected a tuple with {} elements, found a tuple with {} elements",
            expected, actual
        );

        Self::new_from_span(message, span)
    }

    pub fn section(header: Header) -> Self {
        let message = format!(
            "the section header `{}` must have a double bracket visibility in a state `.state` file",
            header
        );
        let span = header.span();

        Self::new_from_span(message, span)
    }
}

impl From<Error<Rule>> for InputParserError {
    fn from(error: Error<Rule>) -> Self {
        InputParserError::SyntaxError(InputSyntaxError::from(error))
    }
}
