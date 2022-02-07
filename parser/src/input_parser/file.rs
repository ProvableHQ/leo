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
use crate::KEYWORD_TOKENS;

use smallvec::smallvec;
use leo_errors::{ParserError, Result};
use leo_span::sym;
use crate::{Token, SpannedToken};
use tendril::format_tendril;

const INT_TYPES: &[Token] = &[
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
];

pub(crate) const TYPE_TOKENS: &[Token] = &[
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

impl InputParserContext<'_> {

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
    /// Returns a [`Program`] AST if all tokens can be consumed and represent a valid Leo program.
    ///
    pub fn parse_input(&mut self) -> Result<Input> {

        while self.has_next() {
            let token = self.peek()?;
            
            match token.token {
                Token::LeftSquare => {
                    let (section, definitions) = self.parse_section()?;
                    println!("Section: {}, Definitions (len): {}", section, definitions.len());
                },
                _ => ()
            };
        }


        Ok(Input::new())
    }

    pub fn parse_section(&mut self) -> Result<(Identifier, IndexMap<Identifier, (Type, Expression)>)> {
        self.expect(Token::LeftSquare)?;
        let section = self.expect_ident()?;
        self.expect(Token::RightSquare)?;
        let mut assignments = IndexMap::new();

        while self.has_next() {
            let token = &self.peek()?.token;
            if let Token::Ident(_) = token {
                let (ident, (type_, value)) = self.parse_assignment()?;
                assignments.insert(ident, (type_, value));
            } else {
                break;
            }
        }
        
        Ok((section, assignments))   
    }

    pub fn parse_assignment(&mut self) -> Result<(Identifier, (Type, Expression))> {
        let var = self.expect_ident()?;
        self.expect(Token::Colon)?;
        let (type_, _span) = self.parse_type()?;
        self.expect(Token::Assign)?;
        let value = self.parse_primary_expression()?;
        self.expect(Token::Semicolon)?;

        Ok((var, (type_, value)))
    }


    /// Returns a [`(Type, Span)`] tuple of AST nodes if the next token represents a type.
    /// Also returns the span of the parsed token.
    pub fn parse_type(&mut self) -> Result<(Type, Span)> {
        Ok(if let Some(token) = self.eat(Token::BigSelf) {
            (Type::SelfType, token.span)
        } else if let Some(ident) = self.eat_identifier() {
            let span = ident.span.clone();
            (Type::Identifier(ident), span)
        } else if self.peek_is_left_par() {
            let (types, _, span) = self.parse_paren_comma_list(|p| p.parse_type().map(|t| Some(t.0)))?;
            (Type::Tuple(types), span)
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

    /// Returns an [`ArrayDimensions`] AST node if the next tokens represent dimensions for an array type.
    pub fn parse_array_dimensions(&mut self) -> Result<ArrayDimensions> {
        Ok(if let Some(dim) = self.parse_array_dimension() {
            ArrayDimensions(smallvec![dim])
        } else {
            let mut had_item_err = false;
            let (dims, _, span) = self.parse_paren_comma_list(|p| {
                Ok(if let Some(dim) = p.parse_array_dimension() {
                    Some(dim)
                } else {
                    let token = p.expect_any()?;
                    p.emit_err(ParserError::unexpected_str(&token.token, "int", &token.span));
                    had_item_err = true;
                    None
                })
            })?;
            if dims.is_empty() && !had_item_err {
                self.emit_err(ParserError::array_tuple_dimensions_empty(&span));
            }
            ArrayDimensions(dims.into())
        })
    }

    /// Parses a basic array dimension, i.e., an integer or `_`.
    fn parse_array_dimension(&mut self) -> Option<Dimension> {
        if let Some((int, _)) = self.eat_int() {
            Some(Dimension::Number(int))
        } else if self.eat(Token::Underscore).is_some() {
            Some(Dimension::Unspecified)
        } else {
            None
        }
    }

    ///
    /// Returns an [`Expression`] AST node if the next token is a primary expression:
    /// - Literals: field, group, unsigned integer, signed integer, boolean, address
    /// - Aggregate types: array, tuple
    /// - Identifiers: variables, keywords
    /// - self
    ///
    /// Returns an expression error if the token cannot be matched.
    ///
    pub fn parse_primary_expression(&mut self) -> Result<Expression> {
        let SpannedToken { token, span } = self.expect_any()?;
        Ok(match token {
            Token::Int(value) => {
                let type_ = self.eat_any(INT_TYPES);
                match type_ {
                    Some(SpannedToken {
                        token: Token::Field,
                        span: type_span,
                    }) => {
                        assert_no_whitespace(&span, &type_span, &value, "field")?;
                        Expression::Value(ValueExpression::Field(value, span + type_span))
                    }
                    Some(SpannedToken {
                        token: Token::Group,
                        span: type_span,
                    }) => {
                        assert_no_whitespace(&span, &type_span, &value, "group")?;
                        Expression::Value(ValueExpression::Group(Box::new(GroupValue::Single(
                            value,
                            span + type_span,
                        ))))
                    }
                    Some(SpannedToken { token, span: type_span }) => {
                        assert_no_whitespace(&span, &type_span, &value, &token.to_string())?;
                        Expression::Value(ValueExpression::Integer(
                            Self::token_to_int_type(token).expect("unknown int type token"),
                            value,
                            span + type_span,
                        ))
                    }
                    None => Expression::Value(ValueExpression::Implicit(value, span)),
                }
            }
            Token::True => Expression::Value(ValueExpression::Boolean("true".into(), span)),
            Token::False => Expression::Value(ValueExpression::Boolean("false".into(), span)),
            Token::AddressLit(value) => Expression::Value(ValueExpression::Address(value, span)),
            Token::CharLit(value) => Expression::Value(ValueExpression::Char(CharValue {
                character: value.into(),
                span,
            })),
            Token::StringLit(value) => Expression::Value(ValueExpression::String(value, span)),
            Token::LeftParen => self.parse_tuple_expression(&span)?,
            Token::LeftSquare => self.parse_array_expression(&span)?,
            Token::Ident(name) => Expression::Identifier(Identifier { name, span }),
            Token::Input => Expression::Identifier(Identifier { name: sym::input, span }),
            t if crate::type_::TYPE_TOKENS.contains(&t) => Expression::Identifier(Identifier {
                name: t.keyword_to_symbol().unwrap(),
                span,
            }),
            token => {
                return Err(ParserError::unexpected_str(token, "expression", &span).into());
            }
        })
    }

    pub fn parse_tuple_expression(&mut self, span: &Span) -> Result<Expression> {
        if let Some((left, right, span)) = self.eat_group_partial().transpose()? {
            return Ok(Expression::Value(ValueExpression::Group(Box::new(GroupValue::Tuple(
                GroupTuple {
                    span,
                    x: left,
                    y: right,
                },
            )))));
        }
        let mut args = Vec::new();
        let end_span;
        loop {
            let end = self.eat(Token::RightParen);
            if let Some(end) = end {
                end_span = end.span;
                break;
            }
            let expr = self.parse_primary_expression()?;
            args.push(expr);
            if self.eat(Token::Comma).is_none() {
                end_span = self.expect(Token::RightParen)?;
                break;
            }
        }
        if args.len() == 1 {
            Ok(args.remove(0))
        } else {
            Ok(Expression::TupleInit(TupleInitExpression {
                span: span + &end_span,
                elements: args,
            }))
        }
    }

    ///
    /// Returns an [`Expression`] AST node if the next tokens represent an
    /// array initialization expression.
    ///
    pub fn parse_array_expression(&mut self, span: &Span) -> Result<Expression> {
        if let Some(end) = self.eat(Token::RightSquare) {
            return Ok(Expression::ArrayInline(ArrayInlineExpression {
                elements: Vec::new(),
                span: span + &end.span,
            }));
        }
        let first = self.parse_primary_expression()?;
        if self.eat(Token::Semicolon).is_some() {
            let dimensions = self
                .parse_array_dimensions()
                .map_err(|_| ParserError::unable_to_parse_array_dimensions(span))?;
            let end = self.expect(Token::RightSquare)?;
            Ok(Expression::ArrayInit(ArrayInitExpression {
                span: span + &end,
                element: Box::new(first),
                dimensions,
            }))
        } else {
            let end_span;
            let mut elements = vec![first];
            loop {
                if let Some(token) = self.eat(Token::RightSquare) {
                    end_span = token.span;
                    break;
                }
                if elements.len() == 1 {
                    self.expect(Token::Comma)?;
                    if let Some(token) = self.eat(Token::RightSquare) {
                        end_span = token.span;
                        break;
                    }
                }
                elements.push(self.parse_primary_expression()?);
                if self.eat(Token::Comma).is_none() {
                    end_span = self.expect(Token::RightSquare)?;
                    break;
                }
            }
            Ok(Expression::ArrayInline(ArrayInlineExpression {
                elements: elements.into_iter().map(|expr| SpreadOrExpression::Expression(expr)).collect(),
                span: span + &end_span,
            }))
        }
    }
}
