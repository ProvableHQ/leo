#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate thiserror;

mod ast;

pub mod access;
pub mod annotations;
pub mod circuits;
pub mod common;
pub mod definitions;
pub mod expressions;
pub mod files;
pub mod functions;
pub mod imports;
pub mod macros;
pub mod operations;
pub mod statements;
pub mod types;
pub mod values;

pub mod errors;
pub use errors::*;

pub(crate) mod span;
pub(crate) use span::*;

use from_pest::FromPest;
use std::{fs, path::PathBuf};

pub struct LeoAst<'ast> {
    ast: files::File<'ast>,
}

impl<'ast> LeoAst<'ast> {
    /// Creates a new abstract syntax tree given the file path.
    pub fn new(file_path: &'ast PathBuf, program_string: &'ast str) -> Result<Self, ParserError> {
        // TODO (howardwu): Turn this check back on after fixing the testing module.
        // assert_eq!(program_string, fs::read_to_string(file_path).map_err(|_| ParserError::FileReadError(file_path.clone()))?);

        // Parse the file using leo.pest
        let file = &mut ast::parse(&program_string)
            .map_err(|error| ParserError::from(error.with_path(file_path.to_str().unwrap())))?;

        // Builds the abstract syntax tree using pest derivation.
        let ast = files::File::<'ast>::from_pest(file).map_err(|_| ParserError::SyntaxTreeError)?;
        log::debug!("{:#?}", ast);

        Ok(Self { ast })
    }

    // TODO (howardwu): Remove this in favor of a dedicated file loader to verify checksums
    //  and maintain a global cache of program strings during the compilation process.
    /// Loads the Leo code as a string from the given file path.
    pub fn load_file(file_path: &'ast PathBuf) -> Result<String, ParserError> {
        Ok(fs::read_to_string(file_path).map_err(|_| ParserError::FileReadError(file_path.clone()))?)
    }

    /// Returns a reference to the inner abstract syntax tree representation.
    pub fn as_repr(&self) -> &files::File<'ast> {
        &self.ast
    }

    /// Serializes the abstract syntax tree into a JSON string.
    pub fn to_json_string(&self) -> Result<String, ParserError> {
        Ok(serde_json::to_string_pretty(&self.ast)?)
    }
}
