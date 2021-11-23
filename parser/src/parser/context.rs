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

use crate::{assert_no_whitespace, tokenizer::*, Token, KEYWORD_TOKENS};
use leo_ast::*;
use leo_errors::emitter::Handler;
use leo_errors::{LeoError, ParserError, Result, Span};
use tendril::format_tendril;

/// Stores a program in tokenized format plus additional context.
/// May be converted into a [`Program`] AST by parsing all tokens.
pub struct ParserContext<'a> {
    #[allow(dead_code)]
    pub(crate) handler: &'a Handler,
    pub(crate) internal: bool,
    tokens: Vec<SpannedToken>,
    end_span: Span,
    // true if parsing an expression for an if statement -- means circuit inits are not legal
    pub(crate) fuzzy_struct_state: bool,
}

impl Iterator for ParserContext<'_> {
    type Item = SpannedToken;

    fn next(&mut self) -> Option<SpannedToken> {
        self.bump()
    }
}

impl<'a> ParserContext<'a> {
    ///
    /// Returns a new [`ParserContext`] type given a vector of tokens.
    ///
    pub fn new(handler: &'a Handler, internal: bool, mut tokens: Vec<SpannedToken>) -> Self {
        tokens.reverse();
        // todo: performance optimization here: drain filter
        tokens = tokens
            .into_iter()
            .filter(|x| !matches!(x.token, Token::CommentLine(_) | Token::CommentBlock(_)))
            .collect();
        Self {
            handler,
            internal,
            end_span: tokens
                .iter()
                .find(|x| !x.span.content.trim().is_empty())
                .map(|x| x.span.clone())
                .unwrap_or_default(),
            tokens,
            fuzzy_struct_state: false,
        }
    }

    /// Returns the current token if there is one.
    pub fn curr(&self) -> Option<&SpannedToken> {
        self.tokens.last()
    }

    /// Emit the error `err`.
    pub(crate) fn emit_err(&self, err: ParserError) {
        self.handler.emit_err(err.into());
    }

    ///
    /// Returns an unexpected end of function [`SyntaxError`].
    ///
    pub fn eof(&self) -> LeoError {
        ParserError::unexpected_eof(&self.end_span).into()
    }

    ///
    /// Returns a reference to the next next token or error if it does not exist.
    ///
    pub fn peek_next(&self) -> Result<&SpannedToken> {
        self.tokens.get(self.tokens.len() - 2).ok_or_else(|| self.eof())
    }

    ///
    /// Returns a reference to the next SpannedToken or error if it does not exist.
    ///
    pub fn peek(&self) -> Result<&SpannedToken> {
        self.curr().ok_or_else(|| self.eof())
    }

