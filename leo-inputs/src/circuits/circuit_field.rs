use crate::{ast::Rule, common::Identifier, expressions::Expression};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::circuit_field))]
pub struct CircuitField<'ast> {
    pub identifier: Identifier<'ast>,
    pub expression: Expression<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
