use crate::{
    ast::Rule,
    circuits::{CircuitFieldDefinition, CircuitFunction},
};

use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::circuit_member))]
pub enum CircuitMember<'ast> {
    CircuitFieldDefinition(CircuitFieldDefinition<'ast>),
    CircuitFunction(CircuitFunction<'ast>),
}
