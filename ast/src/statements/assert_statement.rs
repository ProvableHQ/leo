use crate::{ast::Rule, common::LineEnd, expressions::Expression};

use pest::Span;
use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::statement_assert))]
pub enum AssertStatement<'ast> {
    AssertEq(AssertEq<'ast>),
}

impl<'ast> fmt::Display for AssertStatement<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssertStatement::AssertEq(ref assert) => write!(f, "assert_eq({}, {});", assert.left, assert.right),
        }
    }
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::assert_eq))]
pub struct AssertEq<'ast> {
    pub left: Expression<'ast>,
    pub right: Expression<'ast>,
    pub line_end: LineEnd,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
