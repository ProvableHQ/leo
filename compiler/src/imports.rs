//! The Import type for a Leo program.

use crate::Identifier;

use std::fmt;

#[derive(Clone)]
pub struct ImportSymbol {
    pub symbol: Identifier,
    pub alias: Option<Identifier>,
}

#[derive(Clone)]
pub struct Import {
    pub path_string: String,
    pub symbols: Vec<ImportSymbol>,
}

impl Import {
    pub fn new(source: String, symbols: Vec<ImportSymbol>) -> Import {
        Import {
            path_string: source,
            symbols,
        }
    }

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

impl fmt::Display for ImportSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.alias.is_some() {
            write!(f, "\t{} as {}", self.symbol, self.alias.as_ref().unwrap())
        } else {
            write!(f, "\t{}", self.symbol)
        }
    }
}

impl<'ast> fmt::Display for Import {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl<'ast> fmt::Debug for Import {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}
