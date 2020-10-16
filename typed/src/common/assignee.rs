// Copyright (C) 2019-2020 Aleo Systems Inc.
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

use crate::{Expression, Identifier, RangeOrExpression, Span};
use leo_ast::{
    access::AssigneeAccess as AstAssigneeAccess,
    common::{Assignee as AstAssignee, Identifier as AstIdentifier, SelfKeywordOrIdentifier},
};

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssigneeAccess {
    Array(RangeOrExpression),
    Tuple(usize),
    Member(Identifier),
}

impl<'ast> From<AstAssigneeAccess<'ast>> for AssigneeAccess {
    fn from(access: AstAssigneeAccess<'ast>) -> Self {
        match access {
            AstAssigneeAccess::Array(array) => AssigneeAccess::Array(RangeOrExpression::from(array.expression)),
            AstAssigneeAccess::Tuple(tuple) => AssigneeAccess::Tuple(Expression::get_count_from_ast(tuple.number)),
            AstAssigneeAccess::Member(member) => AssigneeAccess::Member(Identifier::from(member.identifier)),
        }
    }
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

impl<'ast> From<AstAssignee<'ast>> for Assignee {
    fn from(assignee: AstAssignee<'ast>) -> Self {
        Assignee {
            identifier: Identifier::from(assignee.name),
            accesses: assignee
                .accesses
                .into_iter()
                .map(|access| AssigneeAccess::from(access))
                .collect::<Vec<_>>(),
            span: Span::from(assignee.span),
        }
    }
}

impl fmt::Display for Assignee {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.identifier)?;

        for access in &self.accesses {
            match access {
                AssigneeAccess::Array(expression) => write!(f, "[{}]", expression)?,
                AssigneeAccess::Tuple(index) => write!(f, ".{}", index)?,
                AssigneeAccess::Member(member) => write!(f, ".{}", member)?,
            }
        }

        write!(f, "")
    }
}
