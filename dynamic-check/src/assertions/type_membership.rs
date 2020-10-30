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

use crate::TypeAssertionError;
use leo_symbol_table::{Type, TypeVariable};
use leo_typed::Span;

use serde::{Deserialize, Serialize};

/// A predicate that evaluates to true if the given type is equal to a member in the set vector of types.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TypeMembership {
    given: Type,
    set: Vec<Type>,
    span: Span,
}

impl TypeMembership {
    ///
    /// Returns a `TypeMembership` predicate from given and set `Type`s.
    ///
    pub fn new(given: Type, set: Vec<Type>, span: &Span) -> Self {
        Self {
            given,
            set,
            span: span.to_owned(),
        }
    }

    ///
    /// Substitutes the given `TypeVariable` for each `Type` in the `TypeMembership`.
    ///
    pub fn substitute(&mut self, variable: &TypeVariable, type_: &Type) {
        self.given.substitute(variable, type_)
    }

    ///
    /// Returns true if the given type is equal to a member of the set.
    ///
    pub fn evaluate(&self) -> Result<(), TypeAssertionError> {
        if self.set.contains(&self.given) {
            Ok(())
        } else {
            Err(TypeAssertionError::membership_failed(
                &self.given,
                &self.set,
                &self.span,
            ))
        }
    }

    ///
    /// Returns the self.span.
    ///
    pub fn span(&self) -> &Span {
        &self.span
    }
}
