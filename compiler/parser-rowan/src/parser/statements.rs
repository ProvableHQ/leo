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

//! Statement parsing for the Leo language.
//!
//! This module implements parsing for all Leo statement forms:
//! - Let and const bindings
//! - Assignments (including compound assignments)
//! - Control flow (if, for)
//! - Return statements
//! - Assert statements
//! - Expression statements
//! - Blocks

use super::{CompletedMarker, EXPR_RECOVERY, Parser, STMT_RECOVERY, TYPE_RECOVERY, expressions::ExprOpts};
use crate::syntax_kind::{SyntaxKind, SyntaxKind::*};

impl Parser<'_, '_> {
    /// Recovery tokens for pattern parsing.
    const PATTERN_RECOVERY: &'static [SyntaxKind] = &[COMMA, R_PAREN, COLON, EQ];

    /// Parse a statement.
    pub fn parse_stmt(&mut self) -> Option<CompletedMarker> {
        self.skip_trivia();

        match self.current() {
            KW_LET => self.parse_let_stmt(),
            KW_CONST => self.parse_const_stmt(),
            KW_RETURN => self.parse_return_stmt(),
            KW_IF => self.parse_if_stmt(),
            KW_FOR => self.parse_for_stmt(),
            KW_ASSERT => self.parse_assert_stmt(),
            KW_ASSERT_EQ => self.parse_assert_eq_stmt(),
            KW_ASSERT_NEQ => self.parse_assert_neq_stmt(),
            L_BRACE => self.parse_block(),
            _ => self.parse_expr_or_assign_stmt(),
        }
    }

    /// Parse a let statement: `let x: Type = expr;` or `let (a, b) = expr;`
    fn parse_let_stmt(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any(); // let

        // Parse pattern (identifier or tuple destructuring)
        self.parse_pattern();

        // Optional type annotation
        if self.eat(COLON) && self.parse_type().is_none() {
            self.error_recover("expected type", TYPE_RECOVERY);
        }

        // Initializer
        self.expect(EQ);
        if self.parse_expr().is_none() {
            self.error_recover("expected expression", EXPR_RECOVERY);
        }

        self.expect(SEMICOLON);
        Some(m.complete(self, LET_STMT))
    }

    /// Parse a const statement: `const X: Type = expr;`
    fn parse_const_stmt(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any(); // const

        // Name
        self.skip_trivia();
        if self.at(IDENT) {
            self.bump_any();
        } else {
            self.error("expected identifier".to_string());
        }

        // Type annotation (required for const)
        self.expect(COLON);
        if self.parse_type().is_none() {
            self.error_recover("expected type", TYPE_RECOVERY);
        }

        // Initializer
        self.expect(EQ);
        if self.parse_expr().is_none() {
            self.error_recover("expected expression", EXPR_RECOVERY);
        }

        self.expect(SEMICOLON);
        Some(m.complete(self, CONST_STMT))
    }

    /// Parse a return statement: `return expr;` or `return;`
    fn parse_return_stmt(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any(); // return

        // Optional expression - only attempt if we're not at semicolon/EOF
        if !self.at(SEMICOLON) && !self.at_eof() && self.parse_expr().is_none() {
            // Tried to parse expression but failed - recover
            self.error_recover("expected expression or ';'", EXPR_RECOVERY);
        }

        self.expect(SEMICOLON);
        Some(m.complete(self, RETURN_STMT))
    }

    /// Parse an if statement: `if cond { } else if cond { } else { }`
    fn parse_if_stmt(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any(); // if

        // Parse condition (no struct literals to avoid ambiguity)
        if self.parse_expr_with_opts(ExprOpts::no_struct()).is_none() {
            self.error_recover("expected condition", EXPR_RECOVERY);
        }

        // Then block - recover if missing
        if self.parse_block().is_none() && !self.at_eof() {
            self.error_recover("expected block", STMT_RECOVERY);
        }

        // Optional else clause
        if self.eat(KW_ELSE) {
            if self.at(KW_IF) {
                // else if
                self.parse_if_stmt();
            } else {
                // else block - recover if missing
                if self.parse_block().is_none() && !self.at_eof() {
                    self.error_recover("expected block after 'else'", STMT_RECOVERY);
                }
            }
        }

        Some(m.complete(self, IF_STMT))
    }

