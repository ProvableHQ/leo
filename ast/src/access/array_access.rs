use crate::{ast::Rule, common::RangeOrExpression};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::access_array))]
pub struct ArrayAccess<'ast> {
    pub expression: RangeOrExpression<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
