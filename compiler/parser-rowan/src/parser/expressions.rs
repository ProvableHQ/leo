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
///
/// LALRPOP levels go from Expr0 (atoms, tightest) to Expr15 (entry, loosest).
/// Pratt BP: higher = tighter. So we use (16 - Level) * 2 as base BP.
fn infix_binding_power(op: SyntaxKind) -> Option<(u8, u8)> {
    let bp = match op {
        // Ternary is handled specially at the lowest level (Level 15 -> BP 2)
        // Level 14: || (lowest precedence among binary ops)
        PIPE2 => (4, 5),
        // Level 13: &&
        AMP2 => (6, 7),
        // Level 12: == != (non-associative - equal binding powers)
        EQ2 | BANG_EQ => (8, 8),
        // Level 11: < <= > >= (non-associative - equal binding powers)
        LT | LT_EQ | GT | GT_EQ => (10, 10),
        // Level 10: |
        PIPE => (12, 13),
        // Level 9: ^
        CARET => (14, 15),
        // Level 8: &
        AMP => (16, 17),
        // Level 7: << >>
        SHL | SHR => (18, 19),
        // Level 6: + -
        PLUS | MINUS => (20, 21),
        // Level 5: * / %
        STAR | SLASH | PERCENT => (22, 23),
        // Level 4: ** (right-associative: left_bp > right_bp)
        STAR2 => (25, 24),
        // Level 3: as (cast)
        KW_AS => (26, 27),
        _ => return None,
    };
    Some(bp)
}

/// Check if an operator is a comparison operator (non-associative).
fn is_comparison_op(op: SyntaxKind) -> bool {
    matches!(op, EQ2 | BANG_EQ | LT | LT_EQ | GT | GT_EQ)
}

