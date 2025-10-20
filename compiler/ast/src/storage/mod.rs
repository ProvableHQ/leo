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

use crate::{Identifier, Node, NodeID, Type};

use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

/// A storage declaration, e.g `storage x: u32`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageVariable {
    /// The name of the storage variable.
    pub identifier: Identifier,
    /// The type of the variable.
    pub type_: Type,
    /// The entire span of the storage declaration.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl fmt::Display for StorageVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "storage {}: {}", self.identifier, self.type_)
    }
}

crate::simple_node_impl!(StorageVariable);
