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

pub(super) const TYPE_TOKENS: &[Token] = &[
    Token::Address,
    Token::Bool,
    Token::Field,
    Token::Group,
    Token::Scalar,
    Token::String,
    Token::I8,
    Token::I16,
    Token::I32,
    Token::I64,
    Token::I128,
    Token::U8,
    Token::U16,
    Token::U32,
    Token::U64,
    Token::U128,
];

impl ParserContext<'_> {
    /// Returns a [`(Type, Span)`] tuple of AST nodes if the next token represents a primitive type.
    /// Also returns the span of the parsed token.
    pub fn parse_primitive_type(&mut self) -> Result<(Type, Span)> {
        let span = self.expect_any(TYPE_TOKENS)?;
        match &self.prev_token.token {
            Token::Address => Ok((Type::Address, span)),
            Token::Bool => Ok((Type::Boolean, span)),
            Token::Field => Ok((Type::Field, span)),
            Token::Group => Ok((Type::Group, span)),
            Token::I8 => Ok((Type::I8, span)),
            Token::I16 => Ok((Type::I16, span)),
            Token::I32 => Ok((Type::I32, span)),
            Token::I64 => Ok((Type::I64, span)),
            Token::I128 => Ok((Type::I128, span)),
            Token::Scalar => Ok((Type::Scalar, span)),
            Token::String => Ok((Type::String, span)),
            Token::U8 => Ok((Type::U8, span)),
            Token::U16 => Ok((Type::U16, span)),
            Token::U32 => Ok((Type::U32, span)),
            Token::U64 => Ok((Type::U64, span)),
            Token::U128 => Ok((Type::U128, span)),
            _ => Err(ParserError::unexpected_token("Expected a primitive type.", span).into()),
        }
    }

    /// Returns a [`(Type, Span)`] tuple of AST nodes if the next token represents a type.
    /// Also returns the span of the parsed token.
    pub fn parse_all_types(&mut self) -> Result<(Type, Span)> {
        Ok(if let Some(ident) = self.eat_identifier() {
            let span = ident.span;
            (Type::Identifier(ident), span)
        } else {
            self.parse_primitive_type()?
        })
    }
}
