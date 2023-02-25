// Copyright (C) 2019-2023 Aleo Systems Inc.
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

pub mod associated_function_access;
pub use associated_function_access::*;

pub mod member_access;
pub use member_access::*;

pub mod tuple_access;
pub use tuple_access::*;

use crate::Node;

use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

/// An access expressions, extracting a smaller part out of a whole.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessExpression {
    // /// An `array[index]` expression.
    // Array(ArrayAccess),
    // /// An expression accessing a range of an array.
    // ArrayRange(ArrayRangeAccess),
    /// Access to an associated function of a struct e.g `Pedersen64::hash()`.
    AssociatedFunction(AssociatedFunction),
    /// An expression accessing a field in a structure, e.g., `struct_var.field`.
    Member(MemberAccess),
    /// Access to a tuple field using its position, e.g., `tuple.1`.
    Tuple(TupleAccess),
}

impl Node for AccessExpression {
    fn span(&self) -> Span {
        match self {
            AccessExpression::AssociatedFunction(n) => n.span(),
            AccessExpression::Member(n) => n.span(),
            AccessExpression::Tuple(n) => n.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        match self {
            AccessExpression::AssociatedFunction(n) => n.set_span(span),
            AccessExpression::Member(n) => n.set_span(span),
            AccessExpression::Tuple(n) => n.set_span(span),
        }
    }
}

impl fmt::Display for AccessExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AccessExpression::*;

        match self {
            AssociatedFunction(access) => access.fmt(f),
            Member(access) => access.fmt(f),
            Tuple(access) => access.fmt(f),
        }
    }
}
