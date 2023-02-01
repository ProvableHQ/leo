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

use crate::{normalize_json_value, remove_key_from_json, Expression, Struct, Type};

use super::*;
use leo_errors::{AstError, Result};

/// Input data which includes [`ProgramInput`].
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputData {
    pub program_input: ProgramInput,
}

impl InputData {
    /// Serializes the ast into a JSON string.
    pub fn to_json_string(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(&self).map_err(|e| AstError::failed_to_convert_ast_to_json_string(&e))?)
    }
}

/// A raw unprocessed input or state file data. Used for future conversion
/// into [`ProgramInput`].
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputAst {
    pub sections: Vec<Section>,
}

impl InputAst {
    /// Returns all values of the input AST for execution with `leo run`.
    pub fn program_inputs(&self, program_name: &str, structs: IndexMap<Symbol, Struct>) -> Vec<String> {
        self.sections
            .iter()
            .filter(|section| section.name() == program_name)
            .flat_map(|section| {
                section.definitions.iter().map(|definition| match &definition.type_ {
                    // Handle case where the input may be record.
                    Type::Identifier(identifier) => {
                        match structs.get(&identifier.name) {
                            // TODO: Better error handling.
                            None => panic!(
                                "Input error: A struct or record declaration does not exist for {}.",
                                identifier.name
                            ),
                            Some(struct_) => match struct_.is_record {
                                false => definition.value.to_string(),
                                true => match &definition.value {
                                    // Print out the record interface with visibility.
                                    Expression::Struct(struct_expression) => struct_expression.to_record_string(),
                                    _ => panic!("Input error: Expected a struct expression."),
                                },
                            },
                        }
                    }
                    _ => definition.value.to_string(),
                })
            })
            .collect::<Vec<_>>()
    }

    /// Serializes the `Input` into a JSON Value.
    pub fn to_json_value(&self) -> Result<serde_json::Value> {
        Ok(serde_json::to_value(self).map_err(|e| AstError::failed_to_convert_ast_to_json_value(&e))?)
    }

    /// Serializes the input into a JSON file.
    pub fn to_json_file(&self, mut path: std::path::PathBuf, file_name: &str) -> Result<()> {
        path.push(file_name);
        let file = std::fs::File::create(&path).map_err(|e| AstError::failed_to_create_ast_json_file(&path, &e))?;
        let writer = std::io::BufWriter::new(file);
        Ok(serde_json::to_writer_pretty(writer, &self)
            .map_err(|e| AstError::failed_to_write_ast_to_json_file(&path, &e))?)
    }

    /// Serializes the `Input` into a JSON value and removes keys from object mappings before writing to a file.
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
}
