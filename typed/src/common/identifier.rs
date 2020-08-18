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
use leo_ast::{
    annotations::AnnotationArgument,
    common::Identifier as AstIdentifier,
    imports::PackageName as AstPackageName,
};
use leo_input::common::Identifier as InputAstIdentifier;

use serde::{
    de::{self, Visitor},
    Deserialize,
    Deserializer,
    Serialize,
    Serializer,
};
use std::{collections::BTreeMap, fmt};

/// An identifier in the constrained program.
///
/// Attention - When adding or removing fields from this struct,
/// please remember to update it's Serialize and Deserialize implementation
/// to reflect the new struct instantiation.
#[derive(Clone, Hash)]
pub struct Identifier {
    pub name: String,
    pub span: Span,
}

impl Identifier {
    pub fn is_self(&self) -> bool {
        self.name == "Self" || self.name == "self"
    }
}

impl<'ast> From<AstIdentifier<'ast>> for Identifier {
    fn from(identifier: AstIdentifier<'ast>) -> Self {
        Self {
            name: identifier.value,
            span: Span::from(identifier.span),
        }
    }
}

impl<'ast> From<AstPackageName<'ast>> for Identifier {
    fn from(name: AstPackageName<'ast>) -> Self {
        Self {
            name: name.value,
            span: Span::from(name.span),
        }
    }
}

impl<'ast> From<InputAstIdentifier<'ast>> for Identifier {
    fn from(identifier: InputAstIdentifier<'ast>) -> Self {
        Self {
            name: identifier.value,
            span: Span::from(identifier.span),
        }
    }
}

impl<'ast> From<AnnotationArgument<'ast>> for Identifier {
    fn from(argument: AnnotationArgument<'ast>) -> Self {
        Self {
            name: argument.value,
            span: Span::from(argument.span),
        }
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
        self.name == other.name
    }
}

impl Eq for Identifier {}

impl Serialize for Identifier {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // Converts an element that implements Serialize into a string.
        fn to_json_string<E: Serialize, Error: serde::ser::Error>(element: &E) -> Result<String, Error> {
            serde_json::to_string(&element).map_err(|e| Error::custom(e.to_string()))
        }

        // Load the struct elements into a BTreeMap (to preserve serialized ordering of keys).
        let mut key: BTreeMap<String, String> = BTreeMap::new();
        key.insert("name".to_string(), self.name.clone());
        key.insert("span".to_string(), to_json_string(&self.span)?);

        // Convert the serialized object into a string for use as a key.
        serializer.serialize_str(&to_json_string(&key)?)
    }
}

impl<'de> Deserialize<'de> for Identifier {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct IdentifierVisitor;

        impl<'de> Visitor<'de> for IdentifierVisitor {
            type Value = Identifier;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string encoding the typed Identifier struct")
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
                    Some(name) => name.clone(),
                    None => return Err(E::custom("missing 'name' in serialized Identifier struct")),
                };

                let span: Span = match key.get("span") {
                    Some(span) => to_json_string(span)?,
                    None => return Err(E::custom("missing 'span' in serialized Identifier struct")),
                };

                Ok(Identifier { name, span })
            }
        }

        deserializer.deserialize_str(IdentifierVisitor)
    }
}
