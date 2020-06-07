use crate::{ast::{Rule, Access}, common::Identifier};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::expression_postfix))]
pub struct PostfixExpression<'ast> {
    pub identifier: Identifier<'ast>,
    pub accesses: Vec<Access<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
