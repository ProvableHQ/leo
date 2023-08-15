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

use leo_errors::Result;
use leo_span::{Span, Symbol};

use crate::{simple_node_impl, Node, NodeID};
use serde::{
    de::{
        Visitor,
        {self},
    },
    Deserialize,
    Deserializer,
    Serialize,
    Serializer,
};
use std::{
    collections::BTreeMap,
    fmt,
    hash::{Hash, Hasher},
};

/// An identifier in a program.
///
/// Attention - When adding or removing fields from this struct,
/// please remember to update its Serialize and Deserialize implementation
/// to reflect the new struct instantiation.
#[derive(Clone, Copy)]
pub struct Identifier {
    /// The symbol that the user wrote, e.g., `foo`.
    pub name: Symbol,
    /// A span locating where the identifier occurred in the source.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

simple_node_impl!(Identifier);

impl Identifier {
    /// Constructs a new identifier with `name` and `id` and a default span.
    pub fn new(name: Symbol, id: NodeID) -> Self {
        Self { name, span: Span::default(), id }
    }

    /// Check if the Identifier name matches the other name.
    pub fn matches(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl fmt::Debug for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.span == other.span
    }
}

impl Eq for Identifier {}

impl Hash for Identifier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.span.hash(state);
    }
}

impl Serialize for Identifier {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // Converts an element that implements Serialize into a string.
        fn to_json_string<E: Serialize, Error: serde::ser::Error>(element: &E) -> Result<String, Error> {
            serde_json::to_string(&element).map_err(|e| Error::custom(e.to_string()))
        }

        // Load the struct elements into a BTreeMap (to preserve serialized ordering of keys).
        let mut key: BTreeMap<String, String> = BTreeMap::new();
        key.insert("name".to_string(), self.name.to_string());
        key.insert("span".to_string(), to_json_string(&self.span)?);
        key.insert("id".to_string(), to_json_string(&self.id)?);

        // Convert the serialized object into a string for use as a key.
        serializer.serialize_str(&to_json_string(&key)?)
    }
}

impl<'de> Deserialize<'de> for Identifier {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct IdentifierVisitor;

        impl Visitor<'_> for IdentifierVisitor {
            type Value = Identifier;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string encoding the ast Identifier struct")
            }

            /// Implementation for recovering a string that serializes Identifier.
            fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
                // Converts a serialized string into an element that implements Deserialize.
                fn to_json_string<'a, D: Deserialize<'a>, Error: serde::de::Error>(
                    serialized: &'a str,
                ) -> Result<D, Error> {
                    serde_json::from_str::<'a>(serialized).map_err(|e| Error::custom(e.to_string()))
                }

                // Convert the serialized string into a BTreeMap to recover Identifier.
                let key: BTreeMap<String, String> = to_json_string(value)?;

                let name = match key.get("name") {
                    Some(name) => Symbol::intern(name),
                    None => return Err(E::custom("missing 'name' in serialized Identifier struct")),
                };

                let span: Span = match key.get("span") {
                    Some(span) => to_json_string(span)?,
                    None => return Err(E::custom("missing 'span' in serialized Identifier struct")),
                };

                let id: NodeID = match key.get("id") {
                    Some(id) => to_json_string(id)?,
                    None => return Err(E::custom("missing 'id' in serialized Identifier struct")),
                };

                Ok(Identifier { name, span, id })
            }
        }

        deserializer.deserialize_str(IdentifierVisitor)
    }
}
