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

use crate::{assert_no_whitespace, tokenizer::*, Token, KEYWORD_TOKENS};

use leo_ast::*;
use leo_errors::emitter::Handler;
use leo_errors::{ParserError, Result};
use leo_span::{Span, Symbol};

use std::fmt::Display;
use std::mem;

/// Stores a program in tokenized format plus additional context.
/// May be converted into a [`Program`] AST by parsing all tokens.
pub struct ParserContext<'a> {
    /// Handler used to side-channel emit errors from the parser.
    pub(crate) handler: &'a Handler,
    /// All un-bumped tokens.
    tokens: Vec<SpannedToken>,
    /// The current token, i.e., if `p.tokens = ['3', *, '4']`,
    /// then after a `p.bump()`, we'll have `p.token = '3'`.
    pub(crate) token: SpannedToken,
    /// The previous token, i.e., if `p.tokens = ['3', *, '4']`,
    /// then after two `p.bump()`s, we'll have `p.token = '*'` and `p.prev_token = '3'`.
    pub(crate) prev_token: SpannedToken,

    // true if parsing an expression for if and loop statements -- means circuit inits are not legal
    pub(crate) disallow_circuit_construction: bool,
}

impl<'a> ParserContext<'a> {
    /// Returns a new [`ParserContext`] type given a vector of tokens.
    pub fn new(handler: &'a Handler, mut tokens: Vec<SpannedToken>) -> Self {
        // Strip out comments.
        tokens.retain(|x| !matches!(x.token, Token::CommentLine(_) | Token::CommentBlock(_)));
        // For performance we reverse so that we get cheap `.pop()`s.
        tokens.reverse();

        let token = SpannedToken::dummy();
        let mut p = Self {
            handler,
            disallow_circuit_construction: false,
            prev_token: token.clone(),
            token,
            tokens,
        };
        p.bump();
        p
    }

    /// Advances the parser cursor by one token.
    ///
    /// So e.g., if we had `previous = A`, `current = B`, and `tokens = [C, D, E]`,
    /// then after `p.bump()`, the state will be `previous = B`, `current = C`, and `tokens = [D, E]`.
    pub fn bump(&mut self) {
        // Probably a bug (infinite loop), as the previous token was already EOF.
        if let Token::Eof = self.prev_token.token {
            panic!("attempted to bump the parser past EOF (may be stuck in a loop)");
        }

        // Extract next token, or `Eof` if there was none.
        let next_token = self.tokens.pop().unwrap_or_else(|| SpannedToken {
            token: Token::Eof,
            span: self.token.span.clone(),
        });

        // Set the new token.
        self.prev_token = mem::replace(&mut self.token, next_token);
    }

    /// Checks whether the current token is `token`.
    pub fn check(&self, tok: &Token) -> bool {
        &self.token.token == tok
    }

    /// Removes the next token if it exists and returns it, or [None] if
    /// the next token does not exist.
    pub fn eat(&mut self, token: &Token) -> bool {
        self.check(token).then(|| self.bump()).is_some()
    }

    /// Look-ahead `dist` tokens of `self.token` and get access to that token there.
    /// When `dist == 0` then the current token is looked at.
    pub fn look_ahead<R>(&self, dist: usize, looker: impl FnOnce(&SpannedToken) -> R) -> R {
        if dist == 0 {
            return looker(&self.token);
        }

        let eof = SpannedToken {
            token: Token::Eof,
            span: Span::dummy(),
        };

        let idx = match self.tokens.len().checked_sub(dist) {
            None => return looker(&eof),
            Some(idx) => idx,
        };

        looker(self.tokens.get(idx).unwrap_or_else(|| &eof))
    }

    /// Emit the error `err`.
    pub(crate) fn emit_err(&self, err: ParserError) {
        self.handler.emit_err(err.into());
    }

    /// Returns true if the next token exists.
    pub fn has_next(&self) -> bool {
        !matches!(self.token.token, Token::Eof)
    }

    /// At the previous token, return and make an identifier with `name`.
    fn mk_ident_prev(&self, name: Symbol) -> Identifier {
        let span = self.prev_token.span.clone();
        Identifier { name, span }
    }

    /// Eats the next token if its an identifier and returns it.
    pub fn eat_identifier(&mut self) -> Option<Identifier> {
        if let Token::Ident(name) = self.token.token {
            self.bump();
            return Some(self.mk_ident_prev(name));
        }
        None
    }

    /// Expects an identifier, "loosely" speaking, or errors.
    ///
    /// This could be either a keyword, integer, or a normal identifier.
    pub fn expect_loose_identifier(&mut self) -> Result<Identifier> {
        if self.eat_any(KEYWORD_TOKENS) {
            return Ok(self.mk_ident_prev(self.prev_token.token.keyword_to_symbol().unwrap()));
        }
        if let Some(int) = self.eat_int() {
            return Ok(self.mk_ident_prev(Symbol::intern(&int.value)));
        }
        self.expect_ident()
    }

