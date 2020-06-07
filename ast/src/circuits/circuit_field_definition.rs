use crate::{ast::Rule, common::Identifier, types::Type};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::circuit_field_definition))]
pub struct CircuitFieldDefinition<'ast> {
    pub identifier: Identifier<'ast>,
    pub _type: Type<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
