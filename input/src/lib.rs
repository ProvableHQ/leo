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
use std::{fs, path::Path};

pub struct LeoInputParser;

impl LeoInputParser {
    /// Reads in the given file path into a string.
    pub fn load_file(file_path: &Path) -> Result<String, InputParserError> {
        Ok(fs::read_to_string(file_path).map_err(|_| InputParserError::FileReadError(file_path.to_owned()))?)
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
