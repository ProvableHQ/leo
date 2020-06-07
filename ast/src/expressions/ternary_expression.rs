use crate::{ast::Rule, expressions::Expression};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::expression_conditional))]
pub struct TernaryExpression<'ast> {
    pub first: Box<Expression<'ast>>,
    pub second: Box<Expression<'ast>>,
    pub third: Box<Expression<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
