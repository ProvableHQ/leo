use crate::{Identifier, Type};
use leo_ast::common::Variable as AstVariable;

use std::fmt;

/// A variable that is assigned to a value in the constrained program
#[derive(Clone, PartialEq, Eq)]
pub struct Variable {
    pub identifier: Identifier,
    pub mutable: bool,
    pub _type: Option<Type>,
}

impl<'ast> From<AstVariable<'ast>> for Variable {
    fn from(variable: AstVariable<'ast>) -> Self {
        Variable {
            identifier: Identifier::from(variable.identifier),
            mutable: variable.mutable.is_some(),
            _type: variable._type.map(|_type| Type::from(_type)),
        }
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.mutable {
            write!(f, "mut ")?;
        }

        write!(f, "{}", self.identifier)?;

        if self._type.is_some() {
            write!(f, ": {}", self._type.as_ref().unwrap())?;
        }

        write!(f, "")
    }
}