    /// Parse a for statement: `for i: Type in lo..hi { }`
    fn parse_for_stmt(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any(); // for

        // Loop variable
        self.skip_trivia();
        if self.at(IDENT) {
            self.bump_any();
        } else {
            self.error("expected loop variable".to_string());
        }

        // Optional type annotation
        if self.eat(COLON) && self.parse_type().is_none() {
            self.error_recover("expected type", TYPE_RECOVERY);
        }

        // in keyword
        self.expect(KW_IN);

        // Range: lo..hi
        // Disallow struct literals so `IDENT {` parses as range + block body.
        if self.parse_expr_with_opts(ExprOpts::no_struct()).is_none() {
            self.error_recover("expected range start", EXPR_RECOVERY);
        }
        if self.eat(DOT_DOT) && self.parse_expr_with_opts(ExprOpts::no_struct()).is_none() {
            self.error_recover("expected range end", EXPR_RECOVERY);
        }

        // Body - recover if missing
        if self.parse_block().is_none() && !self.at_eof() {
            self.error_recover("expected block", STMT_RECOVERY);
        }

        Some(m.complete(self, FOR_STMT))
    }

    /// Parse an assert statement: `assert(cond);`
    fn parse_assert_stmt(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any(); // assert

        self.expect(L_PAREN);
        if self.parse_expr().is_none() {
            self.error_recover("expected expression", EXPR_RECOVERY);
        }
        self.expect(R_PAREN);

        self.expect(SEMICOLON);
        Some(m.complete(self, ASSERT_STMT))
    }

    /// Parse an assert_eq statement: `assert_eq(a, b);`
    fn parse_assert_eq_stmt(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any(); // assert_eq

        self.expect(L_PAREN);
        if self.parse_expr().is_none() {
            self.error_recover("expected first expression", EXPR_RECOVERY);
        }
        self.expect(COMMA);
        if self.parse_expr().is_none() {
            self.error_recover("expected second expression", EXPR_RECOVERY);
        }
        self.expect(R_PAREN);

        self.expect(SEMICOLON);
        Some(m.complete(self, ASSERT_EQ_STMT))
    }

    /// Parse an assert_neq statement: `assert_neq(a, b);`
    fn parse_assert_neq_stmt(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any(); // assert_neq

        self.expect(L_PAREN);
        if self.parse_expr().is_none() {
            self.error_recover("expected first expression", EXPR_RECOVERY);
        }
        self.expect(COMMA);
        if self.parse_expr().is_none() {
            self.error_recover("expected second expression", EXPR_RECOVERY);
        }
        self.expect(R_PAREN);

        self.expect(SEMICOLON);
        Some(m.complete(self, ASSERT_NEQ_STMT))
    }

    /// Parse a block: `{ stmts... }`
    pub fn parse_block(&mut self) -> Option<CompletedMarker> {
        let m = self.start();

        if !self.eat(L_BRACE) {
            self.error("expected {".to_string());
            m.abandon(self);
            return None;
        }

        // Parse statements until }
        while !self.at(R_BRACE) && !self.at_eof() {
            if self.parse_stmt().is_none() {
                // Error recovery: skip to next statement boundary
                self.error_recover("expected statement", STMT_RECOVERY);
            }
        }

        self.expect(R_BRACE);
        Some(m.complete(self, BLOCK))
    }

    /// Parse an expression statement or assignment.
    fn parse_expr_or_assign_stmt(&mut self) -> Option<CompletedMarker> {
        let m = self.start();

        let expr = self.parse_expr();
        if expr.is_none() {
            m.abandon(self);
            return None;
        }

        // Check for assignment operators
        if let Some(assign_kind) = self.current_assign_op() {
            self.bump_any(); // operator
            if self.parse_expr().is_none() {
                self.error_recover("expected expression after assignment operator", EXPR_RECOVERY);
            }
            self.expect(SEMICOLON);
            return Some(m.complete(self, assign_kind));
        }

        // Expression statement
        self.expect(SEMICOLON);
        Some(m.complete(self, EXPR_STMT))
    }

