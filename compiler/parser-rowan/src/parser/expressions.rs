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

//! Expression parsing for the Leo language.
//!
//! This module implements a Pratt parser (precedence climbing) for Leo expressions.
//! It handles operator precedence, associativity, and all expression forms.

use super::{CompletedMarker, EXPR_RECOVERY, Parser};
use crate::syntax_kind::{SyntaxKind, SyntaxKind::*};

// =============================================================================
// Operator Precedence
// =============================================================================

/// Binding power for operators (higher = tighter binding).
/// Returns (left_bp, right_bp) for the operator.
/// Left-associative: left_bp < right_bp
/// Right-associative: left_bp > right_bp
/// Non-associative: left_bp == right_bp (with special handling)
fn infix_binding_power(op: SyntaxKind) -> Option<(u8, u8)> {
    let bp = match op {
        // Ternary is handled specially at the lowest level
        // Level 14: ||
        PIPE2 => (28, 29),
        // Level 13: &&
        AMP2 => (26, 27),
        // Level 12: == != (non-associative, but we parse left-to-right)
        EQ2 | BANG_EQ => (24, 25),
        // Level 11: < <= > >= (non-associative)
        LT | LT_EQ | GT | GT_EQ => (22, 23),
        // Level 10: |
        PIPE => (20, 21),
        // Level 9: ^
        CARET => (18, 19),
        // Level 8: &
        AMP => (16, 17),
        // Level 7: << >>
        SHL | SHR => (14, 15),
        // Level 6: + -
        PLUS | MINUS => (12, 13),
        // Level 5: * / %
        STAR | SLASH | PERCENT => (10, 11),
        // Level 4: ** (right-associative)
        STAR2 => (9, 8),
        // Level 3: as (cast)
        KW_AS => (6, 7),
        _ => return None,
    };
    Some(bp)
}

/// Prefix binding power for unary operators.
fn prefix_binding_power(op: SyntaxKind) -> Option<u8> {
    match op {
        // Level 2: ! - (unary)
        BANG | MINUS => Some(30),
        _ => None,
    }
}

/// Postfix binding power for postfix operators.
fn postfix_binding_power(op: SyntaxKind) -> Option<u8> {
    match op {
        // Level 1: . [] () (postfix) - highest precedence
        DOT | L_BRACKET | L_PAREN => Some(32),
        _ => None,
    }
}

// =============================================================================
// Expression Options
// =============================================================================

/// Options for expression parsing to handle context-sensitive cases.
#[derive(Default, Clone, Copy)]
pub struct ExprOpts {
    /// Disallow struct literals `Foo { ... }` in this context.
    /// Used in conditional expressions to avoid ambiguity.
    pub no_struct: bool,
}

impl ExprOpts {
    /// Create options that disallow struct literals.
    pub fn no_struct() -> Self {
        Self { no_struct: true }
    }
}

// =============================================================================
// Expression Parsing
// =============================================================================

impl Parser<'_, '_> {
    /// Parse an expression.
    pub fn parse_expr(&mut self) -> Option<CompletedMarker> {
        self.parse_expr_with_opts(ExprOpts::default())
    }

    /// Parse an expression with options.
    pub fn parse_expr_with_opts(&mut self, opts: ExprOpts) -> Option<CompletedMarker> {
        self.parse_expr_bp(0, opts)
    }

    /// Parse an expression with minimum binding power.
    fn parse_expr_bp(&mut self, min_bp: u8, opts: ExprOpts) -> Option<CompletedMarker> {
        // Parse prefix expression or primary
        let mut lhs = self.parse_prefix_expr(opts)?;

        loop {
            // Try postfix operators (highest precedence)
            if let Some(bp) = self.current_postfix_bp() {
                if bp < min_bp {
                    break;
                }
                lhs = self.parse_postfix_expr(lhs, opts)?;
                continue;
            }

            // Handle ternary operator specially (lowest precedence)
            if self.at(QUESTION) && min_bp <= 2 {
                lhs = self.parse_ternary_expr(lhs)?;
                continue;
            }

            // Try infix operators
            let op = self.current();
            if let Some((l_bp, r_bp)) = infix_binding_power(op) {
                if l_bp < min_bp {
                    break;
                }
                lhs = self.parse_infix_expr(lhs, op, r_bp, opts)?;
                continue;
            }

            break;
        }

        Some(lhs)
    }

