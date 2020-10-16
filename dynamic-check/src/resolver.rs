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

use crate::{Program, ResolverError};
use leo_imports::ImportParser;
use leo_symbol_table::{ResolvedNode, SymbolTable};
use leo_typed::LeoTypedAst;

use serde_json;
use std::path::PathBuf;

/// A resolved abstract syntax tree without implicit types.
#[derive(Debug, Eq, PartialEq)]
pub struct LeoResolvedAst {
    pub resolved_ast: Program,
}

impl LeoResolvedAst {
    ///
    /// Creates a new `LeoResolvedAst` resolved syntax tree from a given `LeoTypedAst`
    /// typed syntax tree and main file path.
    ///
    pub fn new(ast: LeoTypedAst, path: PathBuf) -> Result<Self, ResolverError> {
        // Get program typed syntax tree representation.
        let program = ast.into_repr();

        // Get imported program typed syntax tree representations.
        let _imported_programs = ImportParser::parse(&program)?;

        // TODO (collinc97): Get input and state file typed syntax tree representations.

        // Create a new symbol table to track of program variables, circuits, and functions by name.
        let mut symbol_table = SymbolTable::new(None);

        // Pass 1: Check for circuit and function name collisions.
        symbol_table.pass_one(&program).map_err(|mut e| {
            // Set the filepath for the error stacktrace.
            e.set_path(path.clone());

            e
        })?;

        // Pass 2: Check circuit and function definitions for unknown types.
        symbol_table.pass_two(&program).map_err(|mut e| {
            // Set the filepath for the error stacktrace.
            e.set_path(path.clone());

            e
        })?;

        // Pass 3: Check statements for type errors.
        let resolved_ast = Program::resolve(&mut symbol_table, program).map_err(|mut e| {
            // Set the filepath for the error stacktrace.
            e.set_path(path);

            e
        })?;

        Ok(Self { resolved_ast })
    }

    ///
    /// Returns a reference to the inner resolved syntax tree representation.
    ///
    pub fn into_repr(self) -> Program {
        self.resolved_ast
    }

    ///
    /// Serializes the resolved syntax tree into a JSON string.
    ///
    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        Ok(serde_json::to_string_pretty(&self.resolved_ast)?)
    }

    ///
    /// Deserializes the JSON string into a resolved syntax tree.
    ///
    pub fn from_json_string(json: &str) -> Result<Self, serde_json::Error> {
        let resolved_ast: Program = serde_json::from_str(json)?;
        Ok(Self { resolved_ast })
    }
}
//
// /// A node in the `LeoResolvedAST`.
// ///
// /// This node and all of its children should not contain any implicit types.
// pub trait ResolvedNode {
//     /// The expected error type if the type resolution fails.
//     type Error;
//
//     /// The `leo-typed` AST node that is being type checked.
//     type UnresolvedNode;
//
//     ///
//     /// Returns a resolved AST representation given an unresolved AST representation and symbol table.
//     ///
//     fn resolve(table: &mut SymbolTable, unresolved: Self::UnresolvedNode) -> Result<Self, Self::Error>
//     where
//         Self: std::marker::Sized;
// }
