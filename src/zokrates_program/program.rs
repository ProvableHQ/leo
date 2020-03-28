//! A zokrates_program consists of nodes that keep track of position and wrap zokrates_program types.
//!
//! @file zokrates_program.rs
//! @author Collin Chin <collin@aleo.org>
//! @date 2020

use crate::zokrates_program::{Expression, ExpressionList, Statement, StatementNode, Variable};

use pest::Span;
use std::fmt;
use std::fmt::Formatter;

/// Position in input file
#[derive(Clone, Copy)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// Building blocks for a zokrates_program
#[derive(Debug, Clone)]
pub struct Node<T: fmt::Display> {
    start: Position,
    end: Position,
    value: T,
}

impl<T: fmt::Display> Node<T> {
    pub fn new(start: Position, end: Position, value: T) -> Node<T> {
        Self { start, end, value }
    }
}

impl<T: fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T: NodeValue> std::cmp::PartialEq for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool {
        self.value.eq(&other.value)
    }
}

pub trait NodeValue: fmt::Display + Sized + PartialEq {
    fn span(self, span: Span) -> Node<Self> {
        let start = span.start_pos().line_col();
        let end = span.end_pos().line_col();

        let start = Position {
            line: start.0,
            column: start.1,
        };

        let end = Position {
            line: end.0,
            column: end.1,
        };

        Node::new(start, end, self)
    }
}

impl<V: NodeValue> From<V> for Node<V> {
    fn from(v: V) -> Self {
        let mock_position = Position { line: 1, column: 1 };

        Self::new(mock_position, mock_position, v)
    }
}

impl<'ast> NodeValue for Expression<'ast> {}
impl<'ast> NodeValue for ExpressionList<'ast> {}
impl<'ast> NodeValue for Statement<'ast> {}
impl<'ast> NodeValue for Variable<'ast> {}

/// A collection of nodes created from an abstract syntax tree.
#[derive(Debug)]
pub struct Program<'ast> {
    pub nodes: Vec<StatementNode<'ast>>,
}
