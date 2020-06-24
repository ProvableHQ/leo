use crate::{ast::Rule, expressions::Expression};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::range))]
pub struct Range<'ast> {
    pub from: Option<Expression<'ast>>,
    pub to: Option<Expression<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
