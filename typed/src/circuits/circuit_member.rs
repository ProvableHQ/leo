use crate::{Function, Identifier, Type};
use leo_ast::circuits::{
    CircuitFieldDefinition as AstCircuitFieldDefinition,
    CircuitFunction as AstCircuitFunction,
    CircuitMember as AstCircuitMember,
};

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CircuitMember {
    CircuitField(Identifier, Type),
    CircuitFunction(bool, Function),
}

impl<'ast> From<AstCircuitFieldDefinition<'ast>> for CircuitMember {
    fn from(circuit_value: AstCircuitFieldDefinition<'ast>) -> Self {
        CircuitMember::CircuitField(
            Identifier::from(circuit_value.identifier),
            Type::from(circuit_value._type),
        )
    }
}

impl<'ast> From<AstCircuitFunction<'ast>> for CircuitMember {
    fn from(circuit_function: AstCircuitFunction<'ast>) -> Self {
        CircuitMember::CircuitFunction(
            circuit_function._static.is_some(),
            Function::from(circuit_function.function),
        )
    }
}

impl<'ast> From<AstCircuitMember<'ast>> for CircuitMember {
    fn from(object: AstCircuitMember<'ast>) -> Self {
        match object {
            AstCircuitMember::CircuitFieldDefinition(circuit_value) => CircuitMember::from(circuit_value),
            AstCircuitMember::CircuitFunction(circuit_function) => CircuitMember::from(circuit_function),
        }
    }
}

impl fmt::Display for CircuitMember {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CircuitMember::CircuitField(ref identifier, ref _type) => write!(f, "{}: {}", identifier, _type),
            CircuitMember::CircuitFunction(ref _static, ref function) => {
                if *_static {
                    write!(f, "static ")?;
                }
                write!(f, "{}", function)
            }
        }
    }
}
