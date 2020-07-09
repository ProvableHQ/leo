use crate::Span;
use leo_ast::macros::FormattedContainer as AstFormattedContainer;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormattedContainer {
    pub span: Span,
}

impl<'ast> From<AstFormattedContainer<'ast>> for FormattedContainer {
    fn from(container: AstFormattedContainer<'ast>) -> Self {
        Self {
            span: Span::from(container.span),
        }
    }
}

impl fmt::Display for FormattedContainer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{}}")
    }
}
