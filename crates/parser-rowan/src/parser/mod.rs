// Copyright (C) 2019-2026 Provable Inc.
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

//! Parser infrastructure for the rowan-based Leo parser.
//!
//! This module provides the core parser state and helper methods for building
//! a rowan syntax tree from a token stream. The parser uses a hand-written
//! recursive descent approach with support for error recovery.

mod expressions;
mod grammar;
mod items;
mod statements;
mod types;

use crate::{
    SyntaxNode,
    lexer::{LexError, Token},
    syntax_kind::{SyntaxKind, SyntaxKind::*},
};
pub use grammar::{parse_expression_entry, parse_file, parse_module_entry, parse_statement_entry};
use rowan::{Checkpoint, GreenNode, GreenNodeBuilder, TextRange, TextSize};

// =============================================================================
// Parse Result
// =============================================================================

/// An error encountered during parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    /// A description of the error.
    pub message: String,
    /// The source range where the error occurred.
    ///
    /// For "expected X" errors, this covers the found token so that
    /// diagnostic carets underline it. For recovery errors, this spans the
    /// tokens wrapped in the ERROR node.
    pub range: TextRange,
    /// The token that was found (for "expected X, found Y" errors).
    pub found: Option<String>,
    /// The list of expected tokens (for structured error messages).
    pub expected: Vec<String>,
}

/// The result of a parse operation.
///
/// A parse result always contains a syntax tree, even if errors occurred.
/// This is essential for IDE use cases where we need to provide feedback
/// even on incomplete or invalid code.
pub struct Parse {
    /// The green tree (immutable, cheap to clone).
    green: GreenNode,
    /// Errors encountered during parsing.
    errors: Vec<ParseError>,
    /// Errors encountered during lexing.
    lex_errors: Vec<LexError>,
}

impl Parse {
    /// Get the root syntax node.
    pub fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green.clone())
    }

    /// Get the parse errors.
    pub fn errors(&self) -> &[ParseError] {
        &self.errors
    }

    /// Get the lexer errors.
    pub fn lex_errors(&self) -> &[LexError] {
        &self.lex_errors
    }

    /// Check if the parse was successful (no errors).
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty() && self.lex_errors.is_empty()
    }

    /// Convert to a Result, returning the syntax node on success or errors on failure.
    pub fn ok(self) -> Result<SyntaxNode, Vec<ParseError>> {
        if self.errors.is_empty() && self.lex_errors.is_empty() { Ok(self.syntax()) } else { Err(self.errors) }
    }
}

// =============================================================================
// Marker Types
// =============================================================================

/// A marker for a node that is currently being parsed.
///
/// Call `complete()` to finish the node with a specific kind, or `abandon()`
/// to cancel without creating a node. Dropping a `Marker` without calling
/// either method will panic in debug builds.
pub struct Marker {
    /// The checkpoint position where this node started.
    checkpoint: Checkpoint,
    /// Debug bomb to catch forgotten markers.
    #[cfg(debug_assertions)]
    completed: bool,
}

impl Marker {
    fn new(checkpoint: Checkpoint) -> Self {
        Self {
            checkpoint,
            #[cfg(debug_assertions)]
            completed: false,
        }
    }

    /// Finish this node with the given kind.
    #[allow(unused_mut)]
    pub fn complete(mut self, p: &mut Parser, kind: SyntaxKind) -> CompletedMarker {
        #[cfg(debug_assertions)]
        {
            self.completed = true;
        }
        p.builder.start_node_at(self.checkpoint, kind.into());
        p.builder.finish_node();
        CompletedMarker { _checkpoint: self.checkpoint, kind }
    }

    /// Abandon this marker without creating a node.
    #[allow(unused_mut)] // `mut` needed for debug_assertions cfg
    pub fn abandon(mut self, _p: &mut Parser) {
        #[cfg(debug_assertions)]
        {
            self.completed = true;
        }
        // Nothing to do - we just don't create the node
    }
}

#[cfg(debug_assertions)]
impl Drop for Marker {
    fn drop(&mut self) {
        if !self.completed && !std::thread::panicking() {
            panic!("Marker was dropped without being completed or abandoned");
        }
    }
}

/// A marker for a completed node.
///
/// Can be used with `precede()` to wrap the node in an outer node retroactively.
/// This is essential for left-associative operators in Pratt parsing.
#[derive(Clone, Copy)]
pub struct CompletedMarker {
    /// The checkpoint where this node started (for precede).
    _checkpoint: Checkpoint,
    /// The kind of the completed node.
    kind: SyntaxKind,
}

impl CompletedMarker {
    /// Create a new marker that will wrap this completed node.
    ///
    /// Used for left-associative operators: parse LHS, see operator, then
    /// wrap LHS in a binary expression node.
    pub fn precede(self, _p: &mut Parser) -> Marker {
        // We need a new checkpoint at the same position as the completed node.
        // Since GreenNodeBuilder doesn't expose the old checkpoint position,
        // we use start_node_at with the saved checkpoint.
        Marker::new(self._checkpoint)
    }

