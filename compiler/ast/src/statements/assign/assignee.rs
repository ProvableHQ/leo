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

use crate::{Expression, Identifier, PositiveNumber};
use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

#[allow(clippy::large_enum_variant)]
/// A sub-place in a variable to assign to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssigneeAccess {
    /// Assignment to a range in an array.
    ArrayRange(Option<Expression>, Option<Expression>),
    /// Assignment to an element of an array identified by its index.
    ArrayIndex(Expression),
    /// Assignment to a tuple field by its position, e.g., `2`.
    Tuple(PositiveNumber, #[serde(with = "leo_span::span_json")] Span),
    /// Assignment to a field in a structure.
    Member(Identifier),
}

/// Definition assignee, e.g., `v`, `arr[0..2]`, `p.x`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Assignee {
    /// The base variable to assign to.
    pub identifier: Identifier,
    /// Sub-places within `identifier` to assign to, if any.
    pub accesses: Vec<AssigneeAccess>,
    pub span: Span,
}

impl Assignee {
    /// Returns the name of the variable being assigned to.
    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }
}

impl fmt::Display for Assignee {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.identifier)?;

        for access in &self.accesses {
            match access {
                AssigneeAccess::ArrayRange(Some(left), Some(right)) => write!(f, "[{}..{}]", left, right)?,
                AssigneeAccess::ArrayRange(None, Some(right)) => write!(f, "[..{}]", right)?,
                AssigneeAccess::ArrayRange(Some(left), None) => write!(f, "[{}..]", left)?,
                AssigneeAccess::ArrayRange(None, None) => write!(f, "[..]")?,
                AssigneeAccess::ArrayIndex(index) => write!(f, "[{}]", index)?,
                AssigneeAccess::Tuple(index, _span) => write!(f, ".{}", index)?,
                AssigneeAccess::Member(member) => write!(f, ".{}", member)?,
            }
        }

        write!(f, "")
    }
}
