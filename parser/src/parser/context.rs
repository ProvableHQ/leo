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

use crate::{tokenizer::*, Token};

use leo_errors::emitter::Handler;

use leo_span::Span;

use crate::common::*;

/// Stores a program in tokenized format plus additional context.
/// May be converted into a [`Program`] AST by parsing all tokens.
pub struct ParserContext<'a> {
    #[allow(dead_code)]
    pub(crate) handler: &'a Handler,
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

impl<'a> Context for ParserContext<'a> {
    fn tokens(&self) -> &Vec<SpannedToken> {
        &self.tokens
    }

    fn end_span(&self) -> Span {
        self.end_span.clone()
    }

    fn tokens_mut(&mut self) -> &mut Vec<SpannedToken> {
        &mut self.tokens
    }

    fn handler(&self) -> &Handler {
        &self.handler
    }
}

impl<'a> ParserContext<'a> {
    ///
    /// Returns a new [`ParserContext`] type given a vector of tokens.
    ///
    pub fn new(handler: &'a Handler, mut tokens: Vec<SpannedToken>) -> Self {
        tokens.reverse();
        // todo: performance optimization here: drain filter
        tokens = tokens
            .into_iter()
            .filter(|x| !matches!(x.token, Token::CommentLine(_) | Token::CommentBlock(_)))
            .collect();
        Self {
            handler,
            end_span: tokens
                .iter()
                .find(|x| !x.span.content.trim().is_empty())
                .map(|x| x.span.clone())
                .unwrap_or_default(),
            tokens,
            fuzzy_struct_state: false,
        }
    }
}
