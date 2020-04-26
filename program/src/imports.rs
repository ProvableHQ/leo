use crate::Variable;

use snarkos_models::curves::{Field, PrimeField};
use std::fmt;
use std::path::Path;

type ImportPath<'ast> = &'ast Path;
// pub(crate) type Variable<'ast = &'ast str;

#[derive(Clone)]
pub struct ImportSymbol<F: Field + PrimeField> {
    pub symbol: Variable<F>,
    pub alias: Option<Variable<F>>,
}

#[derive(Clone)]
pub struct Import<'ast, F: Field + PrimeField> {
    pub(crate) source: ImportPath<'ast>,
    pub(crate) symbols: Vec<ImportSymbol<F>>,
}

impl<'ast, F: Field + PrimeField> Import<'ast, F> {
    pub fn new(source: ImportPath<'ast>, symbols: Vec<ImportSymbol<F>>) -> Import<'ast, F> {
        Import { source, symbols }
    }

    pub fn get_source(&self) -> &Path {
        &self.source
    }

    pub fn get_file(&self) -> String {
        let path = self.get_source().to_str().unwrap();
        format!("{}.leo", path)
    }

    pub fn is_star(&self) -> bool {
        self.symbols.is_empty()
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

impl<'ast, F: Field + PrimeField> fmt::Display for Import<'ast, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "from {} import ", self.source.display())?;
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

impl<'ast, F: Field + PrimeField> fmt::Debug for Import<'ast, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "from {} import ", self.source.display())?;
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
