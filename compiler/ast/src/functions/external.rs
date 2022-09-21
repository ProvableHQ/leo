// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use crate::{Identifier, Mode, Node, Type};
use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

/// A function output from an external program with type record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct External {
    /// The name of the external program.
    pub external: Identifier,
    /// The name of the external record type.
    pub record: Identifier,
    /// The parameters span from any annotations to its type.
    pub span: Span,
}

impl External {
    pub fn get_type(&self) -> Type {
        Type::Identifier(self.record.clone())
    }
}

impl fmt::Display for External {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.leo/{}.record", self.external, self.record)
    }
}

crate::simple_node_impl!(FunctionOutputExternal);
