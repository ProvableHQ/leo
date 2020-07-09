use crate::{Debug, ErrorMacro, PrintLine};
use leo_ast::macros::MacroName as AstMacroName;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MacroName {
    Debug(Debug),
    Error(ErrorMacro),
    PrintLine(PrintLine),
}

impl<'ast> From<AstMacroName<'ast>> for MacroName {
    fn from(name: AstMacroName<'ast>) -> Self {
        match name {
            AstMacroName::Debug(debug) => MacroName::Debug(Debug::from(debug)),
            AstMacroName::Error(error) => MacroName::Error(ErrorMacro::from(error)),
            AstMacroName::PrintLine(print_line) => MacroName::PrintLine(PrintLine::from(print_line)),
        }
    }
}

impl fmt::Display for MacroName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MacroName::Debug(ref debug) => write!(f, "{}", debug),
            MacroName::Error(ref error) => write!(f, "{}", error),
            MacroName::PrintLine(ref print_line) => write!(f, "{}", print_line),
        }
    }
}
