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

pub(super) const TYPE_TOKENS: &[Token] = &[
    Token::Address,
    Token::Bool,
    Token::Field,
    Token::Future,
    Token::Group,
    Token::Scalar,
    Token::Signature,
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

impl<N: Network> ParserContext<'_, N> {
    /// Returns a [`IntegerType`] AST node if the given token is a supported integer type, or [`None`].
    pub(super) fn token_to_int_type(token: &Token) -> Option<IntegerType> {
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

    /// Returns a [`(Type, Span)`] tuple of AST nodes if the next token represents a primitive type.
    /// Also returns the span of the parsed token.
    pub fn parse_primitive_type(&mut self) -> Result<(Type, Span)> {
        let span = self.expect_any(TYPE_TOKENS)?;
        Ok((
            match &self.prev_token.token {
                Token::Address => Type::Address,
                Token::Bool => Type::Boolean,
                Token::Field => Type::Field,
                Token::Group => Type::Group,
                Token::Scalar => Type::Scalar,
                Token::Signature => Type::Signature,
                Token::String => Type::String,
                x => Type::Integer(Self::token_to_int_type(x).expect("invalid int type")),
            },
            span,
        ))
    }

    /// Returns a [`(Type, Span)`] tuple of AST nodes if the next token represents a type.
    /// Also returns the span of the parsed token.
    pub fn parse_type(&mut self) -> Result<(Type, Span)> {
        if let Some(ident) = self.eat_identifier() {
            // Check if using external type
            let file_type = self.look_ahead(1, |t| &t.token);
            if self.token.token == Token::Dot && (file_type == &Token::Aleo) {
                // Only allow `.aleo` as the network identifier
                if file_type == &Token::Leo {
                    return Err(ParserError::invalid_network(self.token.span).into());
                }

                // Parse `.aleo/`
                self.expect(&Token::Dot)?;
                self.expect(&Token::Aleo)?;
                self.expect(&Token::Div)?;

                // Parse the record name
                if let Some(record_name) = self.eat_identifier() {
                    // Return the external type
                    return Ok((
                        Type::Composite(CompositeType { id: record_name, program: Some(ident.name) }),
                        ident.span + record_name.span,
                    ));
                } else {
                    return Err(ParserError::invalid_external_type(self.token.span).into());
                }
            }

            Ok((Type::Composite(CompositeType { id: ident, program: self.program_name }), ident.span))
        } else if self.token.token == Token::LeftSquare {
            // Parse the left bracket.
            self.expect(&Token::LeftSquare)?;
            // Parse the element type.
            let (element_type, _) = self.parse_type()?;
            // Parse the semi-colon.
            self.expect(&Token::Semicolon)?;
            // Parse the length.
            let (length, _) = self.eat_whole_number()?;
            // Parse the right bracket.
            self.expect(&Token::RightSquare)?;
            // Return the array type.
            Ok((Type::Array(ArrayType::new(element_type, length)), self.prev_token.span))
        } else if self.token.token == Token::LeftParen {
            let (types, _, span) = self.parse_paren_comma_list(|p| p.parse_type().map(Some))?;
            match types.len() {
                // If the parenthetical block is empty, e.g. `()` or `( )`, it should be parsed into `Unit` types.
                0 => Ok((Type::Unit, span)),
                // If the parenthetical block contains a single type, e.g. `(u8)`, emit an error, since tuples must have at least two elements.
                1 => Err(ParserError::tuple_must_have_at_least_two_elements("type", span).into()),
                // Otherwise, parse it into a `Tuple` type.
                // Note: This is the only place where `Tuple` type is constructed in the parser.
                _ => Ok((Type::Tuple(TupleType::new(types.into_iter().map(|t| t.0).collect())), span)),
            }
        } else if self.token.token == Token::Future {
            // Parse the `Future` token.
            let span = self.expect(&Token::Future)?;
            // Parse the explicit future type, e.g. `Future<Fn(u32, u32)>`, `Future<Fn(u32, Future<Fn(u32, u32, u64)>)>` etc.
            if self.token.token == Token::Lt {
                // Expect the sequence `<`, `Fn`.
                self.expect(&Token::Lt)?;
                self.expect(&Token::Fn)?;
                // Parse the parenthesis list of function arguments.
                let (types, _, full_span) = self.parse_paren_comma_list(|p| p.parse_type().map(Some))?;
                // Expect the closing `>`.
                self.expect(&Token::Gt)?;
                Ok((
                    Type::Future(FutureType::new(types.into_iter().map(|t| t.0).collect(), None, true)),
                    span + full_span,
                ))
            } else {
                Ok((Type::Future(Default::default()), span))
            }
        } else {
            self.parse_primitive_type()
        }
    }
}
