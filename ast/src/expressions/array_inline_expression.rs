use crate::{ast::{Rule, SpreadOrExpression}};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::expression_array_inline))]
pub struct ArrayInlineExpression<'ast> {
    pub expressions: Vec<SpreadOrExpression<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
