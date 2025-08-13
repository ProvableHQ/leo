// Copyright (C) 2019-2025 Provable Inc.
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

//! A Leo module represents a collection of declarations within a single scope,
//! typically corresponding to a file or logical unit of code. It stores all
//! relevant definitions associated with a module, including:
//!
//! - The name of the program the module belongs to (`program_name`)
//! - The hierarchical path identifying the module (`path`)
//! - A list of constant declarations (`consts`)
//! - A list of struct type definitions (`structs`)
//! - A list of function definitions (`functions`)

use crate::{Composite, ConstDeclaration, Function, Indent};

use leo_span::Symbol;

use std::fmt;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

/// Stores the abstract syntax tree of a Leo module.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Module {
    /// The name of the program that this module belongs to.
    pub program_name: Symbol,
    /// The path to the module.
    pub path: Vec<Symbol>,
    /// A vector of const definitions.
    pub consts: Vec<(Symbol, ConstDeclaration)>,
    /// A vector of struct definitions.
    pub structs: Vec<(Symbol, Composite)>,
    /// A vector of function definitions.
    pub functions: Vec<(Symbol, Function)>,
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "module {} {{", self.path.iter().format("::"))?;
        for (_, const_decl) in self.consts.iter() {
            writeln!(f, "{};", Indent(const_decl))?;
        }
        for (_, struct_) in self.structs.iter() {
            writeln!(f, "{}", Indent(struct_))?;
        }
        for (_, function) in self.functions.iter() {
            writeln!(f, "{}", Indent(function))?;
        }
        writeln!(f, "}}")
    }
}
