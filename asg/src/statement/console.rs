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

use crate::{AsgConvertError, Expression, FromAst, Node, PartialType, Scope, Span, Statement, Type};
use leo_ast::ConsoleFunction as AstConsoleFunction;
use std::sync::{Arc, Weak};

//todo: refactor to not require/depend on span
pub struct FormattedString {
    pub string: String,
    pub containers: Vec<Span>,
    pub parameters: Vec<Arc<Expression>>,
    pub span: Span,
}

pub enum ConsoleFunction {
    Assert(Arc<Expression>),
    Debug(FormattedString),
    Error(FormattedString),
    Log(FormattedString),
}

pub struct ConsoleStatement {
    pub parent: Option<Weak<Statement>>,
    pub span: Option<Span>,
    pub function: ConsoleFunction,
}

impl Node for ConsoleStatement {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl FromAst<leo_ast::FormattedString> for FormattedString {
    fn from_ast(
        scope: &Scope,
        value: &leo_ast::FormattedString,
        _expected_type: Option<PartialType>,
    ) -> Result<Self, AsgConvertError> {
        if value.parameters.len() != value.containers.len() {
            // + 1 for formatting string as to not confuse user
            return Err(AsgConvertError::unexpected_call_argument_count(
                value.containers.len() + 1,
                value.parameters.len() + 1,
                &value.span,
            ));
        }
        let mut parameters = vec![];
        for parameter in value.parameters.iter() {
            parameters.push(Arc::<Expression>::from_ast(scope, parameter, None)?);
        }
        Ok(FormattedString {
            string: value.string.clone(),
            containers: value.containers.iter().map(|x| x.span.clone()).collect(),
            parameters,
            span: value.span.clone(),
        })
    }
}

impl Into<leo_ast::FormattedString> for &FormattedString {
    fn into(self) -> leo_ast::FormattedString {
        leo_ast::FormattedString {
            string: self.string.clone(),
            containers: self
                .containers
                .iter()
                .map(|span| leo_ast::FormattedContainer { span: span.clone() })
                .collect(),
            parameters: self.parameters.iter().map(|e| e.as_ref().into()).collect(),
            span: self.span.clone(),
        }
    }
}

impl FromAst<leo_ast::ConsoleStatement> for ConsoleStatement {
    fn from_ast(
        scope: &Scope,
        statement: &leo_ast::ConsoleStatement,
        _expected_type: Option<PartialType>,
    ) -> Result<Self, AsgConvertError> {
        Ok(ConsoleStatement {
            parent: None,
            span: Some(statement.span.clone()),
            function: match &statement.function {
                AstConsoleFunction::Assert(expression) => ConsoleFunction::Assert(Arc::<Expression>::from_ast(
                    scope,
                    expression,
                    Some(Type::Boolean.into()),
                )?),
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

impl Into<leo_ast::ConsoleStatement> for &ConsoleStatement {
    fn into(self) -> leo_ast::ConsoleStatement {
        use ConsoleFunction::*;
        leo_ast::ConsoleStatement {
            function: match &self.function {
                Assert(e) => AstConsoleFunction::Assert(e.as_ref().into()),
                Debug(formatted_string) => AstConsoleFunction::Debug(formatted_string.into()),
                Error(formatted_string) => AstConsoleFunction::Error(formatted_string.into()),
                Log(formatted_string) => AstConsoleFunction::Log(formatted_string.into()),
            },
            span: self.span.clone().unwrap_or_default(),
        }
    }
}
