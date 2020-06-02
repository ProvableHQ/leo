//! The Import type for a Leo program.

use crate::Identifier;

use snarkos_models::curves::{Field, PrimeField};
use std::fmt;

#[derive(Clone)]
pub struct ImportSymbol<F: Field + PrimeField> {
    pub symbol: Identifier<F>,
    pub alias: Option<Identifier<F>>,
}

#[derive(Clone)]
pub struct Import<F: Field + PrimeField> {
    pub path_string: String,
    pub symbols: Vec<ImportSymbol<F>>,
}

impl<F: Field + PrimeField> Import<F> {
    pub fn new(source: String, symbols: Vec<ImportSymbol<F>>) -> Import<F> {
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

impl<F: Field + PrimeField> fmt::Display for ImportSymbol<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.alias.is_some() {
            write!(f, "\t{} as {}", self.symbol, self.alias.as_ref().unwrap())
        } else {
            write!(f, "\t{}", self.symbol)
        }
    }
}

impl<'ast, F: Field + PrimeField> fmt::Display for Import<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl<'ast, F: Field + PrimeField> fmt::Debug for Import<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}