    /// Get the statement kind for the current assignment operator, if any.
    fn current_assign_op(&self) -> Option<SyntaxKind> {
        match self.current() {
            EQ => Some(ASSIGN_STMT),
            PLUS_EQ => Some(ASSIGN_STMT),
            MINUS_EQ => Some(ASSIGN_STMT),
            STAR_EQ => Some(ASSIGN_STMT),
            SLASH_EQ => Some(ASSIGN_STMT),
            PERCENT_EQ => Some(ASSIGN_STMT),
            STAR2_EQ => Some(ASSIGN_STMT),
            AMP_EQ => Some(ASSIGN_STMT),
            PIPE_EQ => Some(ASSIGN_STMT),
            CARET_EQ => Some(ASSIGN_STMT),
            SHL_EQ => Some(ASSIGN_STMT),
            SHR_EQ => Some(ASSIGN_STMT),
            AMP2_EQ => Some(ASSIGN_STMT),
            PIPE2_EQ => Some(ASSIGN_STMT),
            _ => None,
        }
    }

    /// Parse a pattern for let bindings.
    fn parse_pattern(&mut self) {
        self.skip_trivia();

        match self.current() {
            // Tuple pattern: (a, b, c)
            L_PAREN => {
                let m = self.start();
                self.bump_any(); // (

                if !self.at(R_PAREN) {
                    self.parse_pattern();
                    while self.eat(COMMA) {
                        if self.at(R_PAREN) {
                            break;
                        }
                        self.parse_pattern();
                    }
                }

                self.expect(R_PAREN);
                m.complete(self, TUPLE_PATTERN);
            }
            // Simple identifier pattern
            IDENT => {
                let m = self.start();
                self.bump_any();
                m.complete(self, IDENT_PATTERN);
            }
            // Wildcard pattern
            UNDERSCORE => {
                let m = self.start();
                self.bump_any();
                m.complete(self, WILDCARD_PATTERN);
            }
            _ => {
                self.error_recover("expected pattern", Self::PATTERN_RECOVERY);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexer::lex, parser::Parse};
    use expect_test::{Expect, expect};

    fn check_stmt(input: &str, expect: Expect) {
        let (tokens, _) = lex(input);
        let mut parser = Parser::new(input, &tokens);
        let root = parser.start();
        parser.parse_stmt();
        parser.skip_trivia();
        root.complete(&mut parser, ROOT);
        let parse: Parse = parser.finish();
        let output = format!("{:#?}", parse.syntax());
        expect.assert_eq(&output);
    }

    // =========================================================================
    // Let Statements
    // =========================================================================

    #[test]
    fn parse_stmt_let_simple() {
        check_stmt("let x = 1;", expect![[r#"
                ROOT@0..10
                  LET_STMT@0..10
                    KW_LET@0..3 "let"
                    WHITESPACE@3..4 " "
                    IDENT_PATTERN@4..5
                      IDENT@4..5 "x"
                    WHITESPACE@5..6 " "
                    EQ@6..7 "="
                    WHITESPACE@7..8 " "
                    LITERAL@8..9
                      INTEGER@8..9 "1"
                    SEMICOLON@9..10 ";"
            "#]]);
    }

    #[test]
    fn parse_stmt_let_typed() {
        check_stmt("let x: u32 = 42;", expect![[r#"
                ROOT@0..16
                  LET_STMT@0..16
                    KW_LET@0..3 "let"
                    WHITESPACE@3..4 " "
                    IDENT_PATTERN@4..5
                      IDENT@4..5 "x"
                    COLON@5..6 ":"
                    WHITESPACE@6..7 " "
                    TYPE_PATH@7..10
                      KW_U32@7..10 "u32"
                    WHITESPACE@10..11 " "
                    EQ@11..12 "="
                    WHITESPACE@12..13 " "
                    LITERAL@13..15
                      INTEGER@13..15 "42"
                    SEMICOLON@15..16 ";"
            "#]]);
    }

    #[test]
    fn parse_stmt_let_tuple_destructure() {
        check_stmt("let (a, b) = tuple;", expect![[r#"
                ROOT@0..19
                  LET_STMT@0..19
                    KW_LET@0..3 "let"
                    WHITESPACE@3..4 " "
                    TUPLE_PATTERN@4..10
                      L_PAREN@4..5 "("
                      IDENT_PATTERN@5..6
                        IDENT@5..6 "a"
                      COMMA@6..7 ","
                      WHITESPACE@7..8 " "
                      IDENT_PATTERN@8..9
                        IDENT@8..9 "b"
                      R_PAREN@9..10 ")"
                    WHITESPACE@10..11 " "
                    EQ@11..12 "="
                    WHITESPACE@12..13 " "
                    PATH_EXPR@13..18
                      IDENT@13..18 "tuple"
                    SEMICOLON@18..19 ";"
            "#]]);
    }

    // =========================================================================
    // Const Statements
    // =========================================================================

    #[test]
    fn parse_stmt_const() {
        check_stmt("const MAX: u32 = 100;", expect![[r#"
                ROOT@0..21
                  CONST_STMT@0..21
                    KW_CONST@0..5 "const"
                    WHITESPACE@5..6 " "
                    IDENT@6..9 "MAX"
                    COLON@9..10 ":"
                    WHITESPACE@10..11 " "
                    TYPE_PATH@11..14
                      KW_U32@11..14 "u32"
                    WHITESPACE@14..15 " "
                    EQ@15..16 "="
                    WHITESPACE@16..17 " "
                    LITERAL@17..20
                      INTEGER@17..20 "100"
                    SEMICOLON@20..21 ";"
            "#]]);
    }

    // =========================================================================
    // Return Statements
    // =========================================================================

    #[test]
    fn parse_stmt_return_value() {
        check_stmt("return 42;", expect![[r#"
                ROOT@0..10
                  RETURN_STMT@0..10
                    KW_RETURN@0..6 "return"
                    WHITESPACE@6..7 " "
                    LITERAL@7..9
                      INTEGER@7..9 "42"
                    SEMICOLON@9..10 ";"
            "#]]);
    }

    #[test]
    fn parse_stmt_return_empty() {
        check_stmt("return;", expect![[r#"
                ROOT@0..7
                  RETURN_STMT@0..7
                    KW_RETURN@0..6 "return"
                    SEMICOLON@6..7 ";"
            "#]]);
    }

    // =========================================================================
    // Assignment Statements
    // =========================================================================

    #[test]
    fn parse_stmt_assign() {
        check_stmt("x = 1;", expect![[r#"
                ROOT@0..6
                  ASSIGN_STMT@0..6
                    PATH_EXPR@0..2
                      IDENT@0..1 "x"
                      WHITESPACE@1..2 " "
                    EQ@2..3 "="
                    WHITESPACE@3..4 " "
                    LITERAL@4..5
                      INTEGER@4..5 "1"
                    SEMICOLON@5..6 ";"
            "#]]);
    }

    #[test]
    fn parse_stmt_assign_add() {
        check_stmt("x += 1;", expect![[r#"
                ROOT@0..7
                  ASSIGN_STMT@0..7
                    PATH_EXPR@0..2
                      IDENT@0..1 "x"
                      WHITESPACE@1..2 " "
                    PLUS_EQ@2..4 "+="
                    WHITESPACE@4..5 " "
                    LITERAL@5..6
                      INTEGER@5..6 "1"
                    SEMICOLON@6..7 ";"
            "#]]);
    }

    // =========================================================================
    // If Statements
    // =========================================================================

    #[test]
    fn parse_stmt_if_simple() {
        check_stmt("if cond { }", expect![[r#"
                ROOT@0..11
                  IF_STMT@0..11
                    KW_IF@0..2 "if"
                    WHITESPACE@2..3 " "
                    PATH_EXPR@3..8
                      IDENT@3..7 "cond"
                      WHITESPACE@7..8 " "
                    BLOCK@8..11
                      L_BRACE@8..9 "{"
                      WHITESPACE@9..10 " "
                      R_BRACE@10..11 "}"
            "#]]);
    }

    #[test]
    fn parse_stmt_if_else() {
        check_stmt("if a { } else { }", expect![[r#"
                ROOT@0..17
                  IF_STMT@0..17
                    KW_IF@0..2 "if"
                    WHITESPACE@2..3 " "
                    PATH_EXPR@3..5
                      IDENT@3..4 "a"
                      WHITESPACE@4..5 " "
                    BLOCK@5..8
                      L_BRACE@5..6 "{"
                      WHITESPACE@6..7 " "
                      R_BRACE@7..8 "}"
                    WHITESPACE@8..9 " "
                    KW_ELSE@9..13 "else"
                    BLOCK@13..17
                      WHITESPACE@13..14 " "
                      L_BRACE@14..15 "{"
                      WHITESPACE@15..16 " "
                      R_BRACE@16..17 "}"
            "#]]);
    }

    // =========================================================================
    // For Statements
    // =========================================================================

    #[test]
    fn parse_stmt_for() {
        check_stmt("for i in 0..10 { }", expect![[r#"
                ROOT@0..18
                  FOR_STMT@0..18
                    KW_FOR@0..3 "for"
                    WHITESPACE@3..4 " "
                    IDENT@4..5 "i"
                    WHITESPACE@5..6 " "
                    KW_IN@6..8 "in"
                    WHITESPACE@8..9 " "
                    LITERAL@9..10
                      INTEGER@9..10 "0"
                    DOT_DOT@10..12 ".."
                    LITERAL@12..14
                      INTEGER@12..14 "10"
                    BLOCK@14..18
                      WHITESPACE@14..15 " "
                      L_BRACE@15..16 "{"
                      WHITESPACE@16..17 " "
                      R_BRACE@17..18 "}"
            "#]]);
    }

    // =========================================================================
    // Assert Statements
    // =========================================================================

    #[test]
    fn parse_stmt_assert() {
        check_stmt("assert(x);", expect![[r#"
                ROOT@0..10
                  ASSERT_STMT@0..10
                    KW_ASSERT@0..6 "assert"
                    L_PAREN@6..7 "("
                    PATH_EXPR@7..8
                      IDENT@7..8 "x"
                    R_PAREN@8..9 ")"
                    SEMICOLON@9..10 ";"
            "#]]);
    }

    #[test]
    fn parse_stmt_assert_eq() {
        check_stmt("assert_eq(a, b);", expect![[r#"
                ROOT@0..16
                  ASSERT_EQ_STMT@0..16
                    KW_ASSERT_EQ@0..9 "assert_eq"
                    L_PAREN@9..10 "("
                    PATH_EXPR@10..11
                      IDENT@10..11 "a"
                    COMMA@11..12 ","
                    WHITESPACE@12..13 " "
                    PATH_EXPR@13..14
                      IDENT@13..14 "b"
                    R_PAREN@14..15 ")"
                    SEMICOLON@15..16 ";"
            "#]]);
    }

    // =========================================================================
    // Block and Expression Statements
    // =========================================================================

    #[test]
    fn parse_stmt_block() {
        check_stmt("{ let x = 1; }", expect![[r#"
                ROOT@0..14
                  BLOCK@0..14
                    L_BRACE@0..1 "{"
                    WHITESPACE@1..2 " "
                    LET_STMT@2..12
                      KW_LET@2..5 "let"
                      WHITESPACE@5..6 " "
                      IDENT_PATTERN@6..7
                        IDENT@6..7 "x"
                      WHITESPACE@7..8 " "
                      EQ@8..9 "="
                      WHITESPACE@9..10 " "
                      LITERAL@10..11
                        INTEGER@10..11 "1"
                      SEMICOLON@11..12 ";"
                    WHITESPACE@12..13 " "
                    R_BRACE@13..14 "}"
            "#]]);
    }

    #[test]
    fn parse_stmt_expr() {
        check_stmt("foo();", expect![[r#"
                ROOT@0..6
                  EXPR_STMT@0..6
                    CALL_EXPR@0..5
                      PATH_EXPR@0..3
                        IDENT@0..3 "foo"
                      L_PAREN@3..4 "("
                      R_PAREN@4..5 ")"
                    SEMICOLON@5..6 ";"
            "#]]);
    }
}
