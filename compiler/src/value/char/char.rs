// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use crate::{errors::FieldError, FieldType};
use leo_ast::Span;
use snarkvm_fields::PrimeField;

/// A char
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Char<F: PrimeField> {
    pub character: char,
    pub field: FieldType<F>,
}

impl<F: PrimeField> Char<F> {
    pub fn constant(character: char, field: String, span: &Span) -> Result<Self, FieldError> {
        Ok(Self {
            character,
            field: FieldType::constant(field, span)?,
        })
    }
}

impl<F: PrimeField + std::fmt::Display> std::fmt::Display for Char<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.character)
    }
}
