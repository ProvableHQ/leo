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

pub mod member;
pub use member::*;

use crate::{Identifier, NodeID};
use crate::cst::Comment;
use leo_span::{Span, Symbol};
use serde::{Deserialize, Serialize};
use std::fmt;

/// A composite type definition, e.g., `struct Foo { my_field: Bar }` and `record Token { owner: address, amount: u64}`.
/// In some languages these are called `struct`s.
///
/// Type identity is decided by the full path including `struct_name`,
/// as the record is nominal, not structural.
/// The fields are named so `struct Foo(u8, u16)` is not allowed.
#[derive(Clone, Serialize, Deserialize)]
pub struct Composite {
    /// The name of the type in the type system in this module.
    pub identifier: Identifier,
    /// The fields, constant variables, and functions of this structure.
    pub members: Vec<(Member, Vec<Comment>)>,
    /// The external program the struct is defined in.
    pub external: Option<Symbol>,
    /// Was this a `record Foo { ... }`?
    /// If so, it wasn't a composite.
    pub is_record: bool,
    /// The entire span of the composite definition.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
    /// The comment
    pub comment: Comment,
}

impl PartialEq for Composite {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier
    }
}

impl Eq for Composite {}

impl fmt::Debug for Composite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

impl fmt::Display for Composite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(if self.is_record { "record" } else { "struct" })?;
        write!(f, " {} {{ ", self.identifier)?;
        self.comment.fmt(f)?;
        for field in self.members.iter() {
            for comment in &field.1 {
                write!(f, "        ")?;
                comment.fmt(f)?;
            }
            write!(f, "        {}", field.0)?;
        }
        if let Some(memeber) = self.members.last() {
            if memeber.0.comment == Comment::None {
                writeln!(f, "")?;
            }else {
                write!(f, "")?;
            }
        }
        write!(f, "    }}")
    }
}