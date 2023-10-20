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

use crate::{tokenizer::*, Token};

use leo_ast::*;
use leo_errors::{emitter::Handler, ParserError, ParserWarning, Result};
use leo_span::{Span, Symbol};

use std::{fmt::Display, mem};

/// Stores a program in tokenized format plus additional context.
/// May be converted into a [`Program`] AST by parsing all tokens.
pub(crate) struct ParserContext<'a> {
    /// Handler used to side-channel emit errors from the parser.
    pub(crate) handler: &'a Handler,
    /// Counter used to generate unique node ids.
    pub(crate) node_builder: &'a NodeBuilder,
    /// All un-bumped tokens.
    tokens: Vec<SpannedToken>,
    /// The current token, i.e., if `p.tokens = ['3', *, '4']`,
    /// then after a `p.bump()`, we'll have `p.token = '3'`.
    pub(crate) token: SpannedToken,
    /// The previous token, i.e., if `p.tokens = ['3', *, '4']`,
    /// then after two `p.bump()`s, we'll have `p.token = '*'` and `p.prev_token = '3'`.
    pub(crate) prev_token: SpannedToken,
    /// true if parsing an expression for if and loop statements -- means struct inits are not legal
    pub(crate) disallow_struct_construction: bool,
    /// true if parsing an identifier inside an input file.
    pub(crate) allow_identifier_underscores: bool,
}

/// Dummy span used to appease borrow checker.
const DUMMY_EOF: SpannedToken = SpannedToken { token: Token::Eof, span: Span::dummy() };

