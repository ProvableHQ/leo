use crate::{ast::Rule, common::LineEnd, expressions::Expression};

use pest::Span;
use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::assert_eq))]
pub struct AssertEq<'ast> {
    pub left: Expression<'ast>,
    pub right: Expression<'ast>,
    pub line_end: LineEnd,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for AssertEq<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "assert_eq({}, {});", self.left, self.right)
    }
}
