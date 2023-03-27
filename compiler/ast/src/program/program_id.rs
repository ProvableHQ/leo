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

use crate::Identifier;

use core::fmt;
use serde::{de, de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::collections::BTreeMap;

/// An identifier for a program that is eventually deployed to the network.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ProgramId {
    /// The name of the program.
    pub name: Identifier,
    /// The network associated with the program.
    pub network: Identifier,
}

impl fmt::Display for ProgramId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.name, self.network)
    }
}

impl Serialize for ProgramId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // Converts an element that implements Serialize into a string.
        fn to_json_string<E: Serialize, Error: serde::ser::Error>(element: &E) -> Result<String, Error> {
            serde_json::to_string(&element).map_err(|e| Error::custom(e.to_string()))
        }

        // Load the struct elements into a BTreeMap (to preserve serialized ordering of keys).
        let mut key: BTreeMap<String, String> = BTreeMap::new();
        key.insert("name".to_string(), self.name.to_string());
        key.insert("network".to_string(), to_json_string(&self.network)?);

        // Convert the serialized object into a string for use as a key.
        serializer.serialize_str(&to_json_string(&key)?)
    }
}

impl<'de> Deserialize<'de> for ProgramId {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ProgramIdVisitor;

        impl Visitor<'_> for ProgramIdVisitor {
            type Value = ProgramId;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string encoding the ast ProgramId struct")
            }

            /// Implementation for recovering a string that serializes Identifier.
            fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
                // Converts a serialized string into an element that implements Deserialize.
                fn to_json_string<'a, D: Deserialize<'a>, Error: serde::de::Error>(
                    serialized: &'a str,
                ) -> Result<D, Error> {
                    serde_json::from_str::<'a>(serialized).map_err(|e| Error::custom(e.to_string()))
                }

                // Convert the serialized string into a BTreeMap to recover ProgramId.
                let key: BTreeMap<String, String> = to_json_string(value)?;

                let name: Identifier = match key.get("name") {
                    Some(name) => to_json_string(name)?,
                    None => return Err(E::custom("missing 'name' in serialized ProgramId struct")),
                };

                let network: Identifier = match key.get("network") {
                    Some(network) => to_json_string(network)?,
                    None => return Err(E::custom("missing 'network' in serialized ProgramId struct")),
                };

                Ok(ProgramId { name, network })
            }
        }

        deserializer.deserialize_str(ProgramIdVisitor)
    }
}
