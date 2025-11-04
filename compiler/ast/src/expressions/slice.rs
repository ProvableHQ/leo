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

use std::fmt::{self, Debug};

use leo_span::Span;
use serde::{Deserialize, Serialize};

use crate::{Expression, Node, NodeID};

/// The slice expression, e.g., `arr[2..=4]``.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Slice {
    /// The expression evaluating to some array type, e.g., `[false, true]`.
    pub source_array: Expression,
    /// The lower_bound to use, if any, when slicing the array.
    pub start: Option<Expression>,
    /// The upper_bound to use, if any, when slicing the array.
    pub stop: Option<Expression>,
    /// Whether `stop` is inclusive or not.
    /// Signified with `=` when parsing.
    pub clusivity: bool,
    /// The span for the entire expression `foo[start..=stop]`.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl fmt::Display for Slice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}[{}..{}{}]",
            self.source_array,
            self.start.clone().map(|start| start.to_string()).unwrap_or("".into()),
            if self.clusivity { "=" } else { "" },
            self.stop.clone().map(|stop| stop.to_string()).unwrap_or("".into())
        )
    }
}

impl From<Slice> for Expression {
    fn from(value: Slice) -> Self {
        Expression::Slice(Box::new(value))
    }
}

crate::simple_node_impl!(Slice);