    ///
    /// Returns a reference to the next Token.
    ///
    pub fn peek_token(&self) -> Cow<'_, Token> {
        self.tokens
            .last()
            .map(|x| &x.token)
            .map(Cow::Borrowed)
            .unwrap_or_else(|| Cow::Owned(Token::Eof))
    }

    // pub fn peek_oneof(&self, token: &[Token]) -> Result<&SpannedToken> {
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

    /// Advances the current token.
    pub fn bump(&mut self) -> Option<SpannedToken> {
        self.tokens.pop()
    }

    ///
    /// Removes the next token if it exists and returns it, or [None] if
    /// the next token does not exist.
    ///
    pub fn eat(&mut self, token: Token) -> Option<SpannedToken> {
        if let Some(SpannedToken { token: inner, .. }) = self.curr() {
            if &token == inner {
                return self.bump();
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
        }) = self.curr()
        {
            if let SpannedToken {
                token: Token::Ident(name),
                span,
            } = self.bump().unwrap()
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

    /// Returns `true` if the next token is Function or if it is a Const followed by Function.
    /// Returns `false` otherwise.
    pub fn peek_is_function(&self) -> Result<bool> {
        let first = &self.peek()?.token;
        let next = if self.tokens.len() >= 2 {
            &self.peek_next()?.token
        } else {
            return Ok(false);
        };
        Ok(matches!(
            (first, next),
            (Token::Function | Token::At, _) | (Token::Const, Token::Function)
        ))
    }

    ///
    /// Removes the next two tokens if they are a pair of [`GroupCoordinate`] and returns them,
    /// or [None] if the next token is not a [`GroupCoordinate`].
    ///
    pub fn eat_group_partial(&mut self) -> Option<Result<(GroupCoordinate, GroupCoordinate, Span)>> {
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
        }) = self.curr()
        {
            if let SpannedToken {
                token: Token::Int(value),
                span,
            } = self.bump().unwrap()
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
        if let Some(SpannedToken { token: inner, .. }) = self.curr() {
            if token.iter().any(|x| x == inner) {
                return self.bump();
            }
        }
        None
    }

    ///
    /// Returns the span of the next token if it is equal to the given [`Token`], or error.
    ///
    pub fn expect(&mut self, token: Token) -> Result<Span> {
        if let Some(SpannedToken { token: inner, span }) = self.curr() {
            if &token == inner {
                Ok(self.bump().unwrap().span)
            } else {
                Err(ParserError::unexpected(inner, token, span).into())
            }
        } else {
            Err(self.eof())
        }
    }

    ///
    /// Returns the span of the next token if it is equal to one of the given [`Token`]s, or error.
    ///
    pub fn expect_oneof(&mut self, token: &[Token]) -> Result<SpannedToken> {
        if let Some(SpannedToken { token: inner, span }) = self.curr() {
            if token.iter().any(|x| x == inner) {
                Ok(self.bump().unwrap())
            } else {
                return Err(ParserError::unexpected(
                    inner,
                    token.iter().map(|x| format!("'{}'", x)).collect::<Vec<_>>().join(", "),
                    span,
                )
                .into());
            }
        } else {
            Err(self.eof())
        }
    }

    ///
    /// Returns the [`Identifier`] of the next token if it is a keyword,
    /// [`Token::Int(_)`], or an [`Identifier`], or error.
    ///
    pub fn expect_loose_identifier(&mut self) -> Result<Identifier> {
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
    pub fn expect_ident(&mut self) -> Result<Identifier> {
        if let Some(SpannedToken { token: inner, span }) = self.curr() {
            if let Token::Ident(_) = inner {
                if let SpannedToken {
                    token: Token::Ident(name),
                    span,
                } = self.bump().unwrap()
                {
                    Ok(Identifier { name, span })
                } else {
                    unimplemented!()
                }
            } else {
                Err(ParserError::unexpected_str(inner, "ident", span).into())
            }
        } else {
            Err(self.eof())
        }
    }

    ///
    /// Returns the next token if it exists or return end of function.
    ///
    pub fn expect_any(&mut self) -> Result<SpannedToken> {
        if let Some(x) = self.tokens.pop() {
            Ok(x)
        } else {
            Err(self.eof())
        }
    }

    /// Parses a list of `T`s using `inner`
    /// The opening and closing delimiters are `bra` and `ket`,
    /// and elements in the list are separated by `sep`.
    /// When `(list, true)` is returned, `sep` was a terminator.
    pub(super) fn parse_list<T>(
        &mut self,
        open: Token,
        close: Token,
        sep: Token,
        mut inner: impl FnMut(&mut Self) -> Result<Option<T>>,
    ) -> Result<(Vec<T>, bool, Span)> {
        let mut list = Vec::new();
        let mut trailing = false;

        // Parse opening delimiter.
        let open_span = self.expect(open)?;

        while self.peek()?.token != close {
            // Parse the element. We allow inner parser recovery through the `Option`.
            if let Some(elem) = inner(self)? {
                list.push(elem);
            }

            // Parse the separator.
            if self.eat(sep.clone()).is_none() {
                trailing = false;
                break;
            }
        }

        // Parse closing delimiter.
        let close_span = self.expect(close)?;

        Ok((list, trailing, open_span + close_span))
    }

    /// Parse a list separated by `,` and delimited by parens.
    pub(super) fn parse_paren_comma_list<T>(
        &mut self,
        f: impl FnMut(&mut Self) -> Result<Option<T>>,
    ) -> Result<(Vec<T>, bool, Span)> {
        self.parse_list(Token::LeftParen, Token::RightParen, Token::Comma, f)
    }

    /// Returns true if the current token is `(`.
    pub(super) fn peek_is_left_par(&self) -> bool {
        matches!(self.curr().map(|t| &t.token), Some(Token::LeftParen))
    }
}
