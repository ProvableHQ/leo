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

//! This module defines a statement node in an asg.
//!
//! Ast statement nodes can be directly converted into asg nodes with no major differences.

mod assign;
pub use assign::*;

mod block;
pub use block::*;

mod conditional;
pub use conditional::*;

mod console;
pub use console::*;

mod definition;
pub use definition::*;

mod expression;
pub use expression::*;

mod iteration;
pub use iteration::*;

mod return_;
pub use return_::*;

use crate::{FromAst, Node, PartialType, Scope};
use leo_errors::{LeoError, Span};

#[derive(Clone)]
pub enum Statement<'a> {
    Return(ReturnStatement<'a>),
    Definition(DefinitionStatement<'a>),
    Assign(AssignStatement<'a>),
    Conditional(ConditionalStatement<'a>),
    Iteration(IterationStatement<'a>),
    Console(ConsoleStatement<'a>),
    Expression(ExpressionStatement<'a>),
    Block(BlockStatement<'a>),
    Empty(Option<Span>),
}

impl<'a> Node for Statement<'a> {
    fn span(&self) -> Option<&Span> {
        use Statement::*;
        match self {
            Return(s) => s.span(),
            Definition(s) => s.span(),
            Assign(s) => s.span(),
            Conditional(s) => s.span(),
            Iteration(s) => s.span(),
            Console(s) => s.span(),
            Expression(s) => s.span(),
            Block(s) => s.span(),
            Empty(s) => s.as_ref(),
        }
    }
}

impl<'a> FromAst<'a, leo_ast::Statement> for &'a Statement<'a> {
    fn from_ast(
        scope: &'a Scope<'a>,
        value: &leo_ast::Statement,
        _expected_type: Option<PartialType<'a>>,
    ) -> Result<&'a Statement<'a>, LeoError> {
        use leo_ast::Statement::*;
        Ok(match value {
            Return(statement) => scope
                .context
                .alloc_statement(Statement::Return(ReturnStatement::from_ast(scope, statement, None)?)),
            Definition(statement) => Self::from_ast(scope, statement, None)?,
            Assign(statement) => Self::from_ast(scope, &**statement, None)?,
            Conditional(statement) => {
                scope
                    .context
                    .alloc_statement(Statement::Conditional(ConditionalStatement::from_ast(
                        scope, statement, None,
                    )?))
            }
            Iteration(statement) => Self::from_ast(scope, &**statement, None)?,
            Console(statement) => scope
                .context
                .alloc_statement(Statement::Console(ConsoleStatement::from_ast(scope, statement, None)?)),
            Expression(statement) => {
                scope
                    .context
                    .alloc_statement(Statement::Expression(ExpressionStatement::from_ast(
                        scope, statement, None,
                    )?))
            }
            Block(statement) => scope
                .context
                .alloc_statement(Statement::Block(BlockStatement::from_ast(scope, statement, None)?)),
        })
    }
}

impl<'a> Into<leo_ast::Statement> for &Statement<'a> {
    fn into(self) -> leo_ast::Statement {
        use Statement::*;
        match self {
            Return(statement) => leo_ast::Statement::Return(statement.into()),
            Definition(statement) => leo_ast::Statement::Definition(statement.into()),
            Assign(statement) => leo_ast::Statement::Assign(Box::new(statement.into())),
            Conditional(statement) => leo_ast::Statement::Conditional(statement.into()),
            Iteration(statement) => leo_ast::Statement::Iteration(Box::new(statement.into())),
            Console(statement) => leo_ast::Statement::Console(statement.into()),
            Expression(statement) => leo_ast::Statement::Expression(statement.into()),
            Block(statement) => leo_ast::Statement::Block(statement.into()),
            Empty(_) => unimplemented!(),
        }
    }
}
