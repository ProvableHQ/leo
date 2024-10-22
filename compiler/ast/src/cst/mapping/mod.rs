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

use crate::{Identifier, NodeID, Type};
use crate::cst::Comment;
use leo_span::Span;
use serde::{Deserialize, Serialize};
use snarkvm::prelude::{Mapping as MappingCore, Network};
use std::fmt;

/// A mapping declaration, e.g `mapping balances: address => u128`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mapping {
    /// The name of the mapping.
    pub identifier: Identifier,
    /// The type of the key.
    pub key_type: Type,
    /// The type of the value.
    pub value_type: Type,
    /// The entire span of the mapping declaration.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
    ///The comment of the mapping
    pub comment: Comment
}

impl Mapping {
    pub fn from_snarkvm<N: Network>(mapping: &MappingCore<N>) -> Self {
        Self {
            identifier: Identifier::from(mapping.name()),
            key_type: Type::from_snarkvm(mapping.key().plaintext_type(), None),
            value_type: Type::from_snarkvm(mapping.value().plaintext_type(), None),
            span: Default::default(),
            id: Default::default(),
            comment: Comment::None
        }
    }
}
impl fmt::Display for Mapping {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "mapping {}: {} => {}; ", self.identifier, self.key_type, self.value_type)?;
        self.comment.fmt(f)?;
        Ok(())
    }
}