    /// Get the postfix binding power of the current token.
    fn current_postfix_bp(&self) -> Option<u8> {
        postfix_binding_power(self.current())
    }

    /// Parse a prefix expression (unary operators or primary).
    fn parse_prefix_expr(&mut self, opts: ExprOpts) -> Option<CompletedMarker> {
        self.skip_trivia();

        // Check for prefix operators
        if let Some(bp) = prefix_binding_power(self.current()) {
            let m = self.start();
            self.bump_any(); // operator

            // Parse operand with prefix binding power
            // If the operand fails, we still complete the unary expression
            if self.parse_expr_bp(bp, opts).is_none() {
                self.error("expected expression after unary operator".to_string());
            }

            return Some(m.complete(self, UNARY_EXPR));
        }

        // Parse primary expression
        self.parse_primary_expr(opts)
    }

    /// Parse a postfix expression (member access, indexing, calls).
    fn parse_postfix_expr(&mut self, lhs: CompletedMarker, opts: ExprOpts) -> Option<CompletedMarker> {
        match self.current() {
            DOT => self.parse_member_access(lhs),
            L_BRACKET => self.parse_index_expr(lhs),
            L_PAREN => self.parse_call_expr(lhs, opts),
            _ => Some(lhs),
        }
    }

    /// Parse an infix (binary) expression.
    fn parse_infix_expr(
        &mut self,
        lhs: CompletedMarker,
        op: SyntaxKind,
        r_bp: u8,
        opts: ExprOpts,
    ) -> Option<CompletedMarker> {
        let m = lhs.precede(self);
        self.bump_any(); // operator

        // Handle cast specially - parse type instead of expression
        if op == KW_AS {
            if self.parse_type().is_none() {
                self.error("expected type after 'as'".to_string());
            }
            return Some(m.complete(self, CAST_EXPR));
        }

        // Parse right-hand side
        // If RHS fails, we still complete the binary expression with an error
        if self.parse_expr_bp(r_bp, opts).is_none() {
            self.error("expected expression after operator".to_string());
        }

        Some(m.complete(self, BINARY_EXPR))
    }

    /// Parse a ternary expression: `condition ? then : else`.
    fn parse_ternary_expr(&mut self, condition: CompletedMarker) -> Option<CompletedMarker> {
        let m = condition.precede(self);
        self.bump_any(); // ?

        // Parse then branch
        if self.parse_expr().is_none() {
            self.error("expected expression after '?'".to_string());
        }

        self.expect(COLON);

        // Parse else branch (right-associative)
        if self.parse_expr_bp(2, ExprOpts::default()).is_none() {
            self.error("expected expression after ':'".to_string());
        }

        Some(m.complete(self, TERNARY_EXPR))
    }

    /// Parse member access: `expr.field` or `expr.0` (tuple index).
    fn parse_member_access(&mut self, lhs: CompletedMarker) -> Option<CompletedMarker> {
        let m = lhs.precede(self);
        self.bump_any(); // .

        self.skip_trivia();

        // Parse field name or tuple index.
        // Keywords are valid as field names (e.g. `.field`, `.owner`).
        if self.at(IDENT) || self.at(INTEGER) || self.current().is_keyword() {
            self.bump_any();
        } else {
            self.error("expected field name or tuple index".to_string());
        }

        Some(m.complete(self, FIELD_EXPR))
    }

    /// Parse index expression: `expr[index]`.
    fn parse_index_expr(&mut self, lhs: CompletedMarker) -> Option<CompletedMarker> {
        let m = lhs.precede(self);
        self.bump_any(); // [

        if self.parse_expr().is_none() {
            self.error("expected index expression".to_string());
        }

        self.expect(R_BRACKET);

        Some(m.complete(self, INDEX_EXPR))
    }

    /// Parse call expression: `expr(args)`.
    fn parse_call_expr(&mut self, lhs: CompletedMarker, _opts: ExprOpts) -> Option<CompletedMarker> {
        let m = lhs.precede(self);
        self.bump_any(); // (

        // Parse arguments
        if !self.at(R_PAREN) {
            if self.parse_expr().is_none() && !self.at(R_PAREN) && !self.at(COMMA) {
                // Skip invalid tokens until we find a recovery point
                self.error_recover("expected argument expression", EXPR_RECOVERY);
            }
            while self.eat(COMMA) {
                if self.at(R_PAREN) {
                    break;
                }
                if self.parse_expr().is_none() && !self.at(R_PAREN) && !self.at(COMMA) {
                    self.error_recover("expected argument expression", EXPR_RECOVERY);
                }
            }
        }

        self.expect(R_PAREN);

        Some(m.complete(self, CALL_EXPR))
    }

