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

use crate::{ast::Rule, console::ConsoleFunctionCall, statements::*};

use pest_ast::FromPest;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::statement))]
pub enum Statement<'ast> {
    Return(ReturnStatement<'ast>),
    Definition(DefinitionStatement<'ast>),
    Assign(AssignStatement<'ast>),
    Conditional(ConditionalStatement<'ast>),
    Iteration(ForStatement<'ast>),
    Console(ConsoleFunctionCall<'ast>),
    Expression(ExpressionStatement<'ast>),
    Block(Block<'ast>),
}

impl<'ast> fmt::Display for Statement<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Statement::Return(ref statement) => write!(f, "{}", statement),
            Statement::Definition(ref statement) => write!(f, "{}", statement),
            Statement::Assign(ref statement) => write!(f, "{}", statement),
            Statement::Conditional(ref statement) => write!(f, "{}", statement),
            Statement::Iteration(ref statement) => write!(f, "{}", statement),
            Statement::Console(ref statement) => write!(f, "{}", statement),
            Statement::Expression(ref statement) => write!(f, "{}", statement.expression),
            Statement::Block(ref block) => write!(f, "{}", block),
        }
    }
}
