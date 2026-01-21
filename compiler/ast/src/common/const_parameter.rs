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

use crate::{Identifier, Node, NodeID, Type};
use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

/// A const parameter for a function or a composite.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstParameter {
    /// The name of the const parameter.
    pub identifier: Identifier,
    /// What's the const parameter's type?
    pub type_: Type,
    /// The const parameter's span from any annotations to its type.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl ConstParameter {
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.identifier, self.type_)
    }

    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }

    pub fn type_(&self) -> &Type {
        &self.type_
    }
}

impl fmt::Display for ConstParameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

crate::simple_node_impl!(ConstParameter);
