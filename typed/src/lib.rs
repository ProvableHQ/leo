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

//! A typed syntax tree is represented as a `Program` and consists of import, circuit, and function definitions.
//! Each defined type consists of typed statements and expressions.

pub mod annotation;
pub use self::annotation::*;

pub mod circuits;
pub use self::circuits::*;

pub mod common;
pub use self::common::*;

pub mod console;
pub use self::console::*;

pub mod errors;
pub use self::errors::*;

pub mod expression;
pub use self::expression::*;

pub mod functions;
pub use self::functions::*;

pub mod groups;
pub use self::groups::*;

pub mod imports;
pub use self::imports::*;

pub mod input;
pub use self::input::*;

pub mod program;
pub use self::program::*;

pub mod statements;
pub use self::statements::*;

pub mod types;
pub use self::types::*;

use leo_ast::LeoAst;

use serde_json;

#[derive(Debug, Eq, PartialEq)]
pub struct LeoTypedAst {
    typed_ast: Program,
}

impl LeoTypedAst {
    /// Creates a new typed syntax tree from a given program name and abstract syntax tree.
    pub fn new<'ast>(program_name: &str, ast: &LeoAst<'ast>) -> Self {
        Self {
            typed_ast: Program::from(program_name, ast.as_repr()),
        }
    }

    /// Returns a reference to the inner typed syntax tree representation.
    pub fn into_repr(self) -> Program {
        self.typed_ast
    }

    /// Serializes the typed syntax tree into a JSON string.
    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        Ok(serde_json::to_string_pretty(&self.typed_ast)?)
    }

    /// Deserializes the JSON string into a typed syntax tree.
    pub fn from_json_string(json: &str) -> Result<Self, serde_json::Error> {
        let typed_ast: Program = serde_json::from_str(json)?;
        Ok(Self { typed_ast })
    }
}
