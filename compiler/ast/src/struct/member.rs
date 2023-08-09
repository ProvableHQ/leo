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

use crate::{Identifier, Mode, Node, NodeID, Type};

use leo_span::{Span, Symbol};

use serde::{Deserialize, Serialize};
use std::fmt;

/// A member of a structured data type, e.g `foobar: u8` or `private baz: bool` .
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Member {
    /// The mode of the member.
    pub mode: Mode,
    /// The identifier of the member.
    pub identifier: Identifier,
    /// The type of the member.
    pub type_: Type,
    /// The span of the member.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl Member {
    /// Returns the name of the struct member without span.
    pub fn name(&self) -> Symbol {
        self.identifier.name
    }
}

impl fmt::Display for Member {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.mode {
            Mode::None => write!(f, "{}: {}", self.identifier, self.type_),
            _ => write!(f, "{} {} {}", self.mode, self.identifier, self.type_),
        }
    }
}

crate::simple_node_impl!(Member);
