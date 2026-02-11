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

use leo_span::{Span, Symbol};

use crate::{Composite, ConstDeclaration, Function, Indent, Interface, Module};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Stores the Leo library abstract syntax tree.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Library {
    pub name: Symbol,
    /// A map from module paths to module definitions.
    pub modules: IndexMap<Vec<Symbol>, Module>,
    /// A map from import names to import definitions.
    pub imports: IndexMap<Symbol, Span>,
    /// A vector of const definitions.
    pub consts: Vec<(Symbol, ConstDeclaration)>,
    /// A vector of composite definitions.
    pub composites: Vec<(Symbol, Composite)>,
    /// A vector of function definitions.
    pub functions: Vec<(Symbol, Function)>,
    /// A vector of interface definitions.
    pub interfaces: Vec<(Symbol, Interface)>,
}

impl fmt::Display for Library {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "library {} {{", self.name)?;

        for (_, module) in self.modules.iter() {
            writeln!(f, "{}", Indent(module))?;
        }
        for (name, _) in self.imports.iter() {
            writeln!(f, "    import {};", name)?;
        }
        for (_, const_decl) in self.consts.iter() {
            writeln!(f, "{};", Indent(const_decl))?;
        }
        for (_, composite) in self.composites.iter() {
            writeln!(f, "{}", Indent(composite))?;
        }
        for (_, function) in self.functions.iter() {
            writeln!(f, "{}", Indent(function))?;
        }
        for (_, interface) in self.interfaces.iter() {
            writeln!(f, "{}", Indent(interface))?;
        }

        writeln!(f, "}}")
    }
}
