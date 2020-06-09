use crate::{Identifier, RangeOrExpression};
use leo_ast::{
    access::AssigneeAccess,
    common::{Assignee as AstAssignee, Identifier as AstIdentifier},
};

use std::fmt;

/// Definition assignee: v, arr[0..2], Point p.x
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Assignee {
    Identifier(Identifier),
    Array(Box<Assignee>, RangeOrExpression),
    CircuitField(Box<Assignee>, Identifier), // (circuit name, circuit field name)
}

impl<'ast> From<AstIdentifier<'ast>> for Assignee {
    fn from(variable: AstIdentifier<'ast>) -> Self {
        Assignee::Identifier(Identifier::from(variable))
    }
}

impl<'ast> From<AstAssignee<'ast>> for Assignee {
    fn from(assignee: AstAssignee<'ast>) -> Self {
        let variable = Assignee::from(assignee.identifier);

        // We start with the id, and we fold the array of accesses by wrapping the current value
        assignee
            .accesses
            .into_iter()
            .fold(variable, |acc, access| match access {
                AssigneeAccess::Array(array) => {
                    Assignee::Array(Box::new(acc), RangeOrExpression::from(array.expression))
                }
                AssigneeAccess::Member(circuit_field) => {
                    Assignee::CircuitField(Box::new(acc), Identifier::from(circuit_field.identifier))
                }
            })
    }
}

impl fmt::Display for Assignee {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Assignee::Identifier(ref variable) => write!(f, "{}", variable),
            Assignee::Array(ref array, ref index) => write!(f, "{}[{}]", array, index),
            Assignee::CircuitField(ref circuit_variable, ref member) => write!(f, "{}.{}", circuit_variable, member),
        }
    }
}
