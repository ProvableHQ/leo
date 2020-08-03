use crate::{Debug, ErrorMacro, Print};
use leo_ast::macros::MacroName as AstMacroName;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MacroName {
    Debug(Debug),
    Error(ErrorMacro),
    Print(Print),
}

impl<'ast> From<AstMacroName<'ast>> for MacroName {
    fn from(name: AstMacroName<'ast>) -> Self {
        match name {
            AstMacroName::Debug(debug) => MacroName::Debug(Debug::from(debug)),
            AstMacroName::Error(error) => MacroName::Error(ErrorMacro::from(error)),
            AstMacroName::Print(print_line) => MacroName::Print(Print::from(print_line)),
        }
    }
}

impl fmt::Display for MacroName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MacroName::Debug(ref debug) => write!(f, "{}", debug),
            MacroName::Error(ref error) => write!(f, "{}", error),
            MacroName::Print(ref print_line) => write!(f, "{}", print_line),
        }
    }
}
