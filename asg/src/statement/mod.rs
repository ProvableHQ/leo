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

mod return_;
pub use return_::*;
mod definition;
pub use definition::*;
mod assign;
pub use assign::*;
mod conditional;
pub use conditional::*;
mod iteration;
pub use iteration::*;
mod console;
pub use console::*;
mod expression;
pub use expression::*;
mod block;
pub use block::*;

use crate::{AsgConvertError, FromAst, Node, PartialType, Scope, Span};
use std::sync::Arc;

pub enum Statement {
    Return(ReturnStatement),
    Definition(DefinitionStatement),
    Assign(AssignStatement),
    Conditional(ConditionalStatement),
    Iteration(IterationStatement),
    Console(ConsoleStatement),
    Expression(ExpressionStatement),
    Block(BlockStatement),
}

impl Node for Statement {
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
        }
    }
}

impl FromAst<leo_ast::Statement> for Arc<Statement> {
    fn from_ast(
        scope: &Scope,
        value: &leo_ast::Statement,
        _expected_type: Option<PartialType>,
    ) -> Result<Arc<Statement>, AsgConvertError> {
        use leo_ast::Statement::*;
        Ok(match value {
            Return(statement) => Arc::new(Statement::Return(ReturnStatement::from_ast(scope, statement, None)?)),
            Definition(statement) => Arc::<Statement>::from_ast(scope, statement, None)?,
            Assign(statement) => Arc::<Statement>::from_ast(scope, statement, None)?,
            Conditional(statement) => Arc::new(Statement::Conditional(ConditionalStatement::from_ast(
                scope, statement, None,
            )?)),
            Iteration(statement) => Arc::<Statement>::from_ast(scope, statement, None)?,
            Console(statement) => Arc::new(Statement::Console(ConsoleStatement::from_ast(scope, statement, None)?)),
            Expression(statement) => Arc::new(Statement::Expression(ExpressionStatement::from_ast(
                scope, statement, None,
            )?)),
            Block(statement) => Arc::new(Statement::Block(BlockStatement::from_ast(scope, statement, None)?)),
        })
    }
}

impl Into<leo_ast::Statement> for &Statement {
    fn into(self) -> leo_ast::Statement {
        use Statement::*;
        match self {
            Return(statement) => leo_ast::Statement::Return(statement.into()),
            Definition(statement) => leo_ast::Statement::Definition(statement.into()),
            Assign(statement) => leo_ast::Statement::Assign(statement.into()),
            Conditional(statement) => leo_ast::Statement::Conditional(statement.into()),
            Iteration(statement) => leo_ast::Statement::Iteration(statement.into()),
            Console(statement) => leo_ast::Statement::Console(statement.into()),
            Expression(statement) => leo_ast::Statement::Expression(statement.into()),
            Block(statement) => leo_ast::Statement::Block(statement.into()),
        }
    }
}
