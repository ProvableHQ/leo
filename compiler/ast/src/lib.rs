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

//! The abstract syntax tree (ast) for a Leo program.
//!
//! This module contains the [`Ast`] type, a wrapper around the [`Program`] type.
//! The [`Ast`] type is intended to be parsed and modified by different passes
//! of the Leo compiler. The Leo compiler can generate a set of R1CS constraints from any [`Ast`].

#![allow(ambiguous_glob_reexports)]

pub mod access;
pub use self::access::*;

pub mod r#struct;
pub use self::r#struct::*;

pub mod common;
pub use self::common::*;

pub mod expressions;
pub use self::expressions::*;

pub mod functions;
pub use self::functions::*;

pub mod groups;
pub use self::groups::*;

pub mod input;
pub use self::input::*;

pub mod mapping;
pub use self::mapping::*;

pub mod passes;
pub use self::passes::*;

pub mod program;
pub use self::program::*;

pub mod statement;
pub use self::statement::*;

pub mod types;
pub use self::types::*;

pub mod value;
pub use self::value::*;

pub use common::node::*;

use leo_errors::{AstError, Result};

/// The abstract syntax tree (AST) for a Leo program.
///
/// The [`Ast`] type represents a Leo program as a series of recursive data types.
/// These data types form a tree that begins from a [`Program`] type root.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Ast {
    pub ast: Program,
}

impl Ast {
    /// Creates a new AST from a given program tree.
    pub fn new(program: Program) -> Self {
        Self { ast: program }
    }

    /// Returns a reference to the inner program AST representation.
    pub fn as_repr(&self) -> &Program {
        &self.ast
    }

    pub fn into_repr(self) -> Program {
        self.ast
    }

    /// Serializes the ast into a JSON string.
    pub fn to_json_string(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(&self.ast).map_err(|e| AstError::failed_to_convert_ast_to_json_string(&e))?)
    }

    // Converts the ast into a JSON value.
    // Note that there is no corresponding `from_json_value` function
    // since we modify JSON values leaving them unable to be converted
    // back into Programs.
    pub fn to_json_value(&self) -> Result<serde_json::Value> {
        Ok(serde_json::to_value(&self.ast).map_err(|e| AstError::failed_to_convert_ast_to_json_value(&e))?)
    }

    /// Serializes the ast into a JSON file.
    pub fn to_json_file(&self, mut path: std::path::PathBuf, file_name: &str) -> Result<()> {
        path.push(file_name);
        let file = std::fs::File::create(&path).map_err(|e| AstError::failed_to_create_ast_json_file(&path, &e))?;
        let writer = std::io::BufWriter::new(file);
        Ok(serde_json::to_writer_pretty(writer, &self.ast)
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
        Ok(Self { ast })
    }

    /// Deserializes the JSON string into a ast from a file.
    pub fn from_json_file(path: std::path::PathBuf) -> Result<Self> {
        let data = std::fs::read_to_string(&path).map_err(|e| AstError::failed_to_read_json_file(&path, &e))?;
        Self::from_json_string(&data)
    }
}

impl AsRef<Program> for Ast {
    fn as_ref(&self) -> &Program {
        &self.ast
    }
}

/// Helper function to recursively filter keys from AST JSON
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

/// Helper function to normalize AST JSON into a form compatible with tgc.
/// This function will traverse the original JSON value and produce a new
/// one under the following rules:
/// 1. Remove empty object mappings from JSON arrays
/// 2. If there are two elements in a JSON array and one is an empty object
///     mapping and the other is not, then lift up the one that isn't
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