    /// Get the kind of the completed node.
    pub fn kind(&self) -> SyntaxKind {
        self.kind
    }
}

// =============================================================================
// Parser
// =============================================================================

/// The parser state.
///
/// This struct maintains the state needed to build a rowan syntax tree from
/// a token stream. It provides methods for token inspection, consumption,
/// and tree building.
pub struct Parser<'t, 's> {
    /// The source text being parsed.
    source: &'s str,
    /// The token stream (from the lexer).
    tokens: &'t [Token],
    /// Current position in the token stream.
    pos: usize,
    /// Current byte offset into the source text.
    byte_offset: usize,
    /// The green tree builder.
    builder: GreenNodeBuilder<'static>,
    /// Accumulated parse errors.
    errors: Vec<ParseError>,
    /// Whether the parser is currently in an error state.
    ///
    /// When `true`, subsequent errors are suppressed until recovery completes.
    /// This prevents cascading errors from a single syntactic mistake.
    erroring: bool,
}

impl<'t, 's> Parser<'t, 's> {
    /// Create a new parser for the given source and tokens.
    pub fn new(source: &'s str, tokens: &'t [Token]) -> Self {
        Self {
            source,
            tokens,
            pos: 0,
            byte_offset: 0,
            builder: GreenNodeBuilder::new(),
            errors: Vec::new(),
            erroring: false,
        }
    }

    // =========================================================================
    // Marker-based Node Building
    // =========================================================================

    /// Start a new node, returning a Marker.
    ///
    /// The marker must be completed with `complete()` or abandoned with
    /// `abandon()`. Dropping it without doing either will panic.
    pub fn start(&mut self) -> Marker {
        Marker::new(self.builder.checkpoint())
    }

    // =========================================================================
    // Token Inspection
    // =========================================================================

    /// Get the kind of the current token (or EOF if at end).
    pub fn current(&self) -> SyntaxKind {
        self.nth(0)
    }

    /// Look ahead by `n` tokens and return the kind (skipping trivia).
    pub fn nth(&self, n: usize) -> SyntaxKind {
        self.nth_non_trivia(n)
    }

    /// Look ahead by `n` non-trivia tokens.
    fn nth_non_trivia(&self, n: usize) -> SyntaxKind {
        let mut pos = self.pos;
        let mut count = 0;
        while pos < self.tokens.len() {
            let kind = self.tokens[pos].kind;
            if !kind.is_trivia() {
                if count == n {
                    return kind;
                }
                count += 1;
            }
            pos += 1;
        }
        EOF
    }

    /// Get the kind of the current token including trivia.
    pub fn current_including_trivia(&self) -> SyntaxKind {
        self.tokens.get(self.pos).map(|t| t.kind).unwrap_or(EOF)
    }

    /// Check if the current token matches the given kind.
    pub fn at(&self, kind: SyntaxKind) -> bool {
        self.current() == kind
    }

    /// Check if the current token matches any of the given kinds.
    pub fn at_any(&self, kinds: &[SyntaxKind]) -> bool {
        kinds.contains(&self.current())
    }

    /// Check if we're at the end of the token stream.
    pub fn at_eof(&self) -> bool {
        self.at(EOF)
    }

