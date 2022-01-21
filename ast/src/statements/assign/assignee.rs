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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssigneeAccess {
    ArrayRange(Option<Expression>, Option<Expression>),
    ArrayIndex(Expression),
    Tuple(PositiveNumber, #[serde(with = "leo_span::span_json")] Span),
    Member(Identifier),
}

/// Definition assignee: v, arr[0..2], Point p.x
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Assignee {
    pub identifier: Identifier,
    pub accesses: Vec<AssigneeAccess>,
    pub span: Span,
}

impl Assignee {
    /// Returns the name of the variable being assigned to
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
