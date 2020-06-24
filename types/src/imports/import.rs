//! The import type for a Leo program.

use crate::{ImportSymbol, Span};
use leo_ast::imports::Import as AstImport;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Serialize, Deserialize)]
pub struct Import {
    pub path_string: String,
    pub symbols: Vec<ImportSymbol>,
    pub span: Span,
}

impl<'ast> From<AstImport<'ast>> for Import {
    fn from(import: AstImport<'ast>) -> Self {
        Import {
            path_string: import.source.value,
            symbols: import
                .symbols
                .into_iter()
                .map(|symbol| ImportSymbol::from(symbol))
                .collect(),
            span: Span::from(import.span),
        }
    }
}

impl Import {
    pub fn path_string_full(&self) -> String {
        format!("{}.leo", self.path_string)
    }

    // from "./import" import *;
    pub fn is_star(&self) -> bool {
        self.symbols.is_empty()
    }

    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "from {} import ", self.path_string)?;
        if self.symbols.is_empty() {
            write!(f, "*")
        } else {
            write!(f, "{{\n")?;
            for (i, symbol) in self.symbols.iter().enumerate() {
                write!(f, "{}", symbol)?;
                if i < self.symbols.len() - 1 {
                    write!(f, ",\n")?;
                }
            }
            write!(f, "\n}}")
        }
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