    /// Get the text of the current token.
    pub fn current_text(&self) -> &'s str {
        if self.pos >= self.tokens.len() {
            return "";
        }
        let len = self.tokens[self.pos].len as usize;
        &self.source[self.byte_offset..self.byte_offset + len]
    }

    /// Get the `TextRange` covering the current non-trivia token.
    ///
    /// Walks past trivia to find the next meaningful token and returns its
    /// range. At EOF, returns a zero-length range at the current byte offset.
    fn current_token_range(&self) -> TextRange {
        let mut pos = self.pos;
        let mut offset = self.byte_offset;
        while pos < self.tokens.len() {
            let token = &self.tokens[pos];
            if !token.kind.is_trivia() {
                let start = TextSize::new(offset as u32);
                let end = TextSize::new(offset as u32 + token.len);
                return TextRange::new(start, end);
            }
            offset += token.len as usize;
            pos += 1;
        }
        let pos = TextSize::new(offset as u32);
        TextRange::empty(pos)
    }

    // =========================================================================
    // Token Consumption
    // =========================================================================

    /// Consume the current token and add it to the tree.
    pub fn bump(&mut self) {
        self.bump_raw();
    }

    /// Consume the current token without skipping trivia first.
    fn bump_raw(&mut self) {
        if self.pos >= self.tokens.len() {
            return;
        }
        let token = &self.tokens[self.pos];

        // Lex error tokens enter error state to suppress cascade parse errors.
        if token.kind == ERROR {
            self.erroring = true;
        }

        let text = &self.source[self.byte_offset..self.byte_offset + token.len as usize];
        self.builder.token(token.kind.into(), text);
        self.byte_offset += token.len as usize;
        self.pos += 1;
    }

    /// Skip trivia tokens, adding them to the tree.
    pub fn skip_trivia(&mut self) {
        while self.current_including_trivia().is_trivia() {
            self.bump_raw();
        }
    }

    /// Consume the current token if it matches the given kind.
    /// Returns true if consumed.
    pub fn eat(&mut self, kind: SyntaxKind) -> bool {
        self.skip_trivia();
        if self.current_including_trivia() == kind {
            self.bump_raw();
            true
        } else {
            false
        }
    }

    /// Consume any trivia and then the next token, regardless of kind.
    pub fn bump_any(&mut self) {
        self.skip_trivia();
        self.bump_raw();
    }

    // =========================================================================
    // Direct Node Building
    // =========================================================================

    /// Start a new node of the given kind.
    ///
    /// Used by error recovery. For general parsing, prefer `start()` and
    /// `Marker::complete()`.
    pub fn start_node(&mut self, kind: SyntaxKind) {
        self.builder.start_node(kind.into());
    }

    /// Finish the current node.
    pub fn finish_node(&mut self) {
        self.builder.finish_node();
    }

    /// Create a checkpoint for later wrapping.
    pub fn checkpoint(&self) -> Checkpoint {
        self.builder.checkpoint()
    }

    /// Start a node at a previous checkpoint.
    pub fn start_node_at(&mut self, checkpoint: Checkpoint, kind: SyntaxKind) {
        self.builder.start_node_at(checkpoint, kind.into());
    }

    // =========================================================================
    // Error Handling
    // =========================================================================

    /// Expect a token of the given kind.
    ///
    /// If the current token matches, it is consumed. Otherwise, an error
    /// is recorded but no token is consumed.
    pub fn expect(&mut self, kind: SyntaxKind) -> bool {
        if self.eat(kind) {
            true
        } else {
            if !self.erroring {
                let found_text = self.current_text().to_string();
                let expected_name = kind.user_friendly_name();
                self.errors.push(ParseError {
                    message: format!("expected {}", expected_name),
                    range: self.current_token_range(),
                    found: Some(found_text),
                    expected: vec![expected_name.to_string()],
                });
                self.erroring = true;
            }
            false
        }
    }

    /// Record a parse error at the current position.
    pub fn error(&mut self, message: impl std::fmt::Display) {
        if self.erroring {
            return;
        }
        let found_text = if !self.at_eof() { Some(self.current_text().to_string()) } else { None };
        self.errors.push(ParseError {
            message: message.to_string(),
            range: self.current_token_range(),
            found: found_text,
            expected: vec![],
        });
        self.erroring = true;
    }

    /// Record an "unexpected token" error with explicit expected tokens.
    ///
    /// This sets both `found` and `expected` fields, allowing rowan.rs to emit
    /// a ParserError::unexpected instead of ParserError::custom.
    /// The `found_kind` parameter should be the SyntaxKind of the unexpected token.
    pub fn error_unexpected(&mut self, found_kind: SyntaxKind, expected: &[&str]) {
        if self.erroring {
            return;
        }
        let range = self.current_token_range();
        // For 'found', use the actual token text for identifiers and numbers
        // (since "an identifier" is less helpful than showing the actual name).
        // For other tokens, use user_friendly_name with quotes stripped.
        let found_text = if matches!(found_kind, IDENT | INTEGER) {
            let start = u32::from(range.start()) as usize;
            let end = u32::from(range.end()) as usize;
            if start < end && end <= self.source.len() {
                self.source[start..end].to_string()
            } else {
                found_kind.user_friendly_name().trim_matches('\'').to_string()
            }
        } else {
            found_kind.user_friendly_name().trim_matches('\'').to_string()
        };
        self.errors.push(ParseError {
            message: format!("expected {}", expected.join(", ")),
            range,
            found: Some(found_text),
            expected: expected.iter().map(|s| s.to_string()).collect(),
        });
        self.erroring = true;
    }

    /// Record a parse error spanning from `start` to the current position.
    fn error_span(&mut self, message: impl std::fmt::Display, start: usize) {
        if self.erroring {
            return;
        }
        self.errors.push(ParseError {
            message: message.to_string(),
            range: TextRange::new(TextSize::new(start as u32), TextSize::new(self.byte_offset as u32)),
            found: None,
            expected: vec![],
        });
        self.erroring = true;
    }

    /// Wrap unexpected tokens in an ERROR node until we reach a recovery point.
    ///
    /// This consumes tokens until we see one of the recovery tokens or EOF.
    /// If already at a recovery token, consumes it to ensure progress is made.
    /// The error range spans the tokens wrapped in the ERROR node.
    pub fn error_recover(&mut self, message: &str, recovery: &[SyntaxKind]) {
        let start = self.byte_offset;
        self.start_node(ERROR);
        self.skip_to_recovery(recovery);
        self.finish_node();

        // Record error with the span of consumed tokens
        self.error_span(message, start);

        // Recovery complete — allow new errors for the next construct.
        self.erroring = false;
    }

    /// Skip tokens until a recovery point, wrapping them in an ERROR node.
    /// Unlike `error_recover`, this does not emit an error message (useful when
    /// the caller has already reported the error).
    ///
    /// Balanced brace pairs (`{ ... }`) encountered during recovery are
    /// consumed as a unit so that we don't stop at an inner `}` that belongs
    /// to a nested block.
    pub fn recover(&mut self, recovery: &[SyntaxKind]) {
        self.start_node(ERROR);
        self.skip_to_recovery(recovery);
        self.finish_node();

        // Recovery complete — allow new errors for the next construct.
        self.erroring = false;
    }

    /// Skip tokens until we reach a recovery point or EOF.
    ///
    /// If already at a recovery token (other than `}`), consumes it to ensure
    /// progress. Balanced brace pairs are consumed as a unit so that we don't
    /// stop at an inner `}` that belongs to a nested block.
    fn skip_to_recovery(&mut self, recovery: &[SyntaxKind]) {
        if self.at_any(recovery) && !self.at(R_BRACE) && !self.at_eof() {
            self.bump_any();
        }
        let mut brace_depth: u32 = 0;
        while !self.at_eof() {
            match self.current() {
                L_BRACE => brace_depth += 1,
                R_BRACE if brace_depth > 0 => brace_depth -= 1,
                kind if brace_depth == 0 && recovery.contains(&kind) => break,
                _ => {}
            }
            self.bump_any();
        }
    }

    /// Create an ERROR node containing the current token.
    /// The error range spans the single consumed token.
    pub fn error_and_bump(&mut self, message: &str) {
        let start = self.byte_offset;
        self.start_node(ERROR);
        self.bump_any();
        self.finish_node();
        self.error_span(message, start);

        // Consumed the bad token — allow new errors for the next construct.
        self.erroring = false;
    }

    // =========================================================================
    // Completion
    // =========================================================================

    /// Get the current number of accumulated parse errors.
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    /// Finish parsing and return the parse result.
    pub fn finish(self, lex_errors: Vec<LexError>) -> Parse {
        Parse { green: self.builder.finish(), errors: self.errors, lex_errors }
    }
}

