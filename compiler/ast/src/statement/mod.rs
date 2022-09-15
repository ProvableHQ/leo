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

pub mod assign;
pub use assign::*;

pub mod block;
pub use block::*;

pub mod conditional;
pub use conditional::*;

pub mod console;
pub use console::*;

pub mod decrement;
pub use decrement::*;

pub mod definition;
pub use definition::*;

pub mod finalize;
pub use finalize::*;

pub mod increment;
pub use increment::*;

pub mod iteration;
pub use iteration::*;

pub mod return_;
pub use return_::*;

use crate::Node;

use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

/// Program statement that defines some action (or expression) to be carried out.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Statement {
    /// An assignment statement.
    Assign(Box<AssignStatement>),
    /// A block statement.
    Block(Block),
    /// An `if` statement.
    Conditional(ConditionalStatement),
    /// A console logging statement.
    Console(ConsoleStatement),
    /// A decrement statement.
    Decrement(DecrementStatement),
    /// A binding or set of bindings / variables to declare.
    Definition(DefinitionStatement),
    /// A finalize statement.
    Finalize(FinalizeStatement),
    /// An increment statement.
    Increment(IncrementStatement),
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
            Statement::Assign(x) => x.fmt(f),
            Statement::Block(x) => x.fmt(f),
            Statement::Conditional(x) => x.fmt(f),
            Statement::Console(x) => x.fmt(f),
            Statement::Decrement(x) => x.fmt(f),
            Statement::Definition(x) => x.fmt(f),
            Statement::Finalize(x) => x.fmt(f),
            Statement::Increment(x) => x.fmt(f),
            Statement::Iteration(x) => x.fmt(f),
            Statement::Return(x) => x.fmt(f),
        }
    }
}

impl Node for Statement {
    fn span(&self) -> Span {
        use Statement::*;
        match self {
            Assign(n) => n.span(),
            Block(n) => n.span(),
            Conditional(n) => n.span(),
            Console(n) => n.span(),
            Decrement(n) => n.span(),
            Definition(n) => n.span(),
            Finalize(n) => n.span(),
            Increment(n) => n.span(),
            Iteration(n) => n.span(),
            Return(n) => n.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        use Statement::*;
        match self {
            Assign(n) => n.set_span(span),
            Block(n) => n.set_span(span),
            Conditional(n) => n.set_span(span),
            Console(n) => n.set_span(span),
            Decrement(n) => n.set_span(span),
            Definition(n) => n.set_span(span),
            Finalize(n) => n.set_span(span),
            Increment(n) => n.set_span(span),
            Iteration(n) => n.set_span(span),
            Return(n) => n.set_span(span),
        }
    }
}
