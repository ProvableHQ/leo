// Copyright (C) 2019-2021 Aleo Systems Inc.
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

const TYPE_TOKENS: &[Token] = &[
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
    Token::Field,
    Token::Group,
    Token::Address,
    Token::Bool,
    Token::Char,
];

impl ParserContext {
    ///
    /// Returns a [`IntegerType`] AST node if the given token is a supported integer type, or [`None`].
    ///
    pub fn token_to_int_type(token: Token) -> Option<IntegerType> {
        Some(match token {
            Token::I8 => IntegerType::I8,
            Token::I16 => IntegerType::I16,
            Token::I32 => IntegerType::I32,
            Token::I64 => IntegerType::I64,
            Token::I128 => IntegerType::I128,
            Token::U8 => IntegerType::U8,
            Token::U16 => IntegerType::U16,
            Token::U32 => IntegerType::U32,
            Token::U64 => IntegerType::U64,
            Token::U128 => IntegerType::U128,
            _ => return None,
        })
    }

    ///
    /// Returns an [`ArrayDimensions`] AST node if the next tokens represent dimensions for an array type.
    ///
    pub fn parse_array_dimensions(&mut self) -> SyntaxResult<ArrayDimensions> {
        Ok(if let Some((int, _)) = self.eat_int() {
            ArrayDimensions(vec![int])
        } else {
            self.expect(Token::LeftParen)?;
            let mut dimensions = Vec::new();
            loop {
                if let Some((int, _)) = self.eat_int() {
                    dimensions.push(int);
                } else {
                    let token = self.peek()?;
                    return Err(SyntaxError::unexpected_str(&token.token, "int", &token.span));
                }
                if self.eat(Token::Comma).is_none() {
                    break;
                }
            }
            self.expect(Token::RightParen)?;
            ArrayDimensions(dimensions)
        })
    }

    ///
    /// Returns a [`(Type, Span)`] tuple of AST nodes if the next token represents a type. Also
    /// returns the span of the parsed token.
    ///
    pub fn parse_type(&mut self) -> SyntaxResult<(Type, Span)> {
        Ok(if let Some(token) = self.eat(Token::BigSelf) {
            (Type::SelfType, token.span)
        } else if let Some(ident) = self.eat_identifier() {
            let span = ident.span.clone();
            (Type::Circuit(ident), span)
        } else if let Some(token) = self.eat(Token::LeftParen) {
            let mut types = Vec::new();
            let end_span;
            loop {
                if let Some(end) = self.eat(Token::RightParen) {
                    end_span = end.span;
                    break;
                }
                types.push(self.parse_type()?.0);
                if self.eat(Token::Comma).is_none() {
                    end_span = self.expect(Token::RightParen)?;
                    break;
                }
            }
            (Type::Tuple(types), token.span + end_span)
        } else if let Some(token) = self.eat(Token::LeftSquare) {
            let (inner, _) = self.parse_type()?;
            self.expect(Token::Semicolon)?;
            let dimensions = self.parse_array_dimensions()?;
            let end_span = self.expect(Token::RightSquare)?;
            (Type::Array(Box::new(inner), dimensions), token.span + end_span)
        } else {
            let token = self.expect_oneof(TYPE_TOKENS)?;
            (
                match token.token {
                    Token::Field => Type::Field,
                    Token::Group => Type::Group,
                    Token::Address => Type::Address,
                    Token::Bool => Type::Boolean,
                    Token::Char => Type::Char,
                    x => Type::IntegerType(Self::token_to_int_type(x).expect("invalid int type")),
                },
                token.span,
            )
        })
    }
}
