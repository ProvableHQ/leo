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

use std::{borrow::Cow, unimplemented};

use crate::{assert_no_whitespace, tokenizer::*, SyntaxError, SyntaxResult, Token, KEYWORD_TOKENS};
use leo_ast::*;
use tendril::format_tendril;

/// Stores a program in tokenized format plus additional context.
/// May be converted into a [`Program`] AST by parsing all tokens.
pub struct ParserContext {
    tokens: Vec<SpannedToken>,
    end_span: Span,
    // true if parsing an expression for an if statement -- means circuit inits are not legal
    pub(crate) fuzzy_struct_state: bool,
}

impl Iterator for ParserContext {
    type Item = SpannedToken;

    fn next(&mut self) -> Option<SpannedToken> {
        self.tokens.pop()
    }
}

impl ParserContext {
    ///
    /// Returns a new [`ParserContext`] type given a vector of tokens.
    ///
    pub fn new(mut tokens: Vec<SpannedToken>) -> Self {
        tokens.reverse();
        // todo: performance optimization here: drain filter
        tokens = tokens
            .into_iter()
            .filter(|x| !matches!(x.token, Token::CommentLine(_) | Token::CommentBlock(_)))
            .collect();
        ParserContext {
            end_span: tokens
                .iter()
                .find(|x| !x.span.content.trim().is_empty())
                .map(|x| x.span.clone())
                .unwrap_or_default(),
            tokens,
            fuzzy_struct_state: false,
        }
    }

    ///
    /// Returns an unexpected end of function [`SyntaxError`].
    ///
    pub fn eof(&self) -> SyntaxError {
        SyntaxError::unexpected_eof(&self.end_span)
    }

    ///
    /// Returns a reference to the next next token or error if it does not exist.
    ///
    pub fn peek_next(&self) -> SyntaxResult<&SpannedToken> {
        self.tokens.get(self.tokens.len() - 2).ok_or_else(|| self.eof())
    }

    ///
    /// Returns a reference to the next token or error if it does not exist.
    ///
    pub fn peek(&self) -> SyntaxResult<&SpannedToken> {
        self.tokens.last().ok_or_else(|| self.eof())
    }

