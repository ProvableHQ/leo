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

use crate::{Expression, Node, NodeID};
use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

/// A special access expression e.g. `self.id`, `block.height` etc.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecialAccess {
    /// The special access variant.
    pub variant: SpecialAccessVariant,
    /// The span covering all of special access expression.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpecialAccessVariant {
    Id,
    Caller,
    Signer,
    Address,
    Edition,
    Checksum,
    ProgramOwner,
    NetworkId,
    BlockHeight,
}

impl fmt::Display for SpecialAccess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.variant {
            SpecialAccessVariant::Id => write!(f, "self.id"),
            SpecialAccessVariant::Caller => write!(f, "self.caller"),
            SpecialAccessVariant::Signer => write!(f, "self.signer"),
            SpecialAccessVariant::Address => write!(f, "self.address"),
            SpecialAccessVariant::Edition => write!(f, "self.edition"),
            SpecialAccessVariant::Checksum => write!(f, "self.checksum"),
            SpecialAccessVariant::ProgramOwner => write!(f, "self.program_owner"),
            SpecialAccessVariant::NetworkId => write!(f, "network.id"),
            SpecialAccessVariant::BlockHeight => write!(f, "block.height"),
        }
    }
}

impl fmt::Display for SpecialAccessVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SpecialAccessVariant::Id => write!(f, "self.id"),
            SpecialAccessVariant::Caller => write!(f, "self.caller"),
            SpecialAccessVariant::Signer => write!(f, "self.signer"),
            SpecialAccessVariant::Address => write!(f, "self.address"),
            SpecialAccessVariant::Edition => write!(f, "self.edition"),
            SpecialAccessVariant::Checksum => write!(f, "self.checksum"),
            SpecialAccessVariant::ProgramOwner => write!(f, "self.program_owner"),
            SpecialAccessVariant::NetworkId => write!(f, "network.id"),
            SpecialAccessVariant::BlockHeight => write!(f, "block.height"),
        }
    }
}

impl From<SpecialAccess> for Expression {
    fn from(value: SpecialAccess) -> Self {
        Expression::SpecialAccess(value)
    }
}

crate::simple_node_impl!(SpecialAccess);
