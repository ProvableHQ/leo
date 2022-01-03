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

use leo_errors::Span;

use serde::{Deserialize, Serialize};
// use serde::de::{Deserialize as SerDeserialize, Deserializer};
use std::fmt;

fn char_to_u32<S>(character: &char, serializer: S) -> Result<S::Ok, S::Error>
where
    S: ::serde::ser::Serializer,
{
    serializer.serialize_u32(*character as u32)
}

fn char_from_u32<'de, D>(deserializer: D) -> Result<char, D::Error>
where
    D: ::serde::de::Deserializer<'de>,
{
    let int = u32::deserialize(deserializer)?;
    std::char::from_u32(int).ok_or_else(|| ::serde::de::Error::custom("Failed to convert u32 to scalar char."))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Char {
    Scalar(
        #[serde(deserialize_with = "char_from_u32")]
        #[serde(serialize_with = "char_to_u32")]
        char,
    ),
    NonScalar(u32),
}

impl fmt::Display for Char {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Scalar(c) => write!(f, "{}", c),
            Self::NonScalar(c) => write!(f, "{}", c),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CharValue {
    pub character: Char,
    pub span: Span,
}

impl fmt::Display for CharValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.character)
    }
}

impl CharValue {
    pub fn set_span(&mut self, new_span: Span) {
        self.span = new_span;
    }

    pub fn span(&self) -> &Span {
        &self.span
    }
}
