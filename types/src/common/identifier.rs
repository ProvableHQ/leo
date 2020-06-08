use leo_ast::common::Identifier as AstIdentifier;

use std::fmt;

/// An identifier in the constrained program.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub name: String,
}

impl Identifier {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn is_self(&self) -> bool {
        self.name == "Self"
    }
}

impl<'ast> From<AstIdentifier<'ast>> for Identifier {
    fn from(identifier: AstIdentifier<'ast>) -> Self {
        Identifier::new(identifier.value)
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl fmt::Debug for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
