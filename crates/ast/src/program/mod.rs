// Copyright (C) 2019-2026 Provable Inc.
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

//! A Leo program consists of import statements and program scopes.

mod program_scope;
pub use program_scope::*;

use leo_span::Symbol;

use crate::{Module, ProgramId, Stub};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fmt;
/// Stores the Leo program abstract syntax tree.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Program {
    /// A map from module paths to module definitions.
    #[serde(with = "module_map")]
    pub modules: IndexMap<Vec<Symbol>, Module>,
    /// A map from import names (including the `.aleo` if present) to import definitions.
    pub imports: IndexMap<Symbol, ProgramId>,
    /// A map from program stub names to program stub scopes.
    pub stubs: IndexMap<Symbol, Stub>,
    /// A map from program names to program scopes.
    pub program_scopes: IndexMap<Symbol, ProgramScope>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (_, stub) in self.stubs.iter() {
            writeln!(f, "{stub}")?;
        }
        for (_, module) in self.modules.iter() {
            writeln!(f, "{module}")?;
        }
        for (_, import_id) in self.imports.iter() {
            writeln!(f, "import {import_id};")?;
        }
        for (_, program_scope) in self.program_scopes.iter() {
            writeln!(f, "{program_scope}")?;
        }
        Ok(())
    }
}

impl Default for Program {
    /// Constructs an empty program node.
    fn default() -> Self {
        Self {
            modules: IndexMap::new(),
            imports: IndexMap::new(),
            stubs: IndexMap::new(),
            program_scopes: IndexMap::new(),
        }
    }
}

/// Helper function to recursively filter keys from AST JSON.
pub fn remove_key_from_json(value: serde_json::Value, key: &str) -> serde_json::Value {
    match value {
        serde_json::Value::Object(map) => serde_json::Value::Object(
            map.into_iter().filter(|(k, _)| k != key).map(|(k, v)| (k, remove_key_from_json(v, key))).collect(),
        ),
        serde_json::Value::Array(values) => {
            serde_json::Value::Array(values.into_iter().map(|v| remove_key_from_json(v, key)).collect())
        }
        _ => value,
    }
}

/// Helper function to normalize AST JSON into a form compatible with TGC.
///
/// This function traverses the original JSON value and produces a new one under the following rules:
///
/// 1. Remove empty object mappings from JSON arrays.
/// 2. If a JSON array contains exactly two elements and one is an empty object
///    mapping while the other is not, lift the non-empty element.
pub fn normalize_json_value(value: serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Array(vec) => {
            let orig_length = vec.len();

            let mut new_vec: Vec<serde_json::Value> = vec
                .into_iter()
                .filter(|v| !matches!(v, serde_json::Value::Object(map) if map.is_empty()))
                .map(normalize_json_value)
                .collect();

            if orig_length == 2 && new_vec.len() == 1 {
                new_vec.pop().unwrap()
            } else {
                serde_json::Value::Array(new_vec)
            }
        }
        serde_json::Value::Object(map) => {
            serde_json::Value::Object(map.into_iter().map(|(k, v)| (k, normalize_json_value(v))).collect())
        }
        _ => value,
    }
}

/// Serde helpers for `IndexMap<Vec<Symbol>, V>` maps keyed by module paths.
///
/// JSON object keys must be strings, so the `Vec<Symbol>` path is joined into a single
/// `::`-separated string when serializing and split back when deserializing.
pub(crate) mod module_map {
    use leo_span::{Symbol, with_session_globals};

    use indexmap::IndexMap;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S, V>(map: &IndexMap<Vec<Symbol>, V>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        V: Serialize,
    {
        let joined: IndexMap<String, &V> = with_session_globals(|globals| {
            map.iter()
                .map(|(path, value)| {
                    let key = path.iter().map(|sym| sym.as_str(globals, str::to_owned)).collect::<Vec<_>>().join("::");
                    (key, value)
                })
                .collect()
        });
        joined.serialize(serializer)
    }

    pub fn deserialize<'de, D, V>(deserializer: D) -> Result<IndexMap<Vec<Symbol>, V>, D::Error>
    where
        D: Deserializer<'de>,
        V: Deserialize<'de>,
    {
        Ok(IndexMap::<String, V>::deserialize(deserializer)?
            .into_iter()
            .map(|(path, value)| (path.split("::").map(Symbol::intern).collect(), value))
            .collect())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use leo_span::create_session_if_not_set_then;

        #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
        struct Wrapper(#[serde(with = "super")] IndexMap<Vec<Symbol>, u32>);

        #[test]
        fn round_trips_single_and_multi_segment_keys() {
            create_session_if_not_set_then(|_| {
                let mut map = IndexMap::new();
                map.insert(vec![Symbol::intern("utils")], 1);
                map.insert(vec![Symbol::intern("utils"), Symbol::intern("math")], 2);
                let wrapper = Wrapper(map);

                let json = serde_json::to_value(&wrapper).unwrap();
                assert_eq!(json, serde_json::json!({ "utils": 1, "utils::math": 2 }));

                let restored: Wrapper = serde_json::from_value(json).unwrap();
                assert_eq!(restored, wrapper);
            });
        }
    }
}
