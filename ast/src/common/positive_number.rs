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

use crate::Span;
use leo_grammar::values::PositiveNumber as GrammarPositiveNumber;
use leo_input::values::PositiveNumber as InputPositiveNumber;

use serde::{Deserialize, Serialize};
use std::{
    fmt,
    hash::{Hash, Hasher},
};

/// A number string guaranteed to be positive by the pest grammar.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositiveNumber {
    pub value: String,
    pub span: Span,
}

/// Create a new [`PositiveNumber`] from a [`GrammarPositiveNumber`] in a Leo program file.
impl<'ast> From<GrammarPositiveNumber<'ast>> for PositiveNumber {
    fn from(array: GrammarPositiveNumber<'ast>) -> Self {
        Self {
            value: array.value,
            span: Span::from(array.span),
        }
    }
}

/// Create a new [`PositiveNumber`] from an [`InputPositiveNumber`]  in a Leo input file.
impl<'ast> From<InputPositiveNumber<'ast>> for PositiveNumber {
    fn from(array: InputPositiveNumber<'ast>) -> Self {
        Self {
            value: array.value,
            span: Span::from(array.span),
        }
    }
}

impl fmt::Display for PositiveNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// Compares two positive numbers and ignores `Span`s.
impl PartialEq for PositiveNumber {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl Eq for PositiveNumber {}

impl Hash for PositiveNumber {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }
}
