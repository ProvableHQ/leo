// Copyright (C) 2019-2020 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::{Identifier, Span};
use leo_ast::imports::ImportSymbol as AstImportSymbol;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
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

// TODO (collin): remove this
impl fmt::Debug for ImportSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.alias.is_some() {
            write!(f, "{} as {}", self.symbol, self.alias.as_ref().unwrap())
        } else {
            write!(f, "{}", self.symbol)
        }
    }
}

impl ImportSymbol {
    pub fn star(span: &Span) -> Self {
        Self {
            symbol: Identifier {
                name: "*".to_string(),
                span: span.clone(),
            },
            alias: None,
            span: span.clone(),
        }
    }

    pub fn is_star(&self) -> bool {
        self.symbol.name.eq("*")
    }
}
