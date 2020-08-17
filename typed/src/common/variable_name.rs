use crate::common::{Identifier, Span};
use leo_ast::common::VariableName as AstVariableName;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VariableName {
    pub mutable: bool,
    pub identifier: Identifier,
    pub span: Span,
}

impl<'ast> From<AstVariableName<'ast>> for VariableName {
    fn from(name: AstVariableName<'ast>) -> Self {
        Self {
            mutable: name.mutable.is_some(),
            identifier: Identifier::from(name.identifier),
            span: Span::from(name.span),
        }
    }
}

impl fmt::Display for VariableName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.mutable {
            write!(f, "mut ")?;
        }

        write!(f, "{}", self.identifier)
    }
}
