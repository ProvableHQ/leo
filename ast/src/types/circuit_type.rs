use crate::{ast::Rule, common::Identifier};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_circuit))]
pub struct CircuitType<'ast> {
    pub identifier: Identifier<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
