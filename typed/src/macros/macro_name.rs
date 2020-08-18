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
