#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate thiserror;

pub mod errors;
pub use errors::*;

mod ast;
pub mod common;
pub mod definitions;
pub mod expressions;
pub mod files;
pub mod parameters;
pub mod sections;
pub mod tables;
pub mod types;
pub mod values;

use from_pest::FromPest;
use std::{fs, path::PathBuf};

pub struct LeoInputsParser;

impl LeoInputsParser {
    /// Reads in the given file path into a string.
    pub fn load_file(file_path: &PathBuf) -> Result<String, InputParserError> {
        Ok(fs::read_to_string(file_path).map_err(|_| InputParserError::FileReadError(file_path.clone()))?)
    }

    /// Parses the input file and constructs a syntax tree.
    pub fn parse_file(input_file: &str) -> Result<files::File, InputParserError> {
        // Parse the file using leo-input.pest
        let mut file = ast::parse(input_file)?;

        // Build the abstract syntax tree
        let syntax_tree = files::File::from_pest(&mut file).map_err(|_| InputParserError::SyntaxTreeError)?;

        Ok(syntax_tree)
    }
}
