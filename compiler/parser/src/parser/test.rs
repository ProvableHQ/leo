// Copyright (C) 2019-2024 Aleo Systems Inc.
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

use super::*;

use leo_errors::{ParserError, Result};

impl<N: Network> ParserContext<'_, N> {
    /// Parses a test file.
    fn parse_test(&mut self) -> Result<Test> {
        // Initialize storage for the components of the test file
        let mut consts: Vec<(Symbol, ConstDeclaration)> = Vec::new();
        let mut functions = Vec::new();
        let mut structs: Vec<(Symbol, Composite)> = Vec::new();
        let mut mappings: Vec<(Symbol, Mapping)> = Vec::new();
        // Parse the components of the test file.
        while self.has_next() {
            match &self.token.token {
                Token::Const => {
                    let declaration = self.parse_const_declaration_statement()?;
                    consts.push((Symbol::intern(&declaration.place.to_string()), declaration));
                }
                Token::Struct | Token::Record => {
                    let (id, struct_) = self.parse_struct()?;
                    structs.push((id, struct_));
                }
                Token::Mapping => {
                    let (id, mapping) = self.parse_mapping()?;
                    mappings.push((id, mapping));
                }
                Token::At | Token::Async | Token::Function | Token::Transition | Token::Inline | Token::Interpret => {
                    let (id, function) = self.parse_function()?;
                    functions.push((id, function));
                }
                _ => {
                    return Err(Self::unexpected_item(&self.token, &[
                        Token::Const,
                        Token::Struct,
                        Token::Record,
                        Token::Mapping,
                        Token::At,
                        Token::Async,
                        Token::Function,
                        Token::Transition,
                        Token::Inline,
                    ])
                    .into());
                }
            }
        }

        Ok(Test { consts, functions, structs, mappings })
    }
}

use leo_span::{Symbol, sym};