// =============================================================================
// Recovery Token Sets
// =============================================================================

/// Tokens that can start a statement (for recovery).
pub(crate) const STMT_RECOVERY: &[SyntaxKind] =
    &[KW_LET, KW_CONST, KW_RETURN, KW_IF, KW_FOR, KW_ASSERT, KW_ASSERT_EQ, KW_ASSERT_NEQ, L_BRACE, R_BRACE, SEMICOLON];

/// Tokens that can start a top-level item (for recovery).
pub(crate) const ITEM_RECOVERY: &[SyntaxKind] =
    &[KW_IMPORT, KW_PROGRAM, KW_FN, KW_STRUCT, KW_RECORD, KW_MAPPING, KW_STORAGE, KW_CONST, KW_FINAL, AT, R_BRACE];

/// Tokens that indicate we should stop expression recovery.
pub(crate) const EXPR_RECOVERY: &[SyntaxKind] = &[
    SEMICOLON,
    COMMA,
    R_PAREN,
    R_BRACKET,
    R_BRACE,
    KW_LET,
    KW_CONST,
    KW_RETURN,
    KW_IF,
    KW_FOR,
    KW_ASSERT,
    KW_ASSERT_EQ,
    KW_ASSERT_NEQ,
];

/// Tokens for recovering inside type parsing.
pub(crate) const TYPE_RECOVERY: &[SyntaxKind] =
    &[COMMA, GT, R_BRACKET, R_PAREN, L_BRACE, R_BRACE, EQ, SEMICOLON, ARROW];

