//! The import type for a Leo program.

use crate::{ImportSymbol, Package, Span};
use leo_ast::imports::Import as AstImport;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Serialize, Deserialize)]
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
    pub fn path_string_full(&self) -> String {
        format!("{}.leo", self.package.name)
    }

    // from "./import" import *;
    pub fn is_star(&self) -> bool {
        // self.symbols.is_empty()
        false
    }

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
