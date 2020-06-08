use crate::{ast::Rule, common::SpreadOrExpression, values::Value};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::expression_array_initializer))]
pub struct ArrayInitializerExpression<'ast> {
    pub expression: Box<SpreadOrExpression<'ast>>,
    pub count: Value<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