/// Tokens for recovering inside parameter lists.
pub(crate) const PARAM_RECOVERY: &[SyntaxKind] = &[COMMA, R_PAREN, L_BRACE, ARROW];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex;
    use expect_test::{Expect, expect};

    /// Helper to parse source and format the tree for snapshot testing.
    fn check_parse(input: &str, parse_fn: impl FnOnce(&mut Parser), expect: Expect) {
        let (tokens, _) = lex(input);
        let mut parser = Parser::new(input, &tokens);
        parse_fn(&mut parser);
        let parse = parser.finish(vec![]);
        let output = format!("{:#?}", parse.syntax());
        expect.assert_eq(&output);
    }

    /// Helper to check parse errors.
    fn check_parse_errors(input: &str, parse_fn: impl FnOnce(&mut Parser), expect: Expect) {
        let (tokens, _) = lex(input);
        let mut parser = Parser::new(input, &tokens);
        parse_fn(&mut parser);
        let parse = parser.finish(vec![]);
        let output = parse
            .errors()
            .iter()
            .map(|e| format!("{}..{}:{}", u32::from(e.range.start()), u32::from(e.range.end()), e.message))
            .collect::<Vec<_>>()
            .join("\n");
        expect.assert_eq(&output);
    }

    // =========================================================================
    // Legacy tests (using start_node/finish_node)
    // =========================================================================

    #[test]
    fn parse_empty() {
        check_parse(
            "",
            |p| {
                p.start_node(ROOT);
                p.finish_node();
            },
            expect![[r#"
            ROOT@0..0
        "#]],
        );
    }

    #[test]
    fn parse_whitespace_only() {
        check_parse(
            "   ",
            |p| {
                p.start_node(ROOT);
                p.skip_trivia();
                p.finish_node();
            },
            expect![[r#"
            ROOT@0..3
              WHITESPACE@0..3 "   "
        "#]],
        );
    }

    #[test]
    fn parse_single_token() {
        check_parse(
            "foo",
            |p| {
                p.start_node(ROOT);
                p.skip_trivia();
                p.bump();
                p.finish_node();
            },
            expect![[r#"
            ROOT@0..3
              IDENT@0..3 "foo"
        "#]],
        );
    }

    #[test]
    fn parse_token_with_trivia() {
        check_parse(
            "  foo  ",
            |p| {
                p.start_node(ROOT);
                p.skip_trivia();
                p.bump();
                p.skip_trivia();
                p.finish_node();
            },
            expect![[r#"
            ROOT@0..7
              WHITESPACE@0..2 "  "
              IDENT@2..5 "foo"
              WHITESPACE@5..7 "  "
        "#]],
        );
    }

    #[test]
    fn parse_nested_nodes() {
        check_parse(
            "(a)",
            |p| {
                p.start_node(ROOT);
                p.start_node(PAREN_EXPR);
                p.eat(L_PAREN);
                p.skip_trivia();
                p.start_node(PATH_EXPR);
                p.bump();
                p.finish_node();
                p.eat(R_PAREN);
                p.finish_node();
                p.finish_node();
            },
            expect![[r#"
            ROOT@0..3
              PAREN_EXPR@0..3
                L_PAREN@0..1 "("
                PATH_EXPR@1..2
                  IDENT@1..2 "a"
                R_PAREN@2..3 ")"
        "#]],
        );
    }

    #[test]
    fn parse_with_checkpoint() {
        check_parse(
            "1 + 2",
            |p| {
                p.start_node(ROOT);
                p.skip_trivia();
                let checkpoint = p.checkpoint();
                p.start_node(LITERAL_INT);
                p.bump();
                p.finish_node();
                p.skip_trivia();
                p.start_node_at(checkpoint, BINARY_EXPR);
                p.bump();
                p.skip_trivia();
                p.start_node(LITERAL_INT);
                p.bump();
                p.finish_node();
                p.finish_node();
                p.finish_node();
            },
            expect![[r#"
                ROOT@0..5
                  BINARY_EXPR@0..5
                    LITERAL_INT@0..1
                      INTEGER@0..1 "1"
                    WHITESPACE@1..2 " "
                    PLUS@2..3 "+"
                    WHITESPACE@3..4 " "
                    LITERAL_INT@4..5
                      INTEGER@4..5 "2"
            "#]],
        );
    }

    #[test]
    fn parse_expect_success() {
        check_parse(
            ";",
            |p| {
                p.start_node(ROOT);
                p.expect(SEMICOLON);
                p.finish_node();
            },
            expect![[r#"
            ROOT@0..1
              SEMICOLON@0..1 ";"
        "#]],
        );
    }

    #[test]
    fn parse_expect_failure() {
        check_parse_errors(
            "foo",
            |p| {
                p.start_node(ROOT);
                p.expect(SEMICOLON);
                p.finish_node();
            },
            expect![[r#"0..3:expected ';'"#]],
        );
    }

    #[test]
    fn parse_error_recover() {
        check_parse(
            "garbage ; ok",
            |p| {
                p.start_node(ROOT);
                p.error_recover("unexpected tokens", &[SEMICOLON]);
                p.eat(SEMICOLON);
                p.skip_trivia();
                p.bump();
                p.finish_node();
            },
            expect![[r#"
            ROOT@0..12
              ERROR@0..7
                IDENT@0..7 "garbage"
              WHITESPACE@7..8 " "
              SEMICOLON@8..9 ";"
              WHITESPACE@9..10 " "
              IDENT@10..12 "ok"
        "#]],
        );
    }

    #[test]
    fn parse_error_and_bump() {
        check_parse(
            "$",
            |p| {
                p.start_node(ROOT);
                p.skip_trivia();
                p.error_and_bump("unexpected token");
                p.finish_node();
            },
            expect![[r#"
            ROOT@0..1
              ERROR@0..1
                ERROR@0..1 "$"
        "#]],
        );
    }

    #[test]
    fn parse_at_and_eat() {
        check_parse(
            "let x",
            |p| {
                p.start_node(ROOT);
                assert!(p.at(KW_LET));
                assert!(!p.at(KW_CONST));
                p.eat(KW_LET);
                assert!(p.at(IDENT));
                p.skip_trivia();
                p.bump();
                p.finish_node();
            },
            expect![[r#"
            ROOT@0..5
              KW_LET@0..3 "let"
              WHITESPACE@3..4 " "
              IDENT@4..5 "x"
        "#]],
        );
    }

    #[test]
    fn parse_nth_lookahead() {
        check_parse(
            "a + b * c",
            |p| {
                p.start_node(ROOT);
                assert_eq!(p.nth(0), IDENT);
                assert_eq!(p.nth(1), PLUS);
                assert_eq!(p.nth(2), IDENT);
                assert_eq!(p.nth(3), STAR);
                assert_eq!(p.nth(4), IDENT);
                assert_eq!(p.nth(5), EOF);
                while !p.at_eof() {
                    p.bump_any();
                }
                p.finish_node();
            },
            expect![[r#"
            ROOT@0..9
              IDENT@0..1 "a"
              WHITESPACE@1..2 " "
              PLUS@2..3 "+"
              WHITESPACE@3..4 " "
              IDENT@4..5 "b"
              WHITESPACE@5..6 " "
              STAR@6..7 "*"
              WHITESPACE@7..8 " "
              IDENT@8..9 "c"
        "#]],
        );
    }

    // =========================================================================
    // Marker tests
    // =========================================================================

    #[test]
    fn marker_complete() {
        check_parse(
            "foo",
            |p| {
                let m = p.start();
                p.skip_trivia();
                p.bump();
                m.complete(p, ROOT);
            },
            expect![[r#"
            ROOT@0..3
              IDENT@0..3 "foo"
        "#]],
        );
    }

    #[test]
    fn marker_precede() {
        // Test precede for binary expression: parse "1 + 2"
        check_parse(
            "1 + 2",
            |p| {
                let root = p.start();
                p.skip_trivia();

                // Parse LHS
                let lhs_m = p.start();
                p.bump(); // 1
                let lhs = lhs_m.complete(p, LITERAL_INT);

                p.skip_trivia();

                // See operator - wrap LHS in binary expr
                let bin_m = lhs.precede(p);
                p.bump(); // +

                p.skip_trivia();

                // Parse RHS
                let rhs_m = p.start();
                p.bump(); // 2
                rhs_m.complete(p, LITERAL_INT);

                bin_m.complete(p, BINARY_EXPR);
                root.complete(p, ROOT);
            },
            expect![[r#"
                ROOT@0..5
                  BINARY_EXPR@0..5
                    LITERAL_INT@0..1
                      INTEGER@0..1 "1"
                    WHITESPACE@1..2 " "
                    PLUS@2..3 "+"
                    WHITESPACE@3..4 " "
                    LITERAL_INT@4..5
                      INTEGER@4..5 "2"
            "#]],
        );
    }

    #[test]
    fn marker_nested() {
        check_parse(
            "(a)",
            |p| {
                let root = p.start();
                let paren = p.start();
                p.eat(L_PAREN);
                p.skip_trivia();
                let name = p.start();
                p.bump();
                name.complete(p, PATH_EXPR);
                p.eat(R_PAREN);
                paren.complete(p, PAREN_EXPR);
                root.complete(p, ROOT);
            },
            expect![[r#"
            ROOT@0..3
              PAREN_EXPR@0..3
                L_PAREN@0..1 "("
                PATH_EXPR@1..2
                  IDENT@1..2 "a"
                R_PAREN@2..3 ")"
        "#]],
        );
    }

    // =========================================================================
    // Error Recovery Tests
    // =========================================================================
    //
    // These tests verify that the parser can recover from various error
    // conditions and continue parsing. The key invariants are:
    // 1. Parser never panics
    // 2. ERROR nodes contain malformed content
    // 3. Valid portions are parsed correctly
    // 4. Reasonable error messages are generated

    /// Helper to parse a file and return both tree and errors.
    fn parse_file(input: &str) -> Parse {
        let (tokens, _) = lex(input);
        let mut parser = Parser::new(input, &tokens);
        let root = parser.start();
        parser.parse_file_items();
        root.complete(&mut parser, ROOT);
        parser.finish(vec![])
    }

    /// Helper to parse a statement and return both tree and errors.
    fn parse_stmt_result(input: &str) -> Parse {
        let (tokens, _) = lex(input);
        let mut parser = Parser::new(input, &tokens);
        let root = parser.start();
        parser.parse_stmt();
        parser.skip_trivia();
        root.complete(&mut parser, ROOT);
        parser.finish(vec![])
    }

    #[test]
    fn recover_missing_semicolon_let() {
        let parse = parse_stmt_result("let x = 1");
        // Parser should complete but report an error
        assert!(!parse.errors().is_empty(), "should have errors");
        // Tree should still have a LET_STMT
        let tree = format!("{:#?}", parse.syntax());
        assert!(tree.contains("LET_STMT"), "tree should have LET_STMT");
    }

    #[test]
    fn recover_missing_semicolon_return() {
        let parse = parse_stmt_result("return 42");
        assert!(!parse.errors().is_empty(), "should have errors");
        let tree = format!("{:#?}", parse.syntax());
        assert!(tree.contains("RETURN_STMT"), "tree should have RETURN_STMT");
    }

    #[test]
    fn recover_missing_expr_in_let() {
        let parse = parse_stmt_result("let x = ;");
        assert!(!parse.errors().is_empty(), "should have errors");
        let tree = format!("{:#?}", parse.syntax());
        assert!(tree.contains("LET_STMT"), "tree should have LET_STMT");
    }

    #[test]
    fn recover_missing_type_in_let() {
        let parse = parse_stmt_result("let x: = 1;");
        assert!(!parse.errors().is_empty(), "should have errors");
        let tree = format!("{:#?}", parse.syntax());
        assert!(tree.contains("LET_STMT"), "tree should have LET_STMT");
    }

    #[test]
    fn recover_missing_condition_in_if() {
        // When the condition is missing and we have `if { }`, the `{` is parsed
        // as an empty block expression in the condition position. The parser
        // should still recover and produce an IF_STMT.
        let parse = parse_stmt_result("if { }");
        // May or may not have errors depending on interpretation
        let tree = format!("{:#?}", parse.syntax());
        assert!(tree.contains("IF_STMT"), "tree should have IF_STMT");
    }

    #[test]
    fn recover_missing_block_in_if() {
        let parse = parse_stmt_result("if x");
        assert!(!parse.errors().is_empty(), "should have errors");
        let tree = format!("{:#?}", parse.syntax());
        assert!(tree.contains("IF_STMT"), "tree should have IF_STMT");
    }

    #[test]
    fn recover_missing_range_in_for() {
        let parse = parse_stmt_result("for i in { }");
        assert!(!parse.errors().is_empty(), "should have errors");
        let tree = format!("{:#?}", parse.syntax());
        assert!(tree.contains("FOR_STMT"), "tree should have FOR_STMT");
    }

    #[test]
    fn recover_missing_operand_binary() {
        let parse = parse_stmt_result("let x = 1 + ;");
        assert!(!parse.errors().is_empty(), "should have errors");
        let tree = format!("{:#?}", parse.syntax());
        assert!(tree.contains("BINARY_EXPR"), "tree should have BINARY_EXPR");
    }

    #[test]
    fn recover_missing_operand_unary() {
        let parse = parse_stmt_result("let x = -;");
        assert!(!parse.errors().is_empty(), "should have errors");
        let tree = format!("{:#?}", parse.syntax());
        assert!(tree.contains("UNARY_EXPR"), "tree should have UNARY_EXPR");
    }

    #[test]
    fn recover_unclosed_paren() {
        let parse = parse_stmt_result("let x = (1 + 2;");
        assert!(!parse.errors().is_empty(), "should have errors");
        let tree = format!("{:#?}", parse.syntax());
        assert!(tree.contains("LET_STMT"), "tree should have LET_STMT");
    }

    #[test]
    fn recover_unclosed_bracket() {
        let parse = parse_stmt_result("let x = [1, 2;");
        assert!(!parse.errors().is_empty(), "should have errors");
        let tree = format!("{:#?}", parse.syntax());
        assert!(tree.contains("LET_STMT"), "tree should have LET_STMT");
    }

    #[test]
    fn recover_invalid_token_in_expr() {
        let parse = parse_stmt_result("let x = 1 @ 2;");
        assert!(!parse.errors().is_empty(), "should have errors");
        let tree = format!("{:#?}", parse.syntax());
        assert!(tree.contains("LET_STMT"), "tree should have LET_STMT");
    }

    #[test]
    fn recover_malformed_assert() {
        let parse = parse_stmt_result("assert();");
        assert!(!parse.errors().is_empty(), "should have errors");
        let tree = format!("{:#?}", parse.syntax());
        assert!(tree.contains("ASSERT_STMT"), "tree should have ASSERT_STMT");
    }

    #[test]
    fn recover_malformed_assert_eq() {
        let parse = parse_stmt_result("assert_eq(1);");
        assert!(!parse.errors().is_empty(), "should have errors");
        let tree = format!("{:#?}", parse.syntax());
        assert!(tree.contains("ASSERT_EQ_STMT"), "tree should have ASSERT_EQ_STMT");
    }

    #[test]
    fn recover_missing_assignment_rhs() {
        let parse = parse_stmt_result("x = ;");
        assert!(!parse.errors().is_empty(), "should have errors");
        let tree = format!("{:#?}", parse.syntax());
        assert!(tree.contains("ASSIGN_STMT"), "tree should have ASSIGN_STMT");
    }

    #[test]
    fn recover_malformed_function_missing_params() {
        let parse = parse_file("program test.aleo { fn foo { } }");
        assert!(!parse.errors().is_empty(), "should have errors");
        let tree = format!("{:#?}", parse.syntax());
        assert!(tree.contains("PROGRAM_DECL"), "tree should have PROGRAM_DECL");
        assert!(tree.contains("FUNCTION_DEF"), "tree should have FUNCTION_DEF");
    }

    #[test]
    fn recover_malformed_function_missing_body() {
        let parse = parse_file("program test.aleo { fn foo() }");
        assert!(!parse.errors().is_empty(), "should have errors");
        let tree = format!("{:#?}", parse.syntax());
        assert!(tree.contains("PROGRAM_DECL"), "tree should have PROGRAM_DECL");
        assert!(tree.contains("FUNCTION_DEF"), "tree should have FUNCTION_DEF");
    }

    #[test]
    fn recover_malformed_struct_field() {
        let parse = parse_file("program test.aleo { struct Foo { x, y: u32 } }");
        assert!(!parse.errors().is_empty(), "should have errors");
        let tree = format!("{:#?}", parse.syntax());
        assert!(tree.contains("STRUCT_DEF"), "tree should have STRUCT_DEF");
    }

    #[test]
    fn recover_malformed_mapping() {
        let parse = parse_file("program test.aleo { mapping balances: => u64; }");
        assert!(!parse.errors().is_empty(), "should have errors");
        let tree = format!("{:#?}", parse.syntax());
        assert!(tree.contains("MAPPING_DEF"), "tree should have MAPPING_DEF");
    }

    #[test]
    fn recover_multiple_errors_in_function() {
        let parse = parse_file(
            r#"program test.aleo {
                fn foo() {
                    let x = ;
                    let y: = 1;
                    return x +;
                }
            }"#,
        );
        // Should have multiple errors but still parse the structure
        assert!(parse.errors().len() >= 3, "should have at least 3 errors");
        let tree = format!("{:#?}", parse.syntax());
        assert!(tree.contains("FUNCTION_DEF"), "tree should have FUNCTION_DEF");
        assert!(tree.contains("LET_STMT"), "tree should have LET_STMT");
        assert!(tree.contains("RETURN_STMT"), "tree should have RETURN_STMT");
    }

    #[test]
    fn recover_valid_items_after_error() {
        let parse = parse_file(
            r#"program test.aleo {
                struct Invalid { x }
                struct Valid { y: u32 }
            }"#,
        );
        // Should have errors but parse both structs
        assert!(!parse.errors().is_empty(), "should have errors");
        let tree = format!("{:#?}", parse.syntax());
        // Both structs should be present
        let struct_count = tree.matches("STRUCT_DEF").count();
        assert_eq!(struct_count, 2, "should have 2 struct definitions");
    }

    #[test]
    fn recover_empty_tuple_pattern() {
        let parse = parse_stmt_result("let () = foo;");
        // Empty tuple pattern should work
        let tree = format!("{:#?}", parse.syntax());
        assert!(tree.contains("LET_STMT"), "tree should have LET_STMT");
        assert!(tree.contains("TUPLE_PATTERN"), "tree should have TUPLE_PATTERN");
    }

    #[test]
    fn recover_nested_errors() {
        // Errors within nested expressions
        let parse = parse_stmt_result("let x = ((1 + ) + 2);");
        assert!(!parse.errors().is_empty(), "should have errors");
        let tree = format!("{:#?}", parse.syntax());
        assert!(tree.contains("LET_STMT"), "tree should have LET_STMT");
        assert!(tree.contains("PAREN_EXPR"), "tree should have PAREN_EXPR");
    }

    #[test]
    fn recover_ternary_missing_then() {
        let parse = parse_stmt_result("let x = cond ? : 2;");
        assert!(!parse.errors().is_empty(), "should have errors");
        let tree = format!("{:#?}", parse.syntax());
        assert!(tree.contains("TERNARY_EXPR"), "tree should have TERNARY_EXPR");
    }

    #[test]
    fn recover_ternary_missing_else() {
        let parse = parse_stmt_result("let x = cond ? 1 :;");
        assert!(!parse.errors().is_empty(), "should have errors");
        let tree = format!("{:#?}", parse.syntax());
        assert!(tree.contains("TERNARY_EXPR"), "tree should have TERNARY_EXPR");
    }

    #[test]
    fn never_panics_on_garbage() {
        // Feed various garbage inputs - parser should never panic
        let garbage_inputs = [
            "@#$%^&*()",
            "{{{{{{",
            "}}}}}}",
            "let let let",
            "program { program { program",
            "struct struct struct",
            ";;;;;;",
            "++ -- ** //",
            "",
            "   \n\t\r  ",
            "🦀🦀🦀", // Unicode
        ];

        for input in garbage_inputs {
            // This should never panic
            let _ = parse_file(input);
        }
    }
}
