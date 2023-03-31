// Copyright (C) 2019-2023 Aleo Systems Inc.
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

impl ParserContext<'_> {
    /// Returns a [`ParsedInputFile`] struct filled with the data acquired in the file.
    pub(crate) fn parse_input_file(&mut self) -> Result<InputAst> {
        // Allow underscores in identifiers for input record declarations.
        self.allow_identifier_underscores = true;
        let mut sections = Vec::new();

        while self.has_next() {
            if self.check(&Token::LeftSquare) {
                sections.push(self.parse_section()?);
            } else {
                return Err(ParserError::unexpected_token(self.token.token.clone(), self.token.span).into());
            }
        }

        // Do not allow underscores in identifiers outside of input files.
        self.allow_identifier_underscores = false;

        Ok(InputAst { sections })
    }

    /// Parses particular section in the Input or State file.
    /// `
    /// [<identifier>]
    /// <...definition>
    /// `
    /// Returns [`Section`].
    fn parse_section(&mut self) -> Result<Section> {
        self.expect(&Token::LeftSquare)?;
        let section = self.expect_identifier()?;
        self.expect(&Token::RightSquare)?;

        let mut definitions = Vec::new();
        while let Token::Constant | Token::Public | Token::Identifier(_) = self.token.token {
            definitions.push(self.parse_input_definition()?);
        }

        Ok(Section { name: section.name, span: section.span, definitions })
    }

    /// Parses a single parameter definition:
    /// `<identifier> : <type> = <expression>;`
    /// Returns [`Definition`].
    fn parse_input_definition(&mut self) -> Result<Definition> {
        let mode = self.parse_mode()?;

        let name = self.expect_identifier()?;
        self.expect(&Token::Colon)?;
        let (type_, span) = self.parse_type()?;
        self.expect(&Token::Assign)?;
        let value = self.parse_unary_expression()?;
        self.expect(&Token::Semicolon)?;

        Ok(Definition { mode, name, type_, value, span })
    }
}
