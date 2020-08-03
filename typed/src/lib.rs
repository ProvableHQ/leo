//! A typed syntax tree is represented as a `Program` and consists of import, circuit, and function definitions.
//! Each defined type consists of typed statements and expressions.

pub mod circuits;
pub use self::circuits::*;

pub mod common;
pub use self::common::*;

pub mod errors;
pub use self::errors::*;

pub mod expression;
pub use self::expression::*;

pub mod functions;
pub use self::functions::*;

pub mod imports;
pub use self::imports::*;

pub mod input;
pub use self::input::*;

pub mod macros;
pub use self::macros::*;

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