    pub fn peek_token(&self) -> Cow<'_, Token> {
        self.tokens
            .last()
            .map(|x| &x.token)
            .map(Cow::Borrowed)
            .unwrap_or_else(|| Cow::Owned(Token::Eof))
    }

    // pub fn peek_oneof(&self, token: &[Token]) -> SyntaxResult<&SpannedToken> {
    //     if let Some(spanned_token) = self.inner.last() {
    //         if token.iter().any(|x| x == &spanned_token.token) {
    //             Ok(spanned_token)
    //         } else {
    //             Err(SyntaxError::unexpected(
    //                 &spanned_token.token,
    //                 token,
    //                 &spanned_token.span,
    //             ))
    //         }
    //     } else {
    //         Err(self.eof())
    //     }
    // }

    ///
    /// Returns true if the next token exists.
    ///
    pub fn has_next(&self) -> bool {
        !self.tokens.is_empty()
    }

    ///
    /// Removes the next token if it exists and returns it, or [None] if
    /// the next token does not exist.
    ///
    pub fn eat(&mut self, token: Token) -> Option<SpannedToken> {
        if let Some(SpannedToken { token: inner, .. }) = self.tokens.last() {
            if &token == inner {
                return self.tokens.pop();
            }
        }
        None
    }

    ///
    /// Appends a token to the back of the vector.
    ///
    pub fn backtrack(&mut self, token: SpannedToken) {
        self.tokens.push(token);
    }

    ///
    /// Removes the next token if it is a [`Token::Ident(_)`] and returns it, or [None] if
    /// the next token is not a [`Token::Ident(_)`] or if the next token does not exist.
    ///
    pub fn eat_identifier(&mut self) -> Option<Identifier> {
        if let Some(SpannedToken {
            token: Token::Ident(_), ..
        }) = self.tokens.last()
        {
            let token = self.tokens.pop().unwrap();
            if let SpannedToken {
                token: Token::Ident(name),
                span,
            } = token
            {
                return Some(Identifier { name, span });
            } else {
                unimplemented!()
            }
        }
        None
    }

    ///
    /// Returns a reference to the next token if it is a [`GroupCoordinate`], or [None] if
    /// the next token is not a [`GroupCoordinate`].
    ///
    fn peek_group_coordinate(&self, i: &mut usize) -> Option<GroupCoordinate> {
        if *i < 1 {
            return None;
        }
        let token = self.tokens.get(*i - 1)?;
        *i -= 1;
        Some(match &token.token {
            Token::Add => GroupCoordinate::SignHigh,
            Token::Minus if *i > 0 => match self.tokens.get(*i - 1) {
                Some(SpannedToken {
                    token: Token::Int(value),
                    span,
                }) => {
                    if *i < 1 {
                        return None;
                    }
                    *i -= 1;
                    GroupCoordinate::Number(format_tendril!("-{}", value), span.clone())
                }
                _ => GroupCoordinate::SignLow,
            },
            Token::Underscore => GroupCoordinate::Inferred,
            Token::Int(value) => GroupCoordinate::Number(value.clone(), token.span.clone()),
            _ => return None,
        })
    }

    ///
    /// Removes the next two tokens if they are a pair of [`GroupCoordinate`] and returns them,
    /// or [None] if the next token is not a [`GroupCoordinate`].
    ///
    pub fn eat_group_partial(&mut self) -> Option<SyntaxResult<(GroupCoordinate, GroupCoordinate, Span)>> {
        let mut i = self.tokens.len();
        if i < 1 {
            return None;
        }
        let start_span = self.tokens.get(i - 1)?.span.clone();
        let first = self.peek_group_coordinate(&mut i)?;
        if i < 1 {
            return None;
        }
        match self.tokens.get(i - 1) {
            Some(SpannedToken {
                token: Token::Comma, ..
            }) => {
                i -= 1;
            }
            _ => {
                return None;
            }
        }
        let second = self.peek_group_coordinate(&mut i)?;
        if i < 1 {
            return None;
        }
        let right_paren_span;
        match self.tokens.get(i - 1) {
            Some(SpannedToken {
                token: Token::RightParen,
                span,
            }) => {
                right_paren_span = span.clone();
                i -= 1;
            }
            _ => {
                return None;
            }
        }
        if i < 1 {
            return None;
        }
        let end_span;
        match self.tokens.get(i - 1) {
            Some(SpannedToken {
                token: Token::Group,
                span,
            }) => {
                end_span = span.clone();
                i -= 1;
            }
            _ => {
                return None;
            }
        }

        self.tokens.drain(i..);
        if let Err(e) = assert_no_whitespace(
            &right_paren_span,
            &end_span,
            &format!("({},{})", first, second),
            "group",
        ) {
            return Some(Err(e));
        }
        Some(Ok((first, second, start_span + end_span)))
    }

    ///
    /// Removes the next token if it is a [`Token::Int(_)`] and returns it, or [None] if
    /// the next token is not a [`Token::Int(_)`] or if the next token does not exist.
    ///
    pub fn eat_int(&mut self) -> Option<(PositiveNumber, Span)> {
        if let Some(SpannedToken {
            token: Token::Int(_), ..
        }) = self.tokens.last()
        {
            let token = self.tokens.pop().unwrap();
            if let SpannedToken {
                token: Token::Int(value),
                span,
            } = token
            {
                return Some((PositiveNumber { value }, span));
            } else {
                unimplemented!()
            }
        }
        None
    }

    ///
    /// Removes the next token if it exists and returns it, or [None] if
    /// the next token  does not exist.
    ///
    pub fn eat_any(&mut self, token: &[Token]) -> Option<SpannedToken> {
        if let Some(SpannedToken { token: inner, .. }) = self.tokens.last() {
            if token.iter().any(|x| x == inner) {
                return self.tokens.pop();
            }
        }
        None
    }

    ///
    /// Returns the span of the next token if it is equal to the given [`Token`], or error.
    ///
    pub fn expect(&mut self, token: Token) -> SyntaxResult<Span> {
        if let Some(SpannedToken { token: inner, span }) = self.tokens.last() {
            if &token == inner {
                Ok(self.tokens.pop().unwrap().span)
            } else {
                Err(SyntaxError::unexpected(inner, &[token], span))
            }
        } else {
            Err(self.eof())
        }
    }

    ///
    /// Returns the span of the next token if it is equal to one of the given [`Token`]s, or error.
    ///
    pub fn expect_oneof(&mut self, token: &[Token]) -> SyntaxResult<SpannedToken> {
        if let Some(SpannedToken { token: inner, span }) = self.tokens.last() {
            if token.iter().any(|x| x == inner) {
                Ok(self.tokens.pop().unwrap())
            } else {
                Err(SyntaxError::unexpected(inner, token, span))
            }
        } else {
            Err(self.eof())
        }
    }

    ///
    /// Returns the [`Identifier`] of the next token if it is a keyword,
    /// [`Token::Int(_)`], or an [`Identifier`], or error.
    ///
    pub fn expect_loose_identifier(&mut self) -> SyntaxResult<Identifier> {
        if let Some(token) = self.eat_any(KEYWORD_TOKENS) {
            return Ok(Identifier {
                name: token.token.to_string().into(),
                span: token.span,
            });
        }
        if let Some((int, span)) = self.eat_int() {
            return Ok(Identifier { name: int.value, span });
        }
        self.expect_ident()
    }

    ///
    /// Returns the [`Identifier`] of the next token if it is an [`Identifier`], or error.
    ///
    pub fn expect_ident(&mut self) -> SyntaxResult<Identifier> {
        if let Some(SpannedToken { token: inner, span }) = self.tokens.last() {
            if let Token::Ident(_) = inner {
                let token = self.tokens.pop().unwrap();
                if let SpannedToken {
                    token: Token::Ident(name),
                    span,
                } = token
                {
                    Ok(Identifier { name, span })
                } else {
                    unimplemented!()
                }
            } else {
                Err(SyntaxError::unexpected_str(inner, "ident", span))
            }
        } else {
            Err(self.eof())
        }
    }

    ///
    /// Returns the next token if it exists or return end of function.
    ///
    pub fn expect_any(&mut self) -> SyntaxResult<SpannedToken> {
        if let Some(x) = self.tokens.pop() {
            Ok(x)
        } else {
            Err(self.eof())
        }
    }
}
