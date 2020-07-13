use crate::{
    ast::Rule,
    macros::{Debug, Error, Print},
};

use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::macro_name))]
pub enum MacroName<'ast> {
    Debug(Debug<'ast>),
    Error(Error<'ast>),
    Print(Print<'ast>),
}

impl<'ast> fmt::Display for MacroName<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MacroName::Debug(ref debug) => write!(f, "{}", debug),
            MacroName::Error(ref error) => write!(f, "{}", error),
            MacroName::Print(ref print_line) => write!(f, "{}", print_line),
        }
    }
}
