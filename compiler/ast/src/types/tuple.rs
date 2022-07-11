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

use crate::Type;
use leo_errors::{AstError, Result};
use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::{fmt, ops::Deref};

/// A type list of at least two types.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Tuple(Vec<Type>);

impl Tuple {
    /// Returns a new `Type::Tuple` enumeration.
    pub fn try_new(elements: Vec<Type>, span: Span) -> Result<Type> {
        match elements.len() {
            0 => Err(AstError::empty_tuple(span).into()),
            1 => Err(AstError::one_element_tuple(span).into()),
            _ => Ok(Type::Tuple(Tuple(elements))),
        }
    }
}

impl Deref for Tuple {
    type Target = Vec<Type>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({})",
            self.0.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")
        )
    }
}
