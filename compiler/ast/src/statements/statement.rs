// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use super::*;

use crate::{ConditionalStatement, Node};

use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

/// Program statement that defines some action (or expression) to be carried out.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Statement {
    /// An block statement of Aleo instructions.
    AssemblyBlock(AssemblyBlock),
    /// An assignment statement.
    Assign(Box<AssignStatement>),
    /// A block statement.
    Block(Block),
    /// An `if` statement.
    Conditional(ConditionalStatement),
    /// A console logging statement.
    Console(ConsoleStatement),
    /// A binding or set of bindings / variables to declare.
    Definition(DefinitionStatement),
    /// A `for` statement.
    Iteration(Box<IterationStatement>),
    /// A return statement `return expr;`.
    Return(ReturnStatement),
}

impl Statement {
    /// Returns a dummy statement made from an empty block `{}`.
    pub fn dummy(span: Span) -> Self {
        Self::Block(Block {
            statements: Vec::new(),
            span,
        })
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::AssemblyBlock(x) => x.fmt(f),
            Statement::Assign(x) => x.fmt(f),
            Statement::Block(x) => x.fmt(f),
            Statement::Conditional(x) => x.fmt(f),
            Statement::Console(x) => x.fmt(f),
            Statement::Definition(x) => x.fmt(f),
            Statement::Iteration(x) => x.fmt(f),
            Statement::Return(x) => x.fmt(f),
        }
    }
}

impl Node for Statement {
    fn span(&self) -> Span {
        match self {
            Statement::AssemblyBlock(n) => n.span(),
            Statement::Assign(n) => n.span(),
            Statement::Block(n) => n.span(),
            Statement::Conditional(n) => n.span(),
            Statement::Console(n) => n.span(),
            Statement::Definition(n) => n.span(),
            Statement::Iteration(n) => n.span(),
            Statement::Return(n) => n.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        match self {
            Statement::AssemblyBlock(n) => n.set_span(span),
            Statement::Assign(n) => n.set_span(span),
            Statement::Block(n) => n.set_span(span),
            Statement::Conditional(n) => n.set_span(span),
            Statement::Console(n) => n.set_span(span),
            Statement::Definition(n) => n.set_span(span),
            Statement::Iteration(n) => n.set_span(span),
            Statement::Return(n) => n.set_span(span),
        }
    }
}
