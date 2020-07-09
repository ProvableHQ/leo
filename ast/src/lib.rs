#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate thiserror;

pub mod errors;
pub use errors::*;

pub mod access;
mod ast;
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

use from_pest::FromPest;
use std::{fs, path::PathBuf};

pub struct LeoParser;

impl LeoParser {
    /// Reads in the given file path into a string.
    pub fn load_file(file_path: &PathBuf) -> Result<String, ParserError> {
        Ok(fs::read_to_string(file_path).map_err(|_| ParserError::FileReadError(file_path.clone()))?)
    }

    /// Parses the input file and constructs a syntax tree.
    pub fn parse_file<'a>(file_path: &'a PathBuf, input_file: &'a str) -> Result<files::File<'a>, ParserError> {
        // Parse the file using leo.pest
        let mut file =
            ast::parse(input_file).map_err(|error| ParserError::from(error.with_path(file_path.to_str().unwrap())))?;

        // Build the abstract syntax tree
        let syntax_tree = files::File::from_pest(&mut file).map_err(|_| ParserError::SyntaxTreeError)?;
        log::debug!("{:#?}", syntax_tree);

        Ok(syntax_tree)
    }
}
