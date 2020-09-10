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

use crate::{Expression, Identifier, RangeOrExpression};
use leo_ast::{
    access::AssigneeAccess as AstAssigneeAccess,
    common::{Assignee as AstAssignee, Identifier as AstIdentifier, KeywordOrIdentifier},
};

use serde::{Deserialize, Serialize};
use std::fmt;

/// Definition assignee: v, arr[0..2], Point p.x
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Assignee {
    Identifier(Identifier),
    Array(Box<Assignee>, RangeOrExpression),
    Tuple(Box<Assignee>, usize),
    CircuitField(Box<Assignee>, Identifier), // (circuit name, circuit field name)
}

impl<'ast> From<AstIdentifier<'ast>> for Assignee {
    fn from(variable: AstIdentifier<'ast>) -> Self {
        Assignee::Identifier(Identifier::from(variable))
    }
}

impl<'ast> From<KeywordOrIdentifier<'ast>> for Assignee {
    fn from(name: KeywordOrIdentifier<'ast>) -> Self {
        Assignee::Identifier(Identifier::from(name))
    }
}

impl<'ast> From<AstAssignee<'ast>> for Assignee {
    fn from(assignee: AstAssignee<'ast>) -> Self {
        let variable = Assignee::from(assignee.name);

        // We start with the id, and we fold the array of accesses by wrapping the current value
        assignee
            .accesses
            .into_iter()
            .fold(variable, |acc, access| match access {
                AstAssigneeAccess::Array(array) => {
                    Assignee::Array(Box::new(acc), RangeOrExpression::from(array.expression))
                }
                AstAssigneeAccess::Tuple(tuple) => {
                    Assignee::Tuple(Box::new(acc), Expression::get_count_from_ast(tuple.number))
                }
                AstAssigneeAccess::Member(circuit_variable) => {
                    Assignee::CircuitField(Box::new(acc), Identifier::from(circuit_variable.identifier))
                }
            })
    }
}

impl fmt::Display for Assignee {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Assignee::Identifier(ref variable) => write!(f, "{}", variable),
            Assignee::Array(ref array, ref index) => write!(f, "{}[{}]", array, index),
            Assignee::Tuple(ref tuple, ref index) => write!(f, "{}.{}", tuple, index),
            Assignee::CircuitField(ref circuit_variable, ref member) => write!(f, "{}.{}", circuit_variable, member),
        }
    }
}
