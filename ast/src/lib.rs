#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate thiserror;

mod ast;

pub mod access;
pub mod circuits;
pub mod common;
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

pub struct LeoParser;

impl LeoParser {
    /// Loads the Leo code as a string from the given file path.
    pub fn load_file(file_path: &PathBuf) -> Result<String, ParserError> {
        Ok(fs::read_to_string(file_path).map_err(|_| ParserError::FileReadError(file_path.clone()))?)
    }

    /// Parses the Leo program string and constructs an abstract syntax tree.
    pub fn parse_file<'a>(file_path: &'a PathBuf, program_string: &'a str) -> Result<files::File<'a>, ParserError> {
        // Parse the file using leo.pest
        let mut file = ast::parse(program_string)
            .map_err(|error| ParserError::from(error.with_path(file_path.to_str().unwrap())))?;

        // Build the abstract syntax tree
        let syntax_tree = files::File::from_pest(&mut file).map_err(|_| ParserError::SyntaxTreeError)?;
        log::debug!("{:#?}", syntax_tree);

        Ok(syntax_tree)
    }

    /// Serializes a given abstract syntax tree into a JSON string.
    pub fn to_json_string(syntax_tree: &files::File) -> Result<String, ParserError> {
        Ok(serde_json::to_string_pretty(syntax_tree)?)
    }
}
