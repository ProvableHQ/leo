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

use crate::{TypeAssertionError, TypeVariablePairs};
use leo_static_check::{Type, TypeVariable};
use leo_typed::Span;

use serde::{Deserialize, Serialize};

/// A predicate that evaluates equality between two `Type`s.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TypeEquality {
    left: Type,
    right: Type,
    span: Span,
}

impl TypeEquality {
    ///
    /// Returns a `TypeEquality` predicate from given left and right `Types`s
    ///
    pub fn new(left: Type, right: Type, span: &Span) -> Self {
        Self {
            left,
            right,
            span: span.to_owned(),
        }
    }

    ///
    /// Substitutes the given `TypeVariable` for each `Types` in the `TypeEquality`.
    ///
    pub fn substitute(&mut self, variable: &TypeVariable, type_: &Type) {
        self.left.substitute(variable, type_);
        self.right.substitute(variable, type_);
    }

    ///
    /// Checks if the `self.left` == `self.right`.
    ///
    pub fn evaluate(&self) -> Result<(), TypeAssertionError> {
        if self.left.eq(&self.right) {
            Ok(())
        } else {
            Err(TypeAssertionError::equality_failed(&self.left, &self.right, &self.span))
        }
    }

    ///
    /// Returns the (type variable, type) pair from this assertion.
    ///
    pub fn pairs(&self) -> Result<TypeVariablePairs, TypeAssertionError> {
        TypeVariablePairs::new(self.left.to_owned(), self.right.to_owned(), &self.span)
    }
}
