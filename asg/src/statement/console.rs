// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use crate::{AsgConvertError, Expression, FromAst, Node, PartialType, Scope, Span, Statement, Type};
use leo_ast::{ConsoleFunction as AstConsoleFunction, FormattedStringPart};

use std::cell::Cell;

// TODO (protryon): Refactor to not require/depend on span
#[derive(Clone)]
pub struct FormattedString<'a> {
    pub parts: Vec<FormattedStringPart>,
    pub parameters: Vec<Cell<&'a Expression<'a>>>,
    pub span: Span,
}

#[derive(Clone)]
pub enum ConsoleFunction<'a> {
    Assert(Cell<&'a Expression<'a>>),
    Debug(FormattedString<'a>),
    Error(FormattedString<'a>),
    Log(FormattedString<'a>),
}

#[derive(Clone)]
pub struct ConsoleStatement<'a> {
    pub parent: Cell<Option<&'a Statement<'a>>>,
    pub span: Option<Span>,
    pub function: ConsoleFunction<'a>,
}

impl<'a> Node for ConsoleStatement<'a> {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl<'a> FromAst<'a, leo_ast::FormattedString> for FormattedString<'a> {
    fn from_ast(
        scope: &'a Scope<'a>,
        value: &leo_ast::FormattedString,
        _expected_type: Option<PartialType<'a>>,
    ) -> Result<Self, AsgConvertError> {
        let expected_param_len = value
            .parts
            .iter()
            .filter(|x| matches!(x, FormattedStringPart::Container))
            .count();
        if value.parameters.len() != expected_param_len {
            // + 1 for formatting string as to not confuse user
            return Err(AsgConvertError::unexpected_call_argument_count(
                expected_param_len + 1,
                value.parameters.len() + 1,
                &value.span,
            ));
        }
        let mut parameters = vec![];
        for parameter in value.parameters.iter() {
            parameters.push(Cell::new(<&Expression<'a>>::from_ast(scope, parameter, None)?));
        }
        Ok(FormattedString {
            parts: value.parts.clone(),
            parameters,
            span: value.span.clone(),
        })
    }
}

impl<'a> Into<leo_ast::FormattedString> for &FormattedString<'a> {
    fn into(self) -> leo_ast::FormattedString {
        leo_ast::FormattedString {
            parts: self.parts.clone(),
            parameters: self.parameters.iter().map(|e| e.get().into()).collect(),
            span: self.span.clone(),
        }
    }
}

impl<'a> FromAst<'a, leo_ast::ConsoleStatement> for ConsoleStatement<'a> {
    fn from_ast(
        scope: &'a Scope<'a>,
        statement: &leo_ast::ConsoleStatement,
        _expected_type: Option<PartialType<'a>>,
    ) -> Result<Self, AsgConvertError> {
        Ok(ConsoleStatement {
            parent: Cell::new(None),
            span: Some(statement.span.clone()),
            function: match &statement.function {
                AstConsoleFunction::Assert(expression) => ConsoleFunction::Assert(Cell::new(
                    <&Expression<'a>>::from_ast(scope, expression, Some(Type::Boolean.into()))?,
                )),
                AstConsoleFunction::Debug(formatted_string) => {
                    ConsoleFunction::Debug(FormattedString::from_ast(scope, formatted_string, None)?)
                }
                AstConsoleFunction::Error(formatted_string) => {
                    ConsoleFunction::Error(FormattedString::from_ast(scope, formatted_string, None)?)
                }
                AstConsoleFunction::Log(formatted_string) => {
                    ConsoleFunction::Log(FormattedString::from_ast(scope, formatted_string, None)?)
                }
            },
        })
    }
}

impl<'a> Into<leo_ast::ConsoleStatement> for &ConsoleStatement<'a> {
    fn into(self) -> leo_ast::ConsoleStatement {
        use ConsoleFunction::*;
        leo_ast::ConsoleStatement {
            function: match &self.function {
                Assert(e) => AstConsoleFunction::Assert(e.get().into()),
                Debug(formatted_string) => AstConsoleFunction::Debug(formatted_string.into()),
                Error(formatted_string) => AstConsoleFunction::Error(formatted_string.into()),
                Log(formatted_string) => AstConsoleFunction::Log(formatted_string.into()),
            },
            span: self.span.clone().unwrap_or_default(),
        }
    }
}