/// Returns the operators valid after a comparison (next precedence level down).
/// These are the lower-precedence operators that can follow a comparison.
fn expected_after_comparison(bp: u8) -> &'static [&'static str] {
    match bp {
        8 => &["'&&'", "'||'", "'?'"],                  // After == != (BP 8)
        10 => &["'&&'", "'||'", "'=='", "'!='", "'?'"], // After < > <= >= (BP 10)
        _ => &["an operator"],
    }
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
    /// Tokens that may follow a complete expression (binary/postfix operators).
    pub const EXPR_CONTINUATION: &'static [SyntaxKind] = &[
        AMP2,
        PIPE2,
        AMP,
        PIPE,
        CARET,
        EQ2,
        BANG_EQ,
        LT,
        LT_EQ,
        GT,
        GT_EQ,
        PLUS,
        MINUS,
        STAR,
        SLASH,
        STAR2,
        PERCENT,
        SHL,
        SHR,
        L_PAREN,
        L_BRACKET,
        L_BRACE,
        DOT,
        COLON_COLON,
        QUESTION,
        KW_AS,
    ];

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
                lhs = self.parse_postfix_expr(lhs)?;
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

                // Check for non-associative operator chaining (e.g., 1 == 2 == 3)
                // With equal binding powers, l_bp == r_bp for comparison operators.
                // If min_bp equals l_bp and this is a comparison, it means we're
                // trying to chain comparisons, which is not allowed.
                if l_bp == r_bp && l_bp == min_bp && is_comparison_op(op) {
                    let expected_tokens = expected_after_comparison(l_bp);
                    self.error_unexpected(op, expected_tokens);
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
                self.error("expected expression after unary operator");
            }

            return Some(m.complete(self, UNARY_EXPR));
        }

        // Parse primary expression
        self.parse_primary_expr(opts)
    }

    /// Parse a postfix expression (member access, indexing, calls).
    fn parse_postfix_expr(&mut self, lhs: CompletedMarker) -> Option<CompletedMarker> {
        match self.current() {
            DOT => self.parse_member_access(lhs),
            L_BRACKET => self.parse_index_expr(lhs),
            L_PAREN => self.parse_call_expr(lhs),
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

        // Handle cast specially - only primitive types are allowed after 'as'.
        if op == KW_AS {
            if self.parse_cast_type().is_none() {
                let expected: Vec<&str> = Self::PRIMITIVE_TYPE_KINDS.iter().map(|k| k.user_friendly_name()).collect();
                self.error_unexpected(self.current(), &expected);
            }
            return Some(m.complete(self, CAST_EXPR));
        }

        // Parse right-hand side
        // If RHS fails, we still complete the binary expression with an error
        if self.parse_expr_bp(r_bp, opts).is_none() {
            self.error("expected expression after operator");
        }

        Some(m.complete(self, BINARY_EXPR))
    }

    /// Parse a ternary expression: `condition ? then : else`.
    fn parse_ternary_expr(&mut self, condition: CompletedMarker) -> Option<CompletedMarker> {
        let m = condition.precede(self);
        self.bump_any(); // ?

        // Parse then branch
        if self.parse_expr().is_none() {
            self.error("expected expression after '?'");
        }

        self.expect(COLON);

        // Parse else branch (right-associative)
        if self.parse_expr_bp(2, ExprOpts::default()).is_none() {
            self.error("expected expression after ':'");
        }

        Some(m.complete(self, TERNARY_EXPR))
    }

    /// Parse member access: `expr.field`, `expr.0` (tuple index), or `expr.method(args)`.
    fn parse_member_access(&mut self, lhs: CompletedMarker) -> Option<CompletedMarker> {
        let m = lhs.precede(self);
        self.bump_any(); // .

        self.skip_trivia();

        // Parse field name or tuple index.
        // Keywords are valid as field names (e.g. `.field`, `.owner`).
        if self.at(INTEGER) {
            self.bump_any();
            return Some(m.complete(self, TUPLE_ACCESS_EXPR));
        }

        if self.at(IDENT) || self.current().is_keyword() {
            self.bump_any();
        } else {
            self.error("expected field name or tuple index");
            return Some(m.complete(self, FIELD_EXPR));
        }

        // If followed by `(`, this is a method call — parse args inline.
        if self.at(L_PAREN) {
            self.bump_any(); // (
            if !self.at(R_PAREN) {
                if self.parse_expr().is_none() && !self.at(R_PAREN) && !self.at(COMMA) {
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
            return Some(m.complete(self, METHOD_CALL_EXPR));
        }

        Some(m.complete(self, FIELD_EXPR))
    }

    /// Parse index expression: `expr[index]`.
    fn parse_index_expr(&mut self, lhs: CompletedMarker) -> Option<CompletedMarker> {
        let m = lhs.precede(self);
        self.bump_any(); // [

        if self.parse_expr().is_none() {
            self.error("expected index expression");
        }

        self.expect(R_BRACKET);

        Some(m.complete(self, INDEX_EXPR))
    }

    /// Parse call expression: `expr(args)`.
    fn parse_call_expr(&mut self, lhs: CompletedMarker) -> Option<CompletedMarker> {
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
            IDENT | KW_FINAL_UPPER => self.parse_ident_expr(opts),

            // Self access
            KW_SELF => self.parse_self_expr(),

            // Block expressions (block, network)
            KW_BLOCK => self.parse_block_access(),
            KW_NETWORK => self.parse_network_access(),

            // Async block expression: `final { ... }`
            KW_FINAL => self.parse_final_block_expr(),

            _ => {
                self.error_unexpected(self.current(), &[
                    "an identifier",
                    "a program id",
                    "an address literal",
                    "an integer literal",
                    "a static string",
                    "'!'",
                    "'-'",
                    "'('",
                    "'['",
                    "'true'",
                    "'false'",
                    "'final'",
                    "'block'",
                    "'network'",
                    "'self'",
                ]);
                None
            }
        }
    }

    /// Parse an integer literal.
    ///
    /// Classifies by suffix: `42field` → `LITERAL_FIELD`, `42group` → `LITERAL_GROUP`,
    /// `42scalar` → `LITERAL_SCALAR`, otherwise `LITERAL_INT`.
    fn parse_integer_literal(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        let text = self.current_text();
        let kind = if text.ends_with("field") {
            LITERAL_FIELD
        } else if text.ends_with("group") {
            LITERAL_GROUP
        } else if text.ends_with("scalar") {
            LITERAL_SCALAR
        } else {
            LITERAL_INT
        };
        self.bump_any();
        Some(m.complete(self, kind))
    }

    /// Parse a string literal.
    fn parse_string_literal(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any();
        Some(m.complete(self, LITERAL_STRING))
    }

    /// Parse an address literal.
    fn parse_address_literal(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any();
        Some(m.complete(self, LITERAL_ADDRESS))
    }

    /// Parse a boolean literal (true/false).
    fn parse_bool_literal(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any();
        Some(m.complete(self, LITERAL_BOOL))
    }

    /// Parse the `none` literal.
    fn parse_none_literal(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any();
        Some(m.complete(self, LITERAL_NONE))
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
                self.error("expected repeat count");
            }
            self.expect(R_BRACKET);
            return Some(m.complete(self, REPEAT_EXPR));
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

            let is_locator = if self.eat(SLASH) {
                // Locator path: name.aleo/Type
                if self.at(IDENT) {
                    self.bump_any();
                }
                true
            } else {
                false
            };

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
                let kind = if is_locator { STRUCT_LOCATOR_EXPR } else { STRUCT_EXPR };
                return Some(m.complete(self, kind));
            }

            // Check for call
            if self.at(L_PAREN) {
                let kind = if is_locator { PATH_LOCATOR_EXPR } else { PROGRAM_REF_EXPR };
                let cm = m.complete(self, kind);
                return self.parse_call_expr(cm);
            }

            let kind = if is_locator { PATH_LOCATOR_EXPR } else { PROGRAM_REF_EXPR };
            return Some(m.complete(self, kind));
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
                self.error("expected identifier after ::");
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
            return self.parse_call_expr(cm);
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
                    self.error("expected field value");
                }
                m.complete(self, STRUCT_FIELD_INIT);
            } else {
                // Shorthand: `{ x }` means `{ x: x }`
                m.complete(self, STRUCT_FIELD_SHORTHAND);
            }
        } else {
            self.error("expected field name");
            m.complete(self, STRUCT_FIELD_INIT);
        }
    }

    /// Parse `self` expression.
    fn parse_self_expr(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any(); // self

        // `self` can only be followed by `.` for member access, not `::`
        if self.at(COLON_COLON) {
            self.error("expected '.' -- found '::'");
        }

        Some(m.complete(self, SELF_EXPR))
    }

    /// Parse `block.height` access.
    fn parse_block_access(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any(); // block
        Some(m.complete(self, BLOCK_KW_EXPR))
    }

    /// Parse `network.id` access.
    fn parse_network_access(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any(); // network
        Some(m.complete(self, NETWORK_KW_EXPR))
    }

    /// Parse a final block expression: `final { stmts }`.
    fn parse_final_block_expr(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any(); // final
        self.skip_trivia();
        if self.parse_block().is_none() {
            self.error("expected block after 'final'");
        }
        Some(m.complete(self, FINAL_EXPR))
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
        let parse: Parse = parser.finish(vec![]);
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
              LITERAL_INT@0..2
                INTEGER@0..2 "42"
        "#]]);
    }

    #[test]
    fn parse_expr_bool_true() {
        check_expr("true", expect![[r#"
            ROOT@0..4
              LITERAL_BOOL@0..4
                KW_TRUE@0..4 "true"
        "#]]);
    }

    #[test]
    fn parse_expr_bool_false() {
        check_expr("false", expect![[r#"
            ROOT@0..5
              LITERAL_BOOL@0..5
                KW_FALSE@0..5 "false"
        "#]]);
    }

    #[test]
    fn parse_expr_none() {
        check_expr("none", expect![[r#"
            ROOT@0..4
              LITERAL_NONE@0..4
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
              SELF_EXPR@0..4
                KW_SELF@0..4 "self"
        "#]]);
    }

    #[test]
    fn parse_expr_self_colon_colon_is_error() {
        // `self::y` is invalid - self can only be followed by `.` not `::`
        let (tokens, _) = lex("self::y");
        let mut parser = Parser::new("self::y", &tokens);
        let root = parser.start();
        parser.parse_expr();
        parser.skip_trivia();
        root.complete(&mut parser, ROOT);
        let parse: Parse = parser.finish(vec![]);
        assert!(!parse.errors().is_empty(), "expected error for self::");
        assert!(
            parse.errors().iter().any(|e| e.message.contains("expected '.'")),
            "expected error message to mention expected '.', got: {:?}",
            parse.errors()
        );
    }

    // =========================================================================
    // Arithmetic
    // =========================================================================

    #[test]
    fn parse_expr_add() {
        check_expr("1 + 2", expect![[r#"
            ROOT@0..5
              BINARY_EXPR@0..5
                LITERAL_INT@0..1
                  INTEGER@0..1 "1"
                WHITESPACE@1..2 " "
                PLUS@2..3 "+"
                WHITESPACE@3..4 " "
                LITERAL_INT@4..5
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
                LITERAL_INT@0..1
                  INTEGER@0..1 "1"
                WHITESPACE@1..2 " "
                PLUS@2..3 "+"
                WHITESPACE@3..4 " "
                BINARY_EXPR@4..9
                  LITERAL_INT@4..5
                    INTEGER@4..5 "2"
                  WHITESPACE@5..6 " "
                  STAR@6..7 "*"
                  WHITESPACE@7..8 " "
                  LITERAL_INT@8..9
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
                  TUPLE_ACCESS_EXPR@0..7
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
                LITERAL_INT@4..5
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
                  METHOD_CALL_EXPR@0..7
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
                TYPE_PRIMITIVE@5..8
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
                LITERAL_INT@1..2
                  INTEGER@1..2 "1"
                COMMA@2..3 ","
                WHITESPACE@3..4 " "
                LITERAL_INT@4..5
                  INTEGER@4..5 "2"
                COMMA@5..6 ","
                WHITESPACE@6..7 " "
                LITERAL_INT@7..8
                  INTEGER@7..8 "3"
                R_BRACKET@8..9 "]"
        "#]]);
    }

    #[test]
    fn parse_expr_array_repeat() {
        check_expr("[0; 10]", expect![[r#"
            ROOT@0..7
              REPEAT_EXPR@0..7
                L_BRACKET@0..1 "["
                LITERAL_INT@1..2
                  INTEGER@1..2 "0"
                SEMICOLON@2..3 ";"
                WHITESPACE@3..4 " "
                LITERAL_INT@4..6
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
                  LITERAL_INT@11..12
                    INTEGER@11..12 "1"
                COMMA@12..13 ","
                STRUCT_FIELD_INIT@13..18
                  WHITESPACE@13..14 " "
                  IDENT@14..15 "y"
                  COLON@15..16 ":"
                  WHITESPACE@16..17 " "
                  LITERAL_INT@17..18
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
                STRUCT_FIELD_SHORTHAND@7..9
                  WHITESPACE@7..8 " "
                  IDENT@8..9 "x"
                COMMA@9..10 ","
                STRUCT_FIELD_SHORTHAND@10..13
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
        let parse: Parse = parser.finish(vec![]);
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
        // Function call with const generic integer arg: CONST_ARG_LIST is inside PATH_EXPR.
        check_expr("foo::[5]()", expect![[r#"
            ROOT@0..10
              CALL_EXPR@0..10
                PATH_EXPR@0..8
                  IDENT@0..3 "foo"
                  COLON_COLON@3..5 "::"
                  CONST_ARG_LIST@5..8
                    L_BRACKET@5..6 "["
                    LITERAL_INT@6..7
                      INTEGER@6..7 "5"
                    R_BRACKET@7..8 "]"
                L_PAREN@8..9 "("
                R_PAREN@9..10 ")"
        "#]]);
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
        // Struct literal with const generic arg: CONST_ARG_LIST is inside STRUCT_EXPR.
        check_expr("Foo::[8u32] { arr: x }", expect![[r#"
            ROOT@0..22
              STRUCT_EXPR@0..22
                IDENT@0..3 "Foo"
                COLON_COLON@3..5 "::"
                CONST_ARG_LIST@5..11
                  L_BRACKET@5..6 "["
                  LITERAL_INT@6..10
                    INTEGER@6..10 "8u32"
                  R_BRACKET@10..11 "]"
                WHITESPACE@11..12 " "
                L_BRACE@12..13 "{"
                STRUCT_FIELD_INIT@13..21
                  WHITESPACE@13..14 " "
                  IDENT@14..17 "arr"
                  COLON@17..18 ":"
                  WHITESPACE@18..19 " "
                  PATH_EXPR@19..21
                    IDENT@19..20 "x"
                    WHITESPACE@20..21 " "
                R_BRACE@21..22 "}"
        "#]]);
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

    // =========================================================================
    // Non-Associative Operator Chaining (should produce errors)
    // =========================================================================

    fn parse_expr_for_test(input: &str) -> Parse {
        let (tokens, _) = lex(input);
        let mut parser = Parser::new(input, &tokens);
        let root = parser.start();
        parser.parse_expr();
        parser.skip_trivia();
        root.complete(&mut parser, ROOT);
        parser.finish(vec![])
    }

    #[test]
    fn parse_expr_chained_eq_is_error() {
        // Chained == is not allowed: 1 == 2 == 3
        let parse = parse_expr_for_test("1 == 2 == 3");
        assert!(!parse.errors().is_empty(), "expected error for chained ==, got none");
        assert!(
            parse.errors().iter().any(|e| e.message.contains("'&&'") || e.message.contains("expected")),
            "expected error message about valid operators, got: {:?}",
            parse.errors()
        );
    }

    #[test]
    fn parse_expr_chained_neq_is_error() {
        // Chained != is not allowed: 1 != 2 != 3
        let parse = parse_expr_for_test("1 != 2 != 3");
        assert!(!parse.errors().is_empty(), "expected error for chained !=, got none");
    }

    #[test]
    fn parse_expr_chained_lt_is_error() {
        // Chained < is not allowed: 1 < 2 < 3
        let parse = parse_expr_for_test("1 < 2 < 3");
        assert!(!parse.errors().is_empty(), "expected error for chained <, got none");
    }

    #[test]
    fn parse_expr_chained_gt_is_error() {
        // Chained > is not allowed: 1 > 2 > 3
        let parse = parse_expr_for_test("1 > 2 > 3");
        assert!(!parse.errors().is_empty(), "expected error for chained >, got none");
    }

    #[test]
    fn parse_expr_comparison_with_logical_is_ok() {
        // Comparison followed by logical is allowed: 1 == 2 && 3 == 4
        check_expr_no_errors("1 == 2 && 3 == 4");
        check_expr_no_errors("1 < 2 || 3 > 4");
    }

    // =========================================================================
    // Associated function calls (type keyword :: function)
    // =========================================================================

    #[test]
    fn parse_expr_group_associated_fn() {
        // The lexer produces a single IDENT token for "group::to_x_coordinate"
        // via the PathSpecial regex pattern.
        check_expr("group::to_x_coordinate(a)", expect![[r#"
                ROOT@0..25
                  CALL_EXPR@0..25
                    PATH_EXPR@0..22
                      IDENT@0..22 "group::to_x_coordinate"
                    L_PAREN@22..23 "("
                    PATH_EXPR@23..24
                      IDENT@23..24 "a"
                    R_PAREN@24..25 ")"
            "#]]);
    }

    #[test]
    fn parse_expr_signature_associated_fn() {
        // The lexer produces a single IDENT token for "signature::verify"
        // via the PathSpecial regex pattern.
        check_expr("signature::verify(s, a, v)", expect![[r#"
                ROOT@0..26
                  CALL_EXPR@0..26
                    PATH_EXPR@0..17
                      IDENT@0..17 "signature::verify"
                    L_PAREN@17..18 "("
                    PATH_EXPR@18..19
                      IDENT@18..19 "s"
                    COMMA@19..20 ","
                    WHITESPACE@20..21 " "
                    PATH_EXPR@21..22
                      IDENT@21..22 "a"
                    COMMA@22..23 ","
                    WHITESPACE@23..24 " "
                    PATH_EXPR@24..25
                      IDENT@24..25 "v"
                    R_PAREN@25..26 ")"
            "#]]);
    }

    // =========================================================================
    // Chained Comparison Errors (1a)
    // =========================================================================

    #[test]
    fn parse_expr_chained_le_is_error() {
        let parse = parse_expr_for_test("1 <= 2 <= 3");
        assert!(!parse.errors().is_empty(), "expected error for chained <=, got none");
    }

    #[test]
    fn parse_expr_chained_ge_is_error() {
        let parse = parse_expr_for_test("1 >= 2 >= 3");
        assert!(!parse.errors().is_empty(), "expected error for chained >=, got none");
    }

    #[test]
    fn parse_expr_chained_mixed_cmp_is_error() {
        let parse = parse_expr_for_test("1 < 2 > 3");
        assert!(!parse.errors().is_empty(), "expected error for mixed chained comparisons, got none");
    }

    // =========================================================================
    // Nested Ternary (1b)
    // =========================================================================

    #[test]
    fn parse_expr_ternary_nested() {
        check_expr("a ? b ? c : d : e", expect![[r#"
            ROOT@0..17
              TERNARY_EXPR@0..17
                PATH_EXPR@0..2
                  IDENT@0..1 "a"
                  WHITESPACE@1..2 " "
                QUESTION@2..3 "?"
                WHITESPACE@3..4 " "
                TERNARY_EXPR@4..14
                  PATH_EXPR@4..6
                    IDENT@4..5 "b"
                    WHITESPACE@5..6 " "
                  QUESTION@6..7 "?"
                  WHITESPACE@7..8 " "
                  PATH_EXPR@8..10
                    IDENT@8..9 "c"
                    WHITESPACE@9..10 " "
                  COLON@10..11 ":"
                  WHITESPACE@11..12 " "
                  PATH_EXPR@12..14
                    IDENT@12..13 "d"
                    WHITESPACE@13..14 " "
                COLON@14..15 ":"
                WHITESPACE@15..16 " "
                PATH_EXPR@16..17
                  IDENT@16..17 "e"
        "#]]);
    }

    // =========================================================================
    // Chained Casts (1c)
    // =========================================================================

    #[test]
    fn parse_expr_cast_chained() {
        check_expr("x as u32 as u64", expect![[r#"
            ROOT@0..15
              CAST_EXPR@0..15
                CAST_EXPR@0..8
                  PATH_EXPR@0..2
                    IDENT@0..1 "x"
                    WHITESPACE@1..2 " "
                  KW_AS@2..4 "as"
                  WHITESPACE@4..5 " "
                  TYPE_PRIMITIVE@5..8
                    KW_U32@5..8 "u32"
                WHITESPACE@8..9 " "
                KW_AS@9..11 "as"
                WHITESPACE@11..12 " "
                TYPE_PRIMITIVE@12..15
                  KW_U64@12..15 "u64"
        "#]]);
    }

    // =========================================================================
    // Collection Edge Cases (1d)
    // =========================================================================

    #[test]
    fn parse_expr_array_trailing_comma() {
        check_expr("[1, 2, 3,]", expect![[r#"
            ROOT@0..10
              ARRAY_EXPR@0..10
                L_BRACKET@0..1 "["
                LITERAL_INT@1..2
                  INTEGER@1..2 "1"
                COMMA@2..3 ","
                WHITESPACE@3..4 " "
                LITERAL_INT@4..5
                  INTEGER@4..5 "2"
                COMMA@5..6 ","
                WHITESPACE@6..7 " "
                LITERAL_INT@7..8
                  INTEGER@7..8 "3"
                COMMA@8..9 ","
                R_BRACKET@9..10 "]"
        "#]]);
    }

    #[test]
    fn parse_expr_array_empty() {
        check_expr("[]", expect![[r#"
            ROOT@0..2
              ARRAY_EXPR@0..2
                L_BRACKET@0..1 "["
                R_BRACKET@1..2 "]"
        "#]]);
    }

    #[test]
    fn parse_expr_tuple_single() {
        check_expr("(a,)", expect![[r#"
            ROOT@0..4
              TUPLE_EXPR@0..4
                L_PAREN@0..1 "("
                PATH_EXPR@1..2
                  IDENT@1..2 "a"
                COMMA@2..3 ","
                R_PAREN@3..4 ")"
        "#]]);
    }

    #[test]
    fn parse_expr_tuple_trailing_comma() {
        check_expr("(1, 2,)", expect![[r#"
            ROOT@0..7
              TUPLE_EXPR@0..7
                L_PAREN@0..1 "("
                LITERAL_INT@1..2
                  INTEGER@1..2 "1"
                COMMA@2..3 ","
                WHITESPACE@3..4 " "
                LITERAL_INT@4..5
                  INTEGER@4..5 "2"
                COMMA@5..6 ","
                R_PAREN@6..7 ")"
        "#]]);
    }

    // =========================================================================
    // Struct Literal Edge Cases (1e)
    // =========================================================================

    #[test]
    fn parse_expr_struct_empty() {
        check_expr("Point { }", expect![[r#"
            ROOT@0..9
              STRUCT_EXPR@0..9
                IDENT@0..5 "Point"
                WHITESPACE@5..6 " "
                L_BRACE@6..7 "{"
                WHITESPACE@7..8 " "
                R_BRACE@8..9 "}"
        "#]]);
    }

    #[test]
    fn parse_expr_struct_trailing_comma() {
        check_expr("Point { x: 1, }", expect![[r#"
            ROOT@0..15
              STRUCT_EXPR@0..15
                IDENT@0..5 "Point"
                WHITESPACE@5..6 " "
                L_BRACE@6..7 "{"
                STRUCT_FIELD_INIT@7..12
                  WHITESPACE@7..8 " "
                  IDENT@8..9 "x"
                  COLON@9..10 ":"
                  WHITESPACE@10..11 " "
                  LITERAL_INT@11..12
                    INTEGER@11..12 "1"
                COMMA@12..13 ","
                WHITESPACE@13..14 " "
                R_BRACE@14..15 "}"
        "#]]);
    }

    #[test]
    fn parse_expr_struct_mixed_fields() {
        check_expr("Point { x, y: 2 }", expect![[r#"
            ROOT@0..17
              STRUCT_EXPR@0..17
                IDENT@0..5 "Point"
                WHITESPACE@5..6 " "
                L_BRACE@6..7 "{"
                STRUCT_FIELD_SHORTHAND@7..9
                  WHITESPACE@7..8 " "
                  IDENT@8..9 "x"
                COMMA@9..10 ","
                STRUCT_FIELD_INIT@10..15
                  WHITESPACE@10..11 " "
                  IDENT@11..12 "y"
                  COLON@12..13 ":"
                  WHITESPACE@13..14 " "
                  LITERAL_INT@14..15
                    INTEGER@14..15 "2"
                WHITESPACE@15..16 " "
                R_BRACE@16..17 "}"
        "#]]);
    }

    // =========================================================================
    // Additional Literals (1f)
    // =========================================================================

    #[test]
    fn parse_expr_string() {
        check_expr("\"hello\"", expect![[r#"
            ROOT@0..7
              LITERAL_STRING@0..7
                STRING@0..7 "\"hello\""
        "#]]);
    }

    #[test]
    fn parse_expr_address() {
        check_expr("aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8s7pyjh9", expect![[r#"
            ROOT@0..63
              LITERAL_ADDRESS@0..63
                ADDRESS_LIT@0..63 "aleo1qnr4dkkvkgfqph0v ..."
        "#]]);
    }

    // =========================================================================
    // Deep Postfix Chains (1g)
    // =========================================================================

    #[test]
    fn parse_expr_deep_postfix() {
        check_expr("a[0].b.c(x)[1]", expect![[r#"
            ROOT@0..14
              INDEX_EXPR@0..14
                METHOD_CALL_EXPR@0..11
                  FIELD_EXPR@0..6
                    INDEX_EXPR@0..4
                      PATH_EXPR@0..1
                        IDENT@0..1 "a"
                      L_BRACKET@1..2 "["
                      LITERAL_INT@2..3
                        INTEGER@2..3 "0"
                      R_BRACKET@3..4 "]"
                    DOT@4..5 "."
                    IDENT@5..6 "b"
                  DOT@6..7 "."
                  IDENT@7..8 "c"
                  L_PAREN@8..9 "("
                  PATH_EXPR@9..10
                    IDENT@9..10 "x"
                  R_PAREN@10..11 ")"
                L_BRACKET@11..12 "["
                LITERAL_INT@12..13
                  INTEGER@12..13 "1"
                R_BRACKET@13..14 "]"
        "#]]);
    }

    // =========================================================================
    // Final Expression (1h)
    // =========================================================================

    #[test]
    fn parse_expr_final() {
        check_expr("final { foo() }", expect![[r#"
            ROOT@0..15
              FINAL_EXPR@0..15
                KW_FINAL@0..5 "final"
                WHITESPACE@5..6 " "
                BLOCK@6..15
                  L_BRACE@6..7 "{"
                  WHITESPACE@7..8 " "
                  EXPR_STMT@8..14
                    CALL_EXPR@8..13
                      PATH_EXPR@8..11
                        IDENT@8..11 "foo"
                      L_PAREN@11..12 "("
                      R_PAREN@12..13 ")"
                    WHITESPACE@13..14 " "
                  ERROR@14..14
                  R_BRACE@14..15 "}"
        "#]]);
    }

    // =========================================================================
    // Complex Precedence (1i)
    // =========================================================================

    #[test]
    fn parse_expr_mixed_arithmetic() {
        // a + b * c / d - e  =>  (a + ((b * c) / d)) - e
        check_expr("a + b * c / d - e", expect![[r#"
            ROOT@0..17
              BINARY_EXPR@0..17
                BINARY_EXPR@0..14
                  PATH_EXPR@0..2
                    IDENT@0..1 "a"
                    WHITESPACE@1..2 " "
                  PLUS@2..3 "+"
                  WHITESPACE@3..4 " "
                  BINARY_EXPR@4..14
                    BINARY_EXPR@4..10
                      PATH_EXPR@4..6
                        IDENT@4..5 "b"
                        WHITESPACE@5..6 " "
                      STAR@6..7 "*"
                      WHITESPACE@7..8 " "
                      PATH_EXPR@8..10
                        IDENT@8..9 "c"
                        WHITESPACE@9..10 " "
                    SLASH@10..11 "/"
                    WHITESPACE@11..12 " "
                    PATH_EXPR@12..14
                      IDENT@12..13 "d"
                      WHITESPACE@13..14 " "
                MINUS@14..15 "-"
                WHITESPACE@15..16 " "
                PATH_EXPR@16..17
                  IDENT@16..17 "e"
        "#]]);
    }

    #[test]
    fn parse_expr_bitwise_precedence() {
        // a | b & c ^ d  =>  a | ((b & c) ^ d)  ... actually:
        // & (BP 16,17) binds tighter than ^ (14,15) tighter than | (12,13)
        // so: a | ((b & c) ^ d)
        check_expr("a | b & c ^ d", expect![[r#"
            ROOT@0..13
              BINARY_EXPR@0..13
                PATH_EXPR@0..2
                  IDENT@0..1 "a"
                  WHITESPACE@1..2 " "
                PIPE@2..3 "|"
                WHITESPACE@3..4 " "
                BINARY_EXPR@4..13
                  BINARY_EXPR@4..10
                    PATH_EXPR@4..6
                      IDENT@4..5 "b"
                      WHITESPACE@5..6 " "
                    AMP@6..7 "&"
                    WHITESPACE@7..8 " "
                    PATH_EXPR@8..10
                      IDENT@8..9 "c"
                      WHITESPACE@9..10 " "
                  CARET@10..11 "^"
                  WHITESPACE@11..12 " "
                  PATH_EXPR@12..13
                    IDENT@12..13 "d"
        "#]]);
    }

    #[test]
    fn parse_expr_shift_chain() {
        // << and >> are left-assoc at same precedence
        // x << 1 >> 2  =>  (x << 1) >> 2
        check_expr("x << 1 >> 2", expect![[r#"
            ROOT@0..11
              BINARY_EXPR@0..11
                BINARY_EXPR@0..6
                  PATH_EXPR@0..2
                    IDENT@0..1 "x"
                    WHITESPACE@1..2 " "
                  SHL@2..4 "<<"
                  WHITESPACE@4..5 " "
                  LITERAL_INT@5..6
                    INTEGER@5..6 "1"
                WHITESPACE@6..7 " "
                SHR@7..9 ">>"
                WHITESPACE@9..10 " "
                LITERAL_INT@10..11
                  INTEGER@10..11 "2"
        "#]]);
    }
}
