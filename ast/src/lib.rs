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

    /// Serializes and stores a given syntax tree in the output file.
    pub fn store_syntax_tree<'a>(syntax_tree: files::File<'a>, output_file: &'a str) -> Result<(), ParserError> {
        // Serialize and store the syntax tree to the given filepath.
        let serialized_syntax_tree = serde_json::to_string(&syntax_tree).unwrap();
        println!("serialized = {}", serialized_syntax_tree);
        fs::write(output_file, serialized_syntax_tree)?;

        Ok(())
    }
}
