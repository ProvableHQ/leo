// Copyright (C) 2019-2022 Aleo Systems Inc.
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
use leo_span::sym;

impl ParserContext<'_> {
    /// Returns a [`ParsedInputFile`] struct filled with the data acquired in the file.
    pub fn parse_input(&mut self) -> Result<ParsedInputFile> {
        let mut sections = Vec::new();
        let mut is_public = true;

        while self.has_next() {
            let token = self.peek()?;
            if matches!(token.token, Token::LeftSquare) {
                // For visibility modifiers: [[public]] or [[private]]
                if self.peek_next()?.token == Token::LeftSquare {
                    is_public = self.parse_visibility()?;
                }

                let mut section = self.parse_section()?;
                section.is_public = is_public;
                sections.push(section);
            } else {
                return Err(ParserError::unexpected_token(token.token.clone(), &token.span).into());
            }
        }

        Ok(ParsedInputFile { sections })
    }

    /// Parses visibility tables in the State or Input file.
    /// Expects: [[<public>]] or [[<private>]], returns true
    /// if visibility is set to public, returns else otherwise.
    pub fn parse_visibility(&mut self) -> Result<bool> {
        self.expect(Token::LeftSquare)?;
        self.expect(Token::LeftSquare)?;
        let visiblity = self.expect_ident()?;
        self.expect(Token::RightSquare)?;
        self.expect(Token::RightSquare)?;

        Ok(match visiblity.name {
            sym::public => true,
            sym::private => false,
            _ => todo!("illegal visibility modifier"),
        })
    }

    /// Parses particular section in the Input or State file.
    /// `
    /// [<identifier>]
    /// <...definition>
    /// `
    /// Returns [`Section`].
    pub fn parse_section(&mut self) -> Result<Section> {
        self.expect(Token::LeftSquare)?;
        let section = self.expect_ident()?;
        self.expect(Token::RightSquare)?;

        let mut definitions = Vec::new();
        
        while let Some(SpannedToken{ token: Token::Ident(_), .. }) = self.peek_option() {
            definitions.push(self.parse_input_definition()?);
        }

        Ok(Section {
            is_public: true,
            name: section.name,
            span: section.span.clone(),
            definitions,
        })
    }

    /// Parses a single parameter definition:
    /// `<identifier> : <type> = <expression>;`
    /// Returns [`Definition`].
    pub fn parse_input_definition(&mut self) -> Result<Definition> {
        let name = self.expect_ident()?;
        self.expect(Token::Colon)?;
        let (type_, span) = self.parse_type()?;
        self.expect(Token::Assign)?;
        let value = self.parse_primary_expression()?;
        self.expect(Token::Semicolon)?;

        Ok(Definition {
            name,
            type_,
            value,
            span,
        })
    }
}