    // =========================================================================
    // Primary Expressions
    // =========================================================================

    /// Parse a primary expression (atoms and grouped expressions).
    fn parse_primary_expr(&mut self, opts: ExprOpts) -> Option<CompletedMarker> {
        self.skip_trivia();

        match self.current() {
            // Literals
            INTEGER => self.parse_integer_literal(),
            STRING => self.parse_string_literal(),
            ADDRESS_LIT => self.parse_address_literal(),
            KW_TRUE | KW_FALSE => self.parse_bool_literal(),
            KW_NONE => self.parse_none_literal(),

            // Parenthesized or tuple expression
            L_PAREN => self.parse_paren_or_tuple_expr(),

            // Array expression
            L_BRACKET => self.parse_array_expr(),

            // Identifier, path, or struct literal
            IDENT => self.parse_ident_expr(opts),

            // Self access
            KW_SELF => self.parse_self_expr(),

            // Block expressions (block, network)
            KW_BLOCK => self.parse_block_access(),
            KW_NETWORK => self.parse_network_access(),

            // Async block expression: `async { ... }`
            KW_ASYNC => self.parse_async_block_expr(),

            _ => {
                self.error(format!("expected expression, found {:?}", self.current()));
                None
            }
        }
    }

    /// Parse an integer literal.
    fn parse_integer_literal(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any();
        Some(m.complete(self, LITERAL))
    }

    /// Parse a string literal.
    fn parse_string_literal(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any();
        Some(m.complete(self, LITERAL))
    }

    /// Parse an address literal.
    fn parse_address_literal(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any();
        Some(m.complete(self, LITERAL))
    }

    /// Parse a boolean literal (true/false).
    fn parse_bool_literal(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any();
        Some(m.complete(self, LITERAL))
    }

    /// Parse the `none` literal.
    fn parse_none_literal(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any();
        Some(m.complete(self, LITERAL))
    }

    /// Parse a parenthesized expression or tuple.
    fn parse_paren_or_tuple_expr(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any(); // (

        // Empty tuple: ()
        if self.eat(R_PAREN) {
            return Some(m.complete(self, TUPLE_EXPR));
        }

        // Parse first expression
        if self.parse_expr().is_none() && !self.at(R_PAREN) && !self.at(COMMA) {
            self.error_recover("expected expression", EXPR_RECOVERY);
        }

        // Check if this is a tuple
        if self.eat(COMMA) {
            // It's a tuple - parse remaining elements
            if !self.at(R_PAREN) {
                if self.parse_expr().is_none() && !self.at(R_PAREN) && !self.at(COMMA) {
                    self.error_recover("expected tuple element", EXPR_RECOVERY);
                }
                while self.eat(COMMA) {
                    if self.at(R_PAREN) {
                        break;
                    }
                    if self.parse_expr().is_none() && !self.at(R_PAREN) && !self.at(COMMA) {
                        self.error_recover("expected tuple element", EXPR_RECOVERY);
                    }
                }
            }
            self.expect(R_PAREN);
            return Some(m.complete(self, TUPLE_EXPR));
        }

        // Single expression - parenthesized
        self.expect(R_PAREN);
        Some(m.complete(self, PAREN_EXPR))
    }

    /// Parse an array expression: `[a, b, c]` or `[x; n]`.
    fn parse_array_expr(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any(); // [

        // Empty array
        if self.eat(R_BRACKET) {
            return Some(m.complete(self, ARRAY_EXPR));
        }

        // Parse first element
        if self.parse_expr().is_none() && !self.at(R_BRACKET) && !self.at(COMMA) && !self.at(SEMICOLON) {
            self.error_recover("expected array element", EXPR_RECOVERY);
        }

        // Check for repeat syntax: [x; n]
        if self.eat(SEMICOLON) {
            if self.parse_expr().is_none() && !self.at(R_BRACKET) {
                self.error("expected repeat count".to_string());
            }
            self.expect(R_BRACKET);
            return Some(m.complete(self, ARRAY_EXPR));
        }

        // List syntax: [a, b, c]
        while self.eat(COMMA) {
            if self.at(R_BRACKET) {
                break;
            }
            if self.parse_expr().is_none() && !self.at(R_BRACKET) && !self.at(COMMA) {
                self.error_recover("expected array element", EXPR_RECOVERY);
            }
        }

        self.expect(R_BRACKET);
        Some(m.complete(self, ARRAY_EXPR))
    }

