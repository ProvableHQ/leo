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

//! Provides custom serialize/deserialize implementations for `Span`.

use crate::Span;

use serde::{
    de::{MapAccess, Visitor},
    ser::SerializeMap,
    Deserializer,
    Serializer,
};
use std::fmt;

/// The AST contains a few tuple-like enum variants that contain spans.
/// #[derive(Serialize, Deserialize)] outputs these fields as anonmyous
/// mappings, which makes them difficult to remove from the JSON AST.
/// This function provides a custom serialization that maps the keyword
/// `span` to the span information.
pub fn serialize<S: Serializer>(span: &Span, serializer: S) -> Result<S::Ok, S::Error> {
    let mut map = serializer.serialize_map(Some(1))?;
    map.serialize_entry("span", span)?;
    map.end()
}

/// Custom deserialization to enable removing spans from enums.
pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Span, D::Error> {
    deserializer.deserialize_map(SpanMapVisitor)
}

/// This visitor is used by the deserializer to unwrap mappings
/// and extract span information.
struct SpanMapVisitor;

impl<'de> Visitor<'de> for SpanMapVisitor {
    type Value = Span;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Mapping from `span` keyword to span information")
    }

    fn visit_map<M: MapAccess<'de>>(self, mut access: M) -> Result<Self::Value, M::Error> {
        let (_, value): (String, Span) = access.next_entry()?.unwrap();
        Ok(value)
    }
}
