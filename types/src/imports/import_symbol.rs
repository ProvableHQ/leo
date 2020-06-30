use crate::{Identifier, Span};
use leo_ast::imports::ImportSymbol as AstImportSymbol;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Serialize, Deserialize)]
pub struct ImportSymbol {
    pub symbol: Identifier,
    pub alias: Option<Identifier>,
    pub span: Span,
}

impl<'ast> From<AstImportSymbol<'ast>> for ImportSymbol {
    fn from(symbol: AstImportSymbol<'ast>) -> Self {
        ImportSymbol {
            symbol: Identifier::from(symbol.value),
            alias: symbol.alias.map(|alias| Identifier::from(alias)),
            span: Span::from(symbol.span),
        }
    }
}

impl fmt::Display for ImportSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.alias.is_some() {
            write!(f, "{} as {}", self.symbol, self.alias.as_ref().unwrap())
        } else {
            write!(f, "{}", self.symbol)
        }
    }
}