impl<'a> ParserContext<'a> {
    /// Returns a new [`ParserContext`] type given a vector of tokens.
    pub fn new(handler: &'a Handler, node_builder: &'a NodeBuilder, mut tokens: Vec<SpannedToken>) -> Self {
        // Strip out comments.
        tokens.retain(|x| !matches!(x.token, Token::CommentLine(_) | Token::CommentBlock(_)));
        // For performance we reverse so that we get cheap `.pop()`s.
        tokens.reverse();

        let token = SpannedToken::dummy();
        let mut p = Self {
            handler,
            node_builder,
            disallow_struct_construction: false,
            allow_identifier_underscores: false,
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
    pub(crate) fn bump(&mut self) {
        // Probably a bug (infinite loop), as the previous token was already EOF.
        if let Token::Eof = self.prev_token.token {
            panic!("attempted to bump the parser past EOF (may be stuck in a loop)");
        }

        // Extract next token, or `Eof` if there was none.
        let next_token = self.tokens.pop().unwrap_or(SpannedToken { token: Token::Eof, span: self.token.span });

        // Set the new token.
        self.prev_token = mem::replace(&mut self.token, next_token);
    }

    /// Checks whether the current token is `tok`.
    pub(super) fn check(&self, tok: &Token) -> bool {
        &self.token.token == tok
    }

    /// Checks whether the current token is a `Token::Int(_)`.
    pub(super) fn check_int(&self) -> bool {
        matches!(&self.token.token, Token::Integer(_))
    }

    /// Returns `true` if the next token is equal to the given token.
    /// Advances the parser to the next token.
    pub(super) fn eat(&mut self, token: &Token) -> bool {
        self.check(token).then(|| self.bump()).is_some()
    }

    /// Look-ahead `dist` tokens of `self.token` and get access to that token there.
    /// When `dist == 0` then the current token is looked at.
    pub(super) fn look_ahead<'s, R>(&'s self, dist: usize, looker: impl FnOnce(&'s SpannedToken) -> R) -> R {
        if dist == 0 {
            return looker(&self.token);
        }

        let idx = match self.tokens.len().checked_sub(dist) {
            None => return looker(&DUMMY_EOF),
            Some(idx) => idx,
        };

        looker(self.tokens.get(idx).unwrap_or(&DUMMY_EOF))
    }

    /// Emit the error `err`.
    pub(super) fn emit_err(&self, err: ParserError) {
        self.handler.emit_err(err);
    }

    /// Emit the warning `warning`.
    pub(super) fn emit_warning(&self, warning: ParserWarning) {
        self.handler.emit_warning(warning.into());
    }

    /// Returns true if the next token exists.
    pub(crate) fn has_next(&self) -> bool {
        !matches!(self.token.token, Token::Eof)
    }

    /// At the previous token, return and make an identifier with `name`.
    fn mk_ident_prev(&self, name: Symbol) -> Identifier {
        let span = self.prev_token.span;
        Identifier { name, span, id: self.node_builder.next_id() }
    }

    /// Eats the next token if its an identifier and returns it.
    pub(super) fn eat_identifier(&mut self) -> Option<Identifier> {
        if let Token::Identifier(name) = self.token.token {
            self.bump();
            return Some(self.mk_ident_prev(name));
        }
        None
    }

    /// Expects an [`Identifier`], or errors.
    pub(super) fn expect_identifier(&mut self) -> Result<Identifier> {
        self.eat_identifier()
            .ok_or_else(|| ParserError::unexpected_str(&self.token.token, "identifier", self.token.span).into())
    }

    ///
    /// Removes the next token if it is a [`Token::Integer(_)`] and returns it, or [None] if
    /// the next token is not a [`Token::Integer(_)`] or if the next token does not exist.
    ///
    pub fn eat_whole_number(&mut self) -> Result<(NonNegativeNumber, Span)> {
        if let Token::Integer(value) = &self.token.token {
            let value = value.clone();
            self.bump();
            // Reject value if the length is over 2 and the first character is 0
            if (value.len() > 1 && value.starts_with('0')) || value.contains('_') {
                return Err(ParserError::tuple_index_must_be_whole_number(&self.token.token, self.token.span).into());
            }

            Ok((NonNegativeNumber::from(value), self.prev_token.span))
        } else {
            Err(ParserError::unexpected(&self.token.token, "integer literal", self.token.span).into())
        }
    }

    /// Eats any of the given `tokens`, returning `true` if anything was eaten.
    pub(super) fn eat_any(&mut self, tokens: &[Token]) -> bool {
        tokens.iter().any(|x| self.check(x)).then(|| self.bump()).is_some()
    }

    /// Returns an unexpected error at the current token.
    pub(super) fn unexpected<T>(&self, expected: impl Display) -> Result<T> {
        Err(ParserError::unexpected(&self.token.token, expected, self.token.span).into())
    }

    /// Eats the expected `token`, or errors.
    pub(super) fn expect(&mut self, token: &Token) -> Result<Span> {
        if self.eat(token) { Ok(self.prev_token.span) } else { self.unexpected(token) }
    }

    /// Eats one of the expected `tokens`, or errors.
    pub(super) fn expect_any(&mut self, tokens: &[Token]) -> Result<Span> {
        if self.eat_any(tokens) {
            Ok(self.prev_token.span)
        } else {
            self.unexpected(tokens.iter().map(|x| format!("'{x}'")).collect::<Vec<_>>().join(", "))
        }
    }

    /// Parses a list of `T`s using `inner`
    /// The opening and closing delimiters are `bra` and `ket`,
    /// and elements in the list are optionally separated by `sep`.
    /// When `(list, true)` is returned, `sep` was a terminator.
    pub(super) fn parse_list<T>(
        &mut self,
        delimiter: Delimiter,
        sep: Option<Token>,
        mut inner: impl FnMut(&mut Self) -> Result<Option<T>>,
    ) -> Result<(Vec<T>, bool, Span)> {
        let (open, close) = delimiter.open_close_pair();
        let mut list = Vec::new();
        let mut trailing = false;

        // Parse opening delimiter.
        let open_span = self.expect(&open)?;

        while !self.check(&close) {
            // Parse the element. We allow inner parser recovery through the `Option`.
            if let Some(elem) = inner(self)? {
                list.push(elem);
            }
            // Parse the separator, if any.
            if sep.as_ref().filter(|sep| !self.eat(sep)).is_some() {
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
        self.parse_list(Delimiter::Parenthesis, Some(Token::Comma), f)
    }

    /// Parse a list separated by `,` and delimited by brackets.
    pub(super) fn parse_bracket_comma_list<T>(
        &mut self,
        f: impl FnMut(&mut Self) -> Result<Option<T>>,
    ) -> Result<(Vec<T>, bool, Span)> {
        self.parse_list(Delimiter::Bracket, Some(Token::Comma), f)
    }

    /// Returns true if the current token is `(`.
    pub(super) fn peek_is_left_par(&self) -> bool {
        matches!(self.token.token, Token::LeftParen)
    }
}
