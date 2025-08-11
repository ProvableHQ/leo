// Copyright (C) 2019-2025 Provable Inc.
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

mod assert;
pub use assert::*;

mod assign;
pub use assign::*;

mod block;
pub use block::*;

mod conditional;
pub use conditional::*;

mod const_;
pub use const_::*;

mod definition;
pub use definition::*;

mod expression;
pub use expression::*;

mod iteration;
pub use iteration::*;

mod return_;
pub use return_::*;

use crate::{Expression, Node, NodeID};

use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

/// Program statement that defines some action (or expression) to be carried out.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Statement {
    /// An assert statement.
    Assert(AssertStatement),
    /// An assignment statement.
    Assign(Box<AssignStatement>),
    /// A block statement.
    Block(Block),
    /// An `if` statement.
    Conditional(ConditionalStatement),
    /// A binding from identifier to constant value.
    Const(ConstDeclaration),
    /// A binding or set of bindings / variables to declare.
    Definition(DefinitionStatement),
    /// An expression statement
    Expression(ExpressionStatement),
    /// A `for` statement.
    Iteration(Box<IterationStatement>),
    /// A return statement `return expr;`.
    Return(ReturnStatement),
}

impl Statement {
    /// Returns a dummy statement made from an empty block `{}`.
    pub fn dummy() -> Self {
        Block { statements: Vec::new(), span: Default::default(), id: Default::default() }.into()
    }

    pub(crate) fn semicolon(&self) -> &'static str {
        use Statement::*;

        if matches!(self, Block(..) | Conditional(..) | Iteration(..)) { "" } else { ";" }
    }

    pub fn is_empty(self: &Statement) -> bool {
        match self {
            Statement::Block(block) => block.statements.is_empty(),
            Statement::Return(return_) => matches!(return_.expression, Expression::Unit(_)),
            _ => false,
        }
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Assert(x) => x.fmt(f),
            Statement::Assign(x) => x.fmt(f),
            Statement::Block(x) => x.fmt(f),
            Statement::Conditional(x) => x.fmt(f),
            Statement::Const(x) => x.fmt(f),
            Statement::Definition(x) => x.fmt(f),
            Statement::Expression(x) => x.fmt(f),
            Statement::Iteration(x) => x.fmt(f),
            Statement::Return(x) => x.fmt(f),
        }
    }
}

impl Node for Statement {
    fn span(&self) -> Span {
        use Statement::*;
        match self {
            Assert(n) => n.span(),
            Assign(n) => n.span(),
            Block(n) => n.span(),
            Conditional(n) => n.span(),
            Const(n) => n.span(),
            Definition(n) => n.span(),
            Expression(n) => n.span(),
            Iteration(n) => n.span(),
            Return(n) => n.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        use Statement::*;
        match self {
            Assert(n) => n.set_span(span),
            Assign(n) => n.set_span(span),
            Block(n) => n.set_span(span),
            Conditional(n) => n.set_span(span),
            Const(n) => n.set_span(span),
            Definition(n) => n.set_span(span),
            Expression(n) => n.set_span(span),
            Iteration(n) => n.set_span(span),
            Return(n) => n.set_span(span),
        }
    }

    fn id(&self) -> NodeID {
        use Statement::*;
        match self {
            Assert(n) => n.id(),
            Assign(n) => n.id(),
            Block(n) => n.id(),
            Conditional(n) => n.id(),
            Const(n) => n.id(),
            Definition(n) => n.id(),
            Expression(n) => n.id(),
            Iteration(n) => n.id(),
            Return(n) => n.id(),
        }
    }

    fn set_id(&mut self, id: NodeID) {
        use Statement::*;
        match self {
            Assert(n) => n.set_id(id),
            Assign(n) => n.set_id(id),
            Block(n) => n.set_id(id),
            Conditional(n) => n.set_id(id),
            Const(n) => n.set_id(id),
            Definition(n) => n.set_id(id),
            Expression(n) => n.set_id(id),
            Iteration(n) => n.set_id(id),
            Return(n) => n.set_id(id),
        }
    }
}
