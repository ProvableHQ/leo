// Copyright (C) 2019-2026 Provable Inc.
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

use leo_span::Symbol;

use crate::{Composite, ConstDeclaration, Function, Indent};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Stores the Leo library abstract syntax tree.
///
/// Libraries may contain `const` declarations, `struct` definitions, and `fn` functions.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Library {
    pub name: Symbol,
    /// The constants defined in this library.
    pub consts: Vec<(Symbol, ConstDeclaration)>,
    /// The struct definitions in this library.
    pub structs: Vec<(Symbol, Composite)>,
    /// The function definitions in this library.
    pub functions: Vec<(Symbol, Function)>,
}

impl fmt::Display for Library {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "library {} {{", self.name)?;

        for (_, struct_def) in self.structs.iter() {
            writeln!(f, "{}", Indent(struct_def))?;
        }

        for (_, func) in self.functions.iter() {
            writeln!(f, "{}", Indent(func))?;
        }

        for (_, const_decl) in self.consts.iter() {
            writeln!(f, "{};", Indent(const_decl))?;
        }

        writeln!(f, "}}")
    }
}
