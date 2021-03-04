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

use std::unimplemented;

use crate::{tokenizer::*, SyntaxError, SyntaxResult, Token, KEYWORD_TOKENS};
use leo_ast::*;

pub struct ParserContext {
    inner: Vec<SpannedToken>,
    end_span: Span,
    // true if parsing an expression for an if statement -- means circuit inits are not legal
    pub(crate) fuzzy_struct_state: bool,
}

impl Iterator for ParserContext {
    type Item = SpannedToken;

    fn next(&mut self) -> Option<SpannedToken> {
        self.inner.pop()
    }
}

impl ParserContext {
    pub fn new(mut tokens: Vec<SpannedToken>) -> Self {
        tokens.reverse();
        // todo: performance optimization here: drain filter
        tokens = tokens
            .into_iter()
            .filter(|x| !matches!(x.token, Token::CommentLine(_) | Token::CommentBlock(_)))
            .collect();
        ParserContext {
            end_span: tokens.last().map(|x| x.span.clone()).unwrap_or_default(),
            inner: tokens,
            fuzzy_struct_state: false,
        }
    }

    pub fn eof(&self) -> SyntaxError {
        SyntaxError::unexpected_eof(&self.end_span)
    }

    pub fn peek(&self) -> SyntaxResult<&SpannedToken> {
        self.inner.last().ok_or_else(|| self.eof())
    }

    pub fn peek_oneof(&self, token: &[Token]) -> SyntaxResult<&SpannedToken> {
        if let Some(spanned_token) = self.inner.last() {
            if token.iter().any(|x| x == &spanned_token.token) {
                Ok(spanned_token)
            } else {
                Err(SyntaxError::unexpected(
                    &spanned_token.token,
                    token,
                    &spanned_token.span,
                ))
            }
        } else {
            Err(self.eof())
        }
    }

    pub fn has_next(&self) -> bool {
        !self.inner.is_empty()
    }

    pub fn eat(&mut self, token: Token) -> Option<SpannedToken> {
        if let Some(SpannedToken { token: inner, .. }) = self.inner.last() {
            if &token == inner {
                return self.inner.pop();
            }
        }
        None
    }

    pub fn backtrack(&mut self, token: SpannedToken) {
        self.inner.push(token);
    }

    pub fn eat_ident(&mut self) -> Option<Identifier> {
        if let Some(SpannedToken {
            token: Token::Ident(_), ..
        }) = self.inner.last()
        {
            let token = self.inner.pop().unwrap();
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

    fn peek_group_coordinate(&self, i: &mut usize) -> Option<GroupCoordinate> {
        let token = self.inner.get(*i)?;
        *i -= 1;
        Some(match &token.token {
            Token::Add => GroupCoordinate::SignHigh,
            Token::Minus => match self.inner.get(*i) {
                Some(SpannedToken {
                    token: Token::Int(value),
                    span,
                }) => {
                    *i -= 1;
                    GroupCoordinate::Number(format!("-{}", value), span.clone())
                }
                _ => GroupCoordinate::SignLow,
            },
            Token::Ident(x) if x == "_" => GroupCoordinate::Inferred,
            Token::Int(value) => GroupCoordinate::Number(value.clone(), token.span.clone()),
            _ => return None,
        })
    }

    // kinda hacky, we're not LALR(1) for groups...
    pub fn eat_group_partial(&mut self) -> Option<(GroupCoordinate, GroupCoordinate, Span)> {
        let mut i = self.inner.len() - 1;
        let start_span = self.inner.get(i)?.span.clone();
        let first = self.peek_group_coordinate(&mut i)?;
        match self.inner.get(i) {
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
        match self.inner.get(i) {
            Some(SpannedToken {
                token: Token::RightParen,
                ..
            }) => {
                i -= 1;
            }
            _ => {
                return None;
            }
        }
        let end_span;
        match self.inner.get(i) {
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

        self.inner.drain((i + 1)..);
        Some((first, second, start_span + end_span))
    }

    pub fn eat_int(&mut self) -> Option<(PositiveNumber, Span)> {
        if let Some(SpannedToken {
            token: Token::Int(_), ..
        }) = self.inner.last()
        {
            let token = self.inner.pop().unwrap();
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

    pub fn eat_any(&mut self, token: &[Token]) -> Option<SpannedToken> {
        if let Some(SpannedToken { token: inner, .. }) = self.inner.last() {
            if token.iter().any(|x| x == inner) {
                return self.inner.pop();
            }
        }
        None
    }

    pub fn expect(&mut self, token: Token) -> SyntaxResult<Span> {
        if let Some(SpannedToken { token: inner, span }) = self.inner.last() {
            if &token == inner {
                Ok(self.inner.pop().unwrap().span)
            } else {
                Err(SyntaxError::unexpected(inner, &[token], span))
            }
        } else {
            Err(self.eof())
        }
    }

    pub fn expect_oneof(&mut self, token: &[Token]) -> SyntaxResult<SpannedToken> {
        if let Some(SpannedToken { token: inner, span }) = self.inner.last() {
            if token.iter().any(|x| x == inner) {
                Ok(self.inner.pop().unwrap())
            } else {
                Err(SyntaxError::unexpected(inner, token, span))
            }
        } else {
            Err(self.eof())
        }
    }

    pub fn expect_loose_ident(&mut self) -> SyntaxResult<Identifier> {
        if let Some(token) = self.eat_any(KEYWORD_TOKENS) {
            return Ok(Identifier {
                name: token.token.to_string(),
                span: token.span,
            });
        }
        if let Some((int, span)) = self.eat_int() {
            return Ok(Identifier { name: int.value, span });
        }
        self.expect_ident()
    }

    pub fn expect_ident(&mut self) -> SyntaxResult<Identifier> {
        if let Some(SpannedToken { token: inner, span }) = self.inner.last() {
            if let Token::Ident(_) = inner {
                let token = self.inner.pop().unwrap();
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

    pub fn expect_any(&mut self) -> SyntaxResult<SpannedToken> {
        if let Some(x) = self.inner.pop() {
            Ok(x)
        } else {
            Err(self.eof())
        }
    }
}
