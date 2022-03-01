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

use super::*;
use crate::accesses::*;

/// An access expressions, extracting a smaller part out of a whole.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessExpression {
    /// An `array[index]` expression.
    Array(ArrayAccess),
    /// An expression accessing a range of an array.
    ArrayRange(ArrayRangeAccess),
    /// An expression accessing a field in a structure, e.g., `circuit_var.field`.
    Member(MemberAccess),
    /// Access to a tuple field using its position, e.g., `tuple.1`.
    Tuple(TupleAccess),
    /// Access to a member constant or a static function of a circuit.
    Static(StaticAccess),
}

impl fmt::Display for AccessExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AccessExpression::*;

        match self {
            Array(access) => access.fmt(f),
            ArrayRange(access) => access.fmt(f),
            Member(access) => access.fmt(f),
            Tuple(access) => access.fmt(f),
            Static(access) => access.fmt(f),
        }
    }
}

impl Node for AccessExpression {
    fn span(&self) -> &Span {
        use AccessExpression::*;

        match &self {
            Array(access) => access.span(),
            ArrayRange(access) => access.span(),
            Member(access) => access.span(),
            Tuple(access) => access.span(),
            Static(access) => access.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        use AccessExpression::*;

        match self {
            Array(access) => access.set_span(span),
            ArrayRange(access) => access.set_span(span),
            Member(access) => access.set_span(span),
            Tuple(access) => access.set_span(span),
            Static(access) => access.set_span(span),
        }
    }
}
