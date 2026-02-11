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

use crate::{ConstDeclaration, Indent};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Stores the Leo library abstract syntax tree.
///
/// Currently libraries may only contain `const` declarations. Extending this to support
/// structs and other items is left for future work.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Library {
    pub name: Symbol,
    /// The constants defined in this library.
    pub consts: Vec<(Symbol, ConstDeclaration)>,
}

impl fmt::Display for Library {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "library {} {{", self.name)?;

        for (_, const_decl) in self.consts.iter() {
            writeln!(f, "{};", Indent(const_decl))?;
        }

        writeln!(f, "}}")
    }
}