    /// Expects an [`Identifier`], or errors.
    pub fn expect_ident(&mut self) -> Result<Identifier> {
        self.eat_identifier()
            .ok_or_else(|| ParserError::unexpected_str(&self.token.token, "ident", &self.token.span).into())
    }

    ///
    /// Returns a reference to the next token if it is a [`GroupCoordinate`], or [None] if
    /// the next token is not a [`GroupCoordinate`].
    ///
    fn peek_group_coordinate(&self, dist: &mut usize) -> Option<GroupCoordinate> {
        let (advanced, gc) = self.look_ahead(*dist, |t0| match &t0.token {
            Token::Add => Some((1, GroupCoordinate::SignHigh)),
            Token::Minus => self.look_ahead(*dist + 1, |t1| match &t1.token {
                Token::Int(value) => Some((2, GroupCoordinate::Number(format!("-{}", value), t1.span.clone()))),
                _ => Some((1, GroupCoordinate::SignLow)),
            }),
            Token::Underscore => Some((1, GroupCoordinate::Inferred)),
            Token::Int(value) => Some((1, GroupCoordinate::Number(value.clone(), t0.span.clone()))),
            _ => None,
        })?;
        *dist += advanced;
        Some(gc)
    }

    /// Returns `true` if the next token is Function or if it is a Const followed by Function.
    /// Returns `false` otherwise.
    pub fn peek_is_function(&self) -> bool {
        matches!(
            (&self.token.token, self.tokens.last().map(|t| &t.token)),
            (Token::Function, _) | (Token::Const, Some(Token::Function))
        )
    }

    /// Removes the next two tokens if they are a pair of [`GroupCoordinate`] and returns them,
    /// or [None] if the next token is not a [`GroupCoordinate`].
    pub fn eat_group_partial(&mut self) -> Option<Result<GroupTuple>> {
        assert!(self.check(&Token::LeftParen)); // `(`.

        // Peek at first gc.
        let start_span = self.look_ahead(1, |t| t.span.clone());
        let mut dist = 1; // 0th is `(` so 1st is first gc's start.
        let first_gc = self.peek_group_coordinate(&mut dist)?;

        let check_ahead = |d, token: &_| self.look_ahead(d, |t| (&t.token == token).then(|| t.span.clone()));

        // Peek at `,`.
        check_ahead(dist, &Token::Comma)?;
        dist += 1; // Standing at `,` so advance one for next gc's start.

        // Peek at second gc.
        let second_gc = self.peek_group_coordinate(&mut dist)?;

        // Peek at `)`.
        let right_paren_span = check_ahead(dist, &Token::RightParen)?;
        dist += 1; // Standing at `)` so advance one for 'group'.

        // Peek at `group`.
        let end_span = check_ahead(dist, &Token::Group)?;
        dist += 1; // Standing at `)` so advance one for 'group'.

        // Eat everything so that this isn't just peeking.
        for _ in 0..dist {
            self.bump();
        }

        if let Err(e) = assert_no_whitespace(
            &right_paren_span,
            &end_span,
            &format!("({},{})", first_gc, second_gc),
            "group",
        ) {
            return Some(Err(e));
        }

        let gt = GroupTuple {
            span: start_span + end_span,
            x: first_gc,
            y: second_gc,
        };

        Some(Ok(gt))
    }

    /// Eats the next token if it is a [`Token::Int(_)`] and returns it.
    pub fn eat_int(&mut self) -> Option<PositiveNumber> {
        if let Token::Int(value) = &self.token.token {
            let value = value.clone();
            self.bump();
            return Some(PositiveNumber { value });
        }
        None
    }

    /// Eats any of the given `tokens`, returning `true` if anything was eaten.
    pub fn eat_any(&mut self, tokens: &[Token]) -> bool {
        tokens.iter().any(|x| self.check(x)).then(|| self.bump()).is_some()
    }

    /// Returns an unexpected error at the current token.
    fn unexpected<T>(&self, expected: impl Display) -> Result<T> {
        Err(ParserError::unexpected(&self.token.token, expected, &self.token.span).into())
    }

    /// Eats the expected `token`, or errors.
    pub fn expect(&mut self, token: &Token) -> Result<Span> {
        if self.eat(token) {
            Ok(self.prev_token.span.clone())
        } else {
            self.unexpected(token)
        }
    }

    /// Eats one of the expected `tokens`, or errors.
    pub fn expect_any(&mut self, tokens: &[Token]) -> Result<Span> {
        if self.eat_any(tokens) {
            Ok(self.prev_token.span.clone())
        } else {
            self.unexpected(tokens.iter().map(|x| format!("'{}'", x)).collect::<Vec<_>>().join(", "))
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
        let open_span = self.expect(&open)?;

        while !self.check(&close) {
            // Parse the element. We allow inner parser recovery through the `Option`.
            if let Some(elem) = inner(self)? {
                list.push(elem);
            }
            // Parse the separator.
            if !self.eat(&sep) {
                trailing = false;
                break;
            }

            trailing = true;
        }

        // Parse closing delimiter.
        let span = open_span + self.expect(&close)?;

        Ok((list, trailing, span))
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
        matches!(self.token.token, Token::LeftParen)
    }
}
