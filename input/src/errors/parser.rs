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

use crate::{
    ast::Rule,
    errors::SyntaxError as InputSyntaxError,
    expressions::{ArrayInlineExpression, Expression},
    sections::Header,
    tables::Table,
    types::{DataType, Type},
    values::{NumberValue, Value},
};

use pest::{
    error::{Error, ErrorVariant},
    Span,
};
use std::{
    num::ParseIntError,
    path::{Path, PathBuf},
    str::ParseBoolError,
};

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
    pub fn set_path(&mut self, path: &Path) {
        if let InputParserError::SyntaxError(error) = self {
            let new_error: Error<Rule> = match error {
                InputSyntaxError::Error(error) => {
                    let new_error = error.clone();
                    new_error.with_path(path.to_str().unwrap())
                }
            };

            tracing::error!("{}", new_error);

            *error = InputSyntaxError::Error(new_error);
        }
    }

    fn new_from_span(message: String, span: Span) -> Self {
        let error = Error::new_from_span(ErrorVariant::CustomError { message }, span);

        InputParserError::SyntaxError(InputSyntaxError::from(error))
    }

    pub fn array_index(actual: String, span: Span) -> Self {
        let message = format!("Expected constant number for array index, found `{}`", actual);

        Self::new_from_span(message, span)
    }

    pub fn implicit_type(data_type: DataType, implicit: NumberValue) -> Self {
        let message = format!("expected `{}`, found `{}`", data_type, implicit);

        Self::new_from_span(message, implicit.span().clone())
    }

    pub fn implicit_group(number: NumberValue) -> Self {
        let message = format!("group coordinates should be in (x, y)group format, found `{}`", number);

        Self::new_from_span(message, number.span().clone())
    }

    pub fn data_type_mismatch(data_type: DataType, value: Value) -> Self {
        let message = format!("expected data type `{}`, found `{}`", data_type, value);
        let span = value.span().to_owned();

        Self::new_from_span(message, span)
    }

    pub fn expression_type_mismatch(type_: Type, expression: Expression) -> Self {
        let message = format!("expected expression type `{}`, found `{}`", type_, expression);
        let span = expression.span().to_owned();

        Self::new_from_span(message, span)
    }

    pub fn array_inline_length(number: usize, array: ArrayInlineExpression) -> Self {
        let message = format!(
            "expected an array with a fixed size of {} elements, found one with {} elements",
            number,
            array.expressions.len()
        );
        let span = array.span.to_owned();

        Self::new_from_span(message, span)
    }

    pub fn array_init_length(expected: Vec<usize>, actual: Vec<usize>, span: Span) -> Self {
        let message = format!(
            "expected an array with a fixed size of {:?} elements, found one with {:?} elements",
            expected, actual
        );

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
