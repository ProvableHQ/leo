// Copyright (C) 2019-2023 Aleo Systems Inc.
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

//! A Leo program scope consists of struct, function, and mapping definitions.

use crate::{ConstDeclaration, Function, Mapping, ProgramId, Struct};

use leo_span::{Span, Symbol};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Stores the Leo program scope abstract syntax tree.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProgramScope {
    /// The program id of the program scope.
    pub program_id: ProgramId,
    /// A vector of const definitions
    pub consts: Vec<(Symbol, ConstDeclaration)>,
    /// A vector of struct definitions.
    pub structs: Vec<(Symbol, Struct)>,
    /// A vector of mapping definitions.
    pub mappings: Vec<(Symbol, Mapping)>,
    /// A vector of function definitions.
    pub functions: Vec<(Symbol, Function)>,
    /// The span associated with the program scope.
    pub span: Span,
}

impl fmt::Display for ProgramScope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "program {} {{", self.program_id)?;
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
