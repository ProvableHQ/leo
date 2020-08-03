//! The import type for a Leo program.

use crate::{Package, Span};
use leo_ast::imports::Import as AstImport;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Import {
    pub package: Package,
    pub span: Span,
}

impl<'ast> From<AstImport<'ast>> for Import {
    fn from(import: AstImport<'ast>) -> Self {
        Import {
            package: Package::from(import.package),
            span: Span::from(import.span),
        }
    }
}

impl Import {
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "import {};", self.package)
    }
}

impl fmt::Display for Import {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl fmt::Debug for Import {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}