    /// Parse an identifier expression, path, or struct literal.
    fn parse_ident_expr(&mut self, opts: ExprOpts) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any(); // first identifier

        // Check for locator: name.aleo/path
        if self.at(DOT) && self.nth(1) == KW_ALEO {
            self.bump_any(); // .
            self.bump_any(); // aleo

            if self.eat(SLASH) {
                // Locator path
                if self.at(IDENT) {
                    self.bump_any();
                }
            }

            // Optional const generic args after locator: child.aleo/foo::[3]
            if self.at(COLON_COLON) && self.nth(1) == L_BRACKET {
                self.bump_any(); // ::
                self.parse_const_generic_args_bracket();
            }

            // Check for struct literal: `child.aleo/Foo::[N] { ... }`
            if !opts.no_struct && self.at(L_BRACE) {
                self.bump_any(); // {
                if !self.at(R_BRACE) {
                    self.parse_struct_field();
                    while self.eat(COMMA) {
                        if self.at(R_BRACE) {
                            break;
                        }
                        self.parse_struct_field();
                    }
                }
                self.expect(R_BRACE);
                return Some(m.complete(self, STRUCT_EXPR));
            }

            // Check for call
            if self.at(L_PAREN) {
                let cm = m.complete(self, PATH_EXPR);
                return self.parse_call_expr(cm, opts);
            }

            return Some(m.complete(self, PATH_EXPR));
        }

        // Check for path or const generics: Foo::Bar or Foo::[N]
        while self.eat(COLON_COLON) {
            if self.at(L_BRACKET) {
                // Const generics with brackets: Foo::[N]
                self.parse_const_generic_args_bracket();
                break;
            } else if self.at(LT) {
                // This could be const generics or just less-than
                // Try to parse as const generics
                self.parse_const_generic_args_angle();
                break;
            } else if self.at(IDENT) {
                self.bump_any();
            } else {
                self.error("expected identifier after ::".to_string());
                break;
            }
        }

        // Check for struct literal: `Foo { field: value }`
        if !opts.no_struct && self.at(L_BRACE) {
            self.bump_any(); // {

            // Parse fields
            if !self.at(R_BRACE) {
                self.parse_struct_field();
                while self.eat(COMMA) {
                    if self.at(R_BRACE) {
                        break;
                    }
                    self.parse_struct_field();
                }
            }

            self.expect(R_BRACE);
            return Some(m.complete(self, STRUCT_EXPR));
        }

        // Check for function call
        if self.at(L_PAREN) {
            let cm = m.complete(self, PATH_EXPR);
            return self.parse_call_expr(cm, opts);
        }

