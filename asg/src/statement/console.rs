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

use crate::{AsgId, CharValue, Expression, FromAst, Node, PartialType, Scope, Statement, Type};
use leo_ast::ConsoleFunction as AstConsoleFunction;
use leo_errors::{Result, Span};

use std::cell::Cell;

// TODO (protryon): Refactor to not require/depend on span
#[derive(Clone)]
pub struct ConsoleArgs<'a> {
    pub id: AsgId,
    pub string: Vec<CharValue>,
    pub parameters: Vec<Cell<&'a Expression<'a>>>,
    pub span: Span,
}

impl<'a> Node for ConsoleArgs<'a> {
    fn span(&self) -> Option<&Span> {
        Some(&self.span)
    }

    fn get_id(&self) -> AsgId {
        self.id
    }
}

#[derive(Clone)]
pub enum ConsoleFunction<'a> {
    Assert(Cell<&'a Expression<'a>>),
    Error(ConsoleArgs<'a>),
    Log(ConsoleArgs<'a>),
}

#[derive(Clone)]
pub struct ConsoleStatement<'a> {
    pub id: AsgId,
    pub parent: Cell<Option<&'a Statement<'a>>>,
    pub span: Option<Span>,
    pub function: ConsoleFunction<'a>,
}

impl<'a> Node for ConsoleStatement<'a> {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }

    fn get_id(&self) -> AsgId {
        self.id
    }
}

impl<'a> FromAst<'a, leo_ast::ConsoleArgs> for ConsoleArgs<'a> {
    fn from_ast(
        scope: &'a Scope<'a>,
        value: &leo_ast::ConsoleArgs,
        _expected_type: Option<PartialType<'a>>,
    ) -> Result<Self> {
        let mut parameters = vec![];
        for parameter in value.parameters.iter() {
            parameters.push(Cell::new(<&Expression<'a>>::from_ast(scope, parameter, None)?));
        }
        Ok(ConsoleArgs {
            id: scope.context.get_id(),
            string: value.string.iter().map(CharValue::from).collect::<Vec<_>>(),
            parameters,
            span: value.span.clone(),
        })
    }
}

impl<'a> Into<leo_ast::ConsoleArgs> for &ConsoleArgs<'a> {
    fn into(self) -> leo_ast::ConsoleArgs {
        leo_ast::ConsoleArgs {
            string: self.string.iter().map(|c| c.into()).collect::<Vec<_>>(),
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
    ) -> Result<Self> {
        Ok(ConsoleStatement {
            id: scope.context.get_id(),
            parent: Cell::new(None),
            span: Some(statement.span.clone()),
            function: match &statement.function {
                AstConsoleFunction::Assert(expression) => ConsoleFunction::Assert(Cell::new(
                    <&Expression<'a>>::from_ast(scope, expression, Some(Type::Boolean.into()))?,
                )),
                AstConsoleFunction::Error(args) => ConsoleFunction::Error(ConsoleArgs::from_ast(scope, args, None)?),
                AstConsoleFunction::Log(args) => ConsoleFunction::Log(ConsoleArgs::from_ast(scope, args, None)?),
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
                Error(args) => AstConsoleFunction::Error(args.into()),
                Log(args) => AstConsoleFunction::Log(args.into()),
            },
            span: self.span.clone().unwrap_or_default(),
        }
    }
}
