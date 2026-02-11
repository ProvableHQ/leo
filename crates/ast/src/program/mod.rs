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

use leo_errors::{AstError, Result};
use leo_span::Symbol;

use crate::{Module, ProgramId, Stub};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fmt;
/// Stores the Leo program abstract syntax tree.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Program {
    /// A map from module paths to module definitions.
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

impl Program {
    /// Serializes the ast into a JSON string.
    pub fn to_json_string(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(&self).map_err(|e| AstError::failed_to_convert_ast_to_json_string(&e))?)
    }

    // Converts the ast into a JSON value.
    // Note that there is no corresponding `from_json_value` function
    // since we modify JSON values leaving them unable to be converted
    // back into Programs.
    pub fn to_json_value(&self) -> Result<serde_json::Value> {
        Ok(serde_json::to_value(self).map_err(|e| AstError::failed_to_convert_ast_to_json_value(&e))?)
    }

    /// Serializes the ast into a JSON file.
    pub fn to_json_file(&self, mut path: std::path::PathBuf, file_name: &str) -> Result<()> {
        path.push(file_name);
        let file = std::fs::File::create(&path).map_err(|e| AstError::failed_to_create_ast_json_file(&path, &e))?;
        let writer = std::io::BufWriter::new(file);
        Ok(serde_json::to_writer_pretty(writer, &self)
            .map_err(|e| AstError::failed_to_write_ast_to_json_file(&path, &e))?)
    }

    /// Serializes the ast into a JSON value and removes keys from object mappings before writing to a file.
    pub fn to_json_file_without_keys(
        &self,
        mut path: std::path::PathBuf,
        file_name: &str,
        excluded_keys: &[&str],
    ) -> Result<()> {
        path.push(file_name);
        let file = std::fs::File::create(&path).map_err(|e| AstError::failed_to_create_ast_json_file(&path, &e))?;
        let writer = std::io::BufWriter::new(file);

        let mut value = self.to_json_value().unwrap();
        for key in excluded_keys {
            value = remove_key_from_json(value, key);
        }
        value = normalize_json_value(value);

        Ok(serde_json::to_writer_pretty(writer, &value)
            .map_err(|e| AstError::failed_to_write_ast_to_json_file(&path, &e))?)
    }

    /// Deserializes the JSON string into a ast.
    pub fn from_json_string(json: &str) -> Result<Self> {
        let ast: Program = serde_json::from_str(json).map_err(|e| AstError::failed_to_read_json_string_to_ast(&e))?;
        Ok(ast)
    }

    /// Deserializes the JSON string into a ast from a file.
    pub fn from_json_file(path: std::path::PathBuf) -> Result<Self> {
        let data = std::fs::read_to_string(&path).map_err(|e| AstError::failed_to_read_json_file(&path, &e))?;
        Self::from_json_string(&data)
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