        Some(m.complete(self, PATH_EXPR))
    }

    /// Parse a struct field: `name: value` or `name` (shorthand).
    fn parse_struct_field(&mut self) {
        let m = self.start();
        self.skip_trivia();

        if self.at(IDENT) {
            self.bump_any(); // field name

            if self.eat(COLON) {
                // Field with value
                if self.parse_expr().is_none() && !self.at(R_BRACE) && !self.at(COMMA) {
                    self.error("expected field value".to_string());
                }
            }
            // Otherwise it's shorthand: `{ x }` means `{ x: x }`
        } else {
            self.error("expected field name".to_string());
        }

        m.complete(self, STRUCT_FIELD_INIT);
    }

    /// Parse `self` expression.
    fn parse_self_expr(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any(); // self
        Some(m.complete(self, PATH_EXPR))
    }

    /// Parse `block.height` access.
    fn parse_block_access(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any(); // block
        Some(m.complete(self, PATH_EXPR))
    }

    /// Parse `network.id` access.
    fn parse_network_access(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any(); // network
        Some(m.complete(self, PATH_EXPR))
    }

    /// Parse an async block expression: `async { stmts }`.
    fn parse_async_block_expr(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any(); // async
        self.skip_trivia();
        if self.parse_block().is_none() {
            self.error("expected block after 'async'".to_string());
        }
        Some(m.complete(self, ASYNC_EXPR))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexer::lex, parser::Parse};
    use expect_test::{Expect, expect};

    fn check_expr(input: &str, expect: Expect) {
        let (tokens, _) = lex(input);
        let mut parser = Parser::new(input, &tokens);
        let root = parser.start();
        parser.parse_expr();
        parser.skip_trivia();
        root.complete(&mut parser, ROOT);
        let parse: Parse = parser.finish();
        let output = format!("{:#?}", parse.syntax());
        expect.assert_eq(&output);
    }

    // =========================================================================
    // Literals
    // =========================================================================

    #[test]
    fn parse_expr_integer() {
        check_expr("42", expect![[r#"
                ROOT@0..2
                  LITERAL@0..2
                    INTEGER@0..2 "42"
            "#]]);
    }

    #[test]
    fn parse_expr_bool_true() {
        check_expr("true", expect![[r#"
                ROOT@0..4
                  LITERAL@0..4
                    KW_TRUE@0..4 "true"
            "#]]);
    }

    #[test]
    fn parse_expr_bool_false() {
        check_expr("false", expect![[r#"
                ROOT@0..5
                  LITERAL@0..5
                    KW_FALSE@0..5 "false"
            "#]]);
    }

    #[test]
    fn parse_expr_none() {
        check_expr("none", expect![[r#"
                ROOT@0..4
                  LITERAL@0..4
                    KW_NONE@0..4 "none"
            "#]]);
    }

    // =========================================================================
    // Identifiers and Paths
    // =========================================================================

    #[test]
    fn parse_expr_ident() {
        check_expr("foo", expect![[r#"
                ROOT@0..3
                  PATH_EXPR@0..3
                    IDENT@0..3 "foo"
            "#]]);
    }

    #[test]
    fn parse_expr_path() {
        check_expr("Foo::bar", expect![[r#"
                ROOT@0..8
                  PATH_EXPR@0..8
                    IDENT@0..3 "Foo"
                    COLON_COLON@3..5 "::"
                    IDENT@5..8 "bar"
            "#]]);
    }

    #[test]
    fn parse_expr_self() {
        check_expr("self", expect![[r#"
                ROOT@0..4
                  PATH_EXPR@0..4
                    KW_SELF@0..4 "self"
            "#]]);
    }

    // =========================================================================
    // Arithmetic
    // =========================================================================

    #[test]
    fn parse_expr_add() {
        check_expr("1 + 2", expect![[r#"
                ROOT@0..5
                  BINARY_EXPR@0..5
                    LITERAL@0..1
                      INTEGER@0..1 "1"
                    WHITESPACE@1..2 " "
                    PLUS@2..3 "+"
                    WHITESPACE@3..4 " "
                    LITERAL@4..5
                      INTEGER@4..5 "2"
            "#]]);
    }

    #[test]
    fn parse_expr_mul() {
        check_expr("a * b", expect![[r#"
                ROOT@0..5
                  BINARY_EXPR@0..5
                    PATH_EXPR@0..2
                      IDENT@0..1 "a"
                      WHITESPACE@1..2 " "
                    STAR@2..3 "*"
                    WHITESPACE@3..4 " "
                    PATH_EXPR@4..5
                      IDENT@4..5 "b"
            "#]]);
    }

    #[test]
    fn parse_expr_precedence() {
        // 1 + 2 * 3 should parse as 1 + (2 * 3)
        check_expr("1 + 2 * 3", expect![[r#"
                ROOT@0..9
                  BINARY_EXPR@0..9
                    BINARY_EXPR@0..5
                      LITERAL@0..1
                        INTEGER@0..1 "1"
                      WHITESPACE@1..2 " "
                      PLUS@2..3 "+"
                      WHITESPACE@3..4 " "
                      LITERAL@4..5
                        INTEGER@4..5 "2"
                    WHITESPACE@5..6 " "
                    STAR@6..7 "*"
                    WHITESPACE@7..8 " "
                    LITERAL@8..9
                      INTEGER@8..9 "3"
            "#]]);
    }

    #[test]
    fn parse_expr_power_right_assoc() {
        // a ** b ** c should parse as a ** (b ** c)
        check_expr("a ** b ** c", expect![[r#"
                ROOT@0..11
                  BINARY_EXPR@0..11
                    PATH_EXPR@0..2
                      IDENT@0..1 "a"
                      WHITESPACE@1..2 " "
                    STAR2@2..4 "**"
                    WHITESPACE@4..5 " "
                    BINARY_EXPR@5..11
                      PATH_EXPR@5..7
                        IDENT@5..6 "b"
                        WHITESPACE@6..7 " "
                      STAR2@7..9 "**"
                      WHITESPACE@9..10 " "
                      PATH_EXPR@10..11
                        IDENT@10..11 "c"
            "#]]);
    }

    // =========================================================================
    // Unary Operators
    // =========================================================================

    #[test]
    fn parse_expr_unary_neg() {
        check_expr("-x", expect![[r#"
                ROOT@0..2
                  UNARY_EXPR@0..2
                    MINUS@0..1 "-"
                    PATH_EXPR@1..2
                      IDENT@1..2 "x"
            "#]]);
    }

    #[test]
    fn parse_expr_unary_not() {
        check_expr("!flag", expect![[r#"
                ROOT@0..5
                  UNARY_EXPR@0..5
                    BANG@0..1 "!"
                    PATH_EXPR@1..5
                      IDENT@1..5 "flag"
            "#]]);
    }

    // =========================================================================
    // Comparison and Logical
    // =========================================================================

    #[test]
    fn parse_expr_comparison() {
        check_expr("a < b", expect![[r#"
                ROOT@0..5
                  BINARY_EXPR@0..5
                    PATH_EXPR@0..2
                      IDENT@0..1 "a"
                      WHITESPACE@1..2 " "
                    LT@2..3 "<"
                    WHITESPACE@3..4 " "
                    PATH_EXPR@4..5
                      IDENT@4..5 "b"
            "#]]);
    }

    #[test]
    fn parse_expr_logical_and() {
        check_expr("a && b", expect![[r#"
                ROOT@0..6
                  BINARY_EXPR@0..6
                    PATH_EXPR@0..2
                      IDENT@0..1 "a"
                      WHITESPACE@1..2 " "
                    AMP2@2..4 "&&"
                    WHITESPACE@4..5 " "
                    PATH_EXPR@5..6
                      IDENT@5..6 "b"
            "#]]);
    }

    #[test]
    fn parse_expr_logical_or() {
        check_expr("a || b", expect![[r#"
                ROOT@0..6
                  BINARY_EXPR@0..6
                    PATH_EXPR@0..2
                      IDENT@0..1 "a"
                      WHITESPACE@1..2 " "
                    PIPE2@2..4 "||"
                    WHITESPACE@4..5 " "
                    PATH_EXPR@5..6
                      IDENT@5..6 "b"
            "#]]);
    }

    // =========================================================================
    // Ternary
    // =========================================================================

    #[test]
    fn parse_expr_ternary() {
        check_expr("a ? b : c", expect![[r#"
                ROOT@0..9
                  TERNARY_EXPR@0..9
                    PATH_EXPR@0..2
                      IDENT@0..1 "a"
                      WHITESPACE@1..2 " "
                    QUESTION@2..3 "?"
                    WHITESPACE@3..4 " "
                    PATH_EXPR@4..6
                      IDENT@4..5 "b"
                      WHITESPACE@5..6 " "
                    COLON@6..7 ":"
                    WHITESPACE@7..8 " "
                    PATH_EXPR@8..9
                      IDENT@8..9 "c"
            "#]]);
    }

    // =========================================================================
    // Postfix: Member Access, Indexing, Calls
    // =========================================================================

    #[test]
    fn parse_expr_member_access() {
        check_expr("foo.bar", expect![[r#"
                ROOT@0..7
                  FIELD_EXPR@0..7
                    PATH_EXPR@0..3
                      IDENT@0..3 "foo"
                    DOT@3..4 "."
                    IDENT@4..7 "bar"
            "#]]);
    }

    #[test]
    fn parse_expr_tuple_access() {
        check_expr("tuple.0", expect![[r#"
                ROOT@0..7
                  FIELD_EXPR@0..7
                    PATH_EXPR@0..5
                      IDENT@0..5 "tuple"
                    DOT@5..6 "."
                    INTEGER@6..7 "0"
            "#]]);
    }

    #[test]
    fn parse_expr_index() {
        check_expr("arr[0]", expect![[r#"
                ROOT@0..6
                  INDEX_EXPR@0..6
                    PATH_EXPR@0..3
                      IDENT@0..3 "arr"
                    L_BRACKET@3..4 "["
                    LITERAL@4..5
                      INTEGER@4..5 "0"
                    R_BRACKET@5..6 "]"
            "#]]);
    }

    #[test]
    fn parse_expr_call() {
        check_expr("foo(a, b)", expect![[r#"
                ROOT@0..9
                  CALL_EXPR@0..9
                    PATH_EXPR@0..3
                      IDENT@0..3 "foo"
                    L_PAREN@3..4 "("
                    PATH_EXPR@4..5
                      IDENT@4..5 "a"
                    COMMA@5..6 ","
                    WHITESPACE@6..7 " "
                    PATH_EXPR@7..8
                      IDENT@7..8 "b"
                    R_PAREN@8..9 ")"
            "#]]);
    }

    #[test]
    fn parse_expr_method_call() {
        check_expr("x.foo()", expect![[r#"
                ROOT@0..7
                  CALL_EXPR@0..7
                    FIELD_EXPR@0..5
                      PATH_EXPR@0..1
                        IDENT@0..1 "x"
                      DOT@1..2 "."
                      IDENT@2..5 "foo"
                    L_PAREN@5..6 "("
                    R_PAREN@6..7 ")"
            "#]]);
    }

    // =========================================================================
    // Cast
    // =========================================================================

    #[test]
    fn parse_expr_cast() {
        check_expr("x as u64", expect![[r#"
                ROOT@0..8
                  CAST_EXPR@0..8
                    PATH_EXPR@0..2
                      IDENT@0..1 "x"
                      WHITESPACE@1..2 " "
                    KW_AS@2..4 "as"
                    WHITESPACE@4..5 " "
                    TYPE_PATH@5..8
                      KW_U64@5..8 "u64"
            "#]]);
    }

    // =========================================================================
    // Parentheses and Tuples
    // =========================================================================

    #[test]
    fn parse_expr_paren() {
        check_expr("(a + b)", expect![[r#"
                ROOT@0..7
                  PAREN_EXPR@0..7
                    L_PAREN@0..1 "("
                    BINARY_EXPR@1..6
                      PATH_EXPR@1..3
                        IDENT@1..2 "a"
                        WHITESPACE@2..3 " "
                      PLUS@3..4 "+"
                      WHITESPACE@4..5 " "
                      PATH_EXPR@5..6
                        IDENT@5..6 "b"
                    R_PAREN@6..7 ")"
            "#]]);
    }

    #[test]
    fn parse_expr_tuple() {
        check_expr("(a, b)", expect![[r#"
                ROOT@0..6
                  TUPLE_EXPR@0..6
                    L_PAREN@0..1 "("
                    PATH_EXPR@1..2
                      IDENT@1..2 "a"
                    COMMA@2..3 ","
                    WHITESPACE@3..4 " "
                    PATH_EXPR@4..5
                      IDENT@4..5 "b"
                    R_PAREN@5..6 ")"
            "#]]);
    }

    #[test]
    fn parse_expr_unit() {
        check_expr("()", expect![[r#"
                ROOT@0..2
                  TUPLE_EXPR@0..2
                    L_PAREN@0..1 "("
                    R_PAREN@1..2 ")"
            "#]]);
    }

    // =========================================================================
    // Arrays
    // =========================================================================

    #[test]
    fn parse_expr_array() {
        check_expr("[1, 2, 3]", expect![[r#"
                ROOT@0..9
                  ARRAY_EXPR@0..9
                    L_BRACKET@0..1 "["
                    LITERAL@1..2
                      INTEGER@1..2 "1"
                    COMMA@2..3 ","
                    WHITESPACE@3..4 " "
                    LITERAL@4..5
                      INTEGER@4..5 "2"
                    COMMA@5..6 ","
                    WHITESPACE@6..7 " "
                    LITERAL@7..8
                      INTEGER@7..8 "3"
                    R_BRACKET@8..9 "]"
            "#]]);
    }

    #[test]
    fn parse_expr_array_repeat() {
        check_expr("[0; 10]", expect![[r#"
                ROOT@0..7
                  ARRAY_EXPR@0..7
                    L_BRACKET@0..1 "["
                    LITERAL@1..2
                      INTEGER@1..2 "0"
                    SEMICOLON@2..3 ";"
                    WHITESPACE@3..4 " "
                    LITERAL@4..6
                      INTEGER@4..6 "10"
                    R_BRACKET@6..7 "]"
            "#]]);
    }

    // =========================================================================
    // Struct Literals
    // =========================================================================

    #[test]
    fn parse_expr_struct_init() {
        check_expr("Point { x: 1, y: 2 }", expect![[r#"
                ROOT@0..20
                  STRUCT_EXPR@0..20
                    IDENT@0..5 "Point"
                    WHITESPACE@5..6 " "
                    L_BRACE@6..7 "{"
                    STRUCT_FIELD_INIT@7..12
                      WHITESPACE@7..8 " "
                      IDENT@8..9 "x"
                      COLON@9..10 ":"
                      WHITESPACE@10..11 " "
                      LITERAL@11..12
                        INTEGER@11..12 "1"
                    COMMA@12..13 ","
                    STRUCT_FIELD_INIT@13..18
                      WHITESPACE@13..14 " "
                      IDENT@14..15 "y"
                      COLON@15..16 ":"
                      WHITESPACE@16..17 " "
                      LITERAL@17..18
                        INTEGER@17..18 "2"
                    WHITESPACE@18..19 " "
                    R_BRACE@19..20 "}"
            "#]]);
    }

    #[test]
    fn parse_expr_struct_shorthand() {
        check_expr("Point { x, y }", expect![[r#"
                ROOT@0..14
                  STRUCT_EXPR@0..14
                    IDENT@0..5 "Point"
                    WHITESPACE@5..6 " "
                    L_BRACE@6..7 "{"
                    STRUCT_FIELD_INIT@7..9
                      WHITESPACE@7..8 " "
                      IDENT@8..9 "x"
                    COMMA@9..10 ","
                    STRUCT_FIELD_INIT@10..13
                      WHITESPACE@10..11 " "
                      IDENT@11..12 "y"
                      WHITESPACE@12..13 " "
                    R_BRACE@13..14 "}"
            "#]]);
    }

    // =========================================================================
    // Complex Expressions
    // =========================================================================

    // =========================================================================
    // Const Generic Arguments (Use Sites) in Expressions
    // =========================================================================

    fn check_expr_no_errors(input: &str) {
        let (tokens, _) = lex(input);
        let mut parser = Parser::new(input, &tokens);
        let root = parser.start();
        parser.parse_expr();
        parser.skip_trivia();
        root.complete(&mut parser, ROOT);
        let parse: Parse = parser.finish();
        if !parse.errors().is_empty() {
            for err in parse.errors() {
                eprintln!("error at {:?}: {}", err.range, err.message);
            }
            eprintln!("tree:\n{:#?}", parse.syntax());
            panic!("expression parse had {} error(s)", parse.errors().len());
        }
    }

    #[test]
    fn parse_expr_call_const_generic_simple() {
        // Function call with const generic integer arg
        check_expr_no_errors("foo::[5]()");
    }

    #[test]
    fn parse_expr_call_const_generic_expr() {
        // Function call with expression const generic arg
        check_expr_no_errors("foo::[N + 1]()");
    }

    #[test]
    fn parse_expr_call_const_generic_multi() {
        // Multi-arg const generic call
        check_expr_no_errors("bar::[M, K, N]()");
    }

    #[test]
    fn parse_expr_struct_lit_const_generic() {
        // Struct literal with const generic arg
        check_expr_no_errors("Foo::[8u32] { arr: x }");
    }

    #[test]
    fn parse_expr_locator_call_const_generic() {
        // Locator + const generic call
        check_expr_no_errors("child.aleo/foo::[3]()");
    }

    #[test]
    fn parse_expr_assoc_fn_const_generic() {
        // Associated function with const generic: Path::method::[N]()
        check_expr_no_errors("Foo::bar::[N]()");
    }

    // =========================================================================
    // Complex Expressions
    // =========================================================================

    #[test]
    fn parse_expr_complex() {
        check_expr("a.b[c](d) + e", expect![[r#"
                ROOT@0..13
                  BINARY_EXPR@0..13
                    CALL_EXPR@0..9
                      INDEX_EXPR@0..6
                        FIELD_EXPR@0..3
                          PATH_EXPR@0..1
                            IDENT@0..1 "a"
                          DOT@1..2 "."
                          IDENT@2..3 "b"
                        L_BRACKET@3..4 "["
                        PATH_EXPR@4..5
                          IDENT@4..5 "c"
                        R_BRACKET@5..6 "]"
                      L_PAREN@6..7 "("
                      PATH_EXPR@7..8
                        IDENT@7..8 "d"
                      R_PAREN@8..9 ")"
                    WHITESPACE@9..10 " "
                    PLUS@10..11 "+"
                    WHITESPACE@11..12 " "
                    PATH_EXPR@12..13
                      IDENT@12..13 "e"
            "#]]);
    }
}
