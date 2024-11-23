// Copyright (C) 2019-2024 Aleo Systems Inc.
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

//! A Leo test consists of struct, record, function, transition, and mapping definitions.
//! Anything that can be defined within a program scope can be defined in a test.

use crate::{Composite, ConstDeclaration, Function, Mapping, ProgramId, Stub};

use leo_span::{Span, Symbol};
use serde::{Deserialize, Serialize};
use std::fmt;

/// An abstract syntax tree for a Leo test.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Test {
    /// The constant definitions
    pub consts: Vec<(Symbol, ConstDeclaration)>,
    /// A vector of struct/record definitions.
    pub structs: Vec<(Symbol, Composite)>,
    /// A vector of mapping definitions.
    pub mappings: Vec<(Symbol, Mapping)>,
    /// A vector of function definitions.
    pub functions: Vec<(Symbol, Function)>,
}

impl fmt::Display for Test {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (_, const_) in self.consts.iter() {
            writeln!(f, "    {const_}")?;
        }
        for (_, struct_) in self.structs.iter() {
            writeln!(f, "    {struct_}")?;
        }
        for (_, mapping) in self.mappings.iter() {
            writeln!(f, "    {mapping}")?;
        }
        for (_, function) in self.functions.iter() {
            writeln!(f, "    {function}")?;
        }
        Ok(())
    }
}
