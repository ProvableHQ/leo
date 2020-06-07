use crate::{ast::{Rule, CircuitField}, common::Identifier,};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::expression_circuit_inline))]
pub struct CircuitInlineExpression<'ast> {
    pub identifier: Identifier<'ast>,
    pub members: Vec<CircuitField<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
