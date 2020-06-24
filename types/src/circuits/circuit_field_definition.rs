use crate::{Expression, Identifier};
use leo_ast::circuits::CircuitField;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CircuitFieldDefinition {
    pub identifier: Identifier,
    pub expression: Expression,
}

impl<'ast> From<CircuitField<'ast>> for CircuitFieldDefinition {
    fn from(member: CircuitField<'ast>) -> Self {
        CircuitFieldDefinition {
            identifier: Identifier::from(member.identifier),
            expression: Expression::from(member.expression),
        }
    }
}
