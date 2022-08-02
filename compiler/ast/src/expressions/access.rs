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

use crate::{Expression, Identifier, PositiveNumber, Type};

use serde::{Deserialize, Serialize};
use std::fmt;

/// An access expressions, extracting a smaller part out of a whole.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessExpression {
    // /// An `array[index]` expression.
    // Array(ArrayAccess),
    // /// An expression accessing a range of an array.
    // ArrayRange(ArrayRangeAccess),
    /// Access to an associated variable of a circuit e.g `u8::MAX`.
    AssociatedConstant(AssociatedConstant),
    /// Access to an associated function of a circuit e.g `Pedersen64::hash()`.
    AssociatedFunction(AssociatedFunction),
    /// An expression accessing a field in a structure, e.g., `circuit_var.field`.
    Member(MemberAccess),
    /// Access to a tuple field using its position, e.g., `tuple.1`.
    Tuple(TupleAccess),
}

impl fmt::Display for AccessExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AccessExpression::*;

        match self {
            AssociatedConstant(access) => access.fmt(f),
            AssociatedFunction(access) => access.fmt(f),
            Member(access) => access.fmt(f),
            Tuple(access) => access.fmt(f),
        }
    }
}

/// An access expression to an circuit constant., e.g. `u8::MAX`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssociatedConstant {
    /// The inner circuit type.
    pub ty: Type,
    /// The circuit constant that is being accessed.
    pub name: Identifier,
}

impl fmt::Display for AssociatedConstant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}::{}", self.ty, self.name)
    }
}

/// An access expression to an associated function in a circuit, e.g.`Pedersen64::hash()`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssociatedFunction {
    /// The inner circuit type.
    pub ty: Type,
    /// The static circuit member function that is being accessed.
    pub name: Identifier,
    /// The arguments passed to the function `name`.
    pub args: Vec<Expression>,
}

impl fmt::Display for AssociatedFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}::{}", self.ty, self.name)
    }
}

/// A circuit member access expression `inner.name` to some structure with *named members*.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemberAccess {
    /// The inner circuit that is being accessed.
    pub inner: Box<Expression>,
    /// The name of the circuit member to access.
    pub name: Identifier,
}

impl fmt::Display for MemberAccess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.inner, self.name)
    }
}

/// A tuple access expression, e.g., `tuple.index`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TupleAccess {
    /// An expression evaluating to some tuple type, e.g., `(5, 2)`.
    pub tuple: Box<Expression>,
    /// The index to access in the tuple expression. E.g., `0` for `(5, 2)` would yield `5`.
    pub index: PositiveNumber,
}

impl fmt::Display for TupleAccess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.tuple, self.index)
    }
}
