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

//! Type parsing for the Leo language.
//!
//! This module implements parsing for all Leo type expressions:
//! - Primitive types: bool, field, group, scalar, address, signature, string
//! - Integer types: u8, u16, u32, u64, u128, i8, i16, i32, i64, i128
//! - Array types: [Type; len] or [Type] (vector)
//! - Tuple types: (Type1, Type2, ...) or () (unit)
//! - Optional types: Type?
//! - Future types: Future or Future<fn(Types) -> Type>
//! - Composite types: Named, Foo::[N], program.aleo/Type (locator)

use super::{CompletedMarker, Parser};
use crate::syntax_kind::SyntaxKind::*;

/// Options for type parsing to handle context-sensitive cases.
#[derive(Default, Clone, Copy)]
pub struct TypeOpts {
    /// Whether to allow the optional `?` suffix.
    pub allow_optional: bool,
}

impl TypeOpts {
    /// Allow optional suffix on parsed type.
    pub fn allow_optional(mut self) -> Self {
        self.allow_optional = true;
        self
    }
}

impl Parser<'_, '_> {
    /// Primitive type keywords allowed in cast expressions.
    pub const PRIMITIVE_TYPE_KINDS: &'static [crate::syntax_kind::SyntaxKind] = &[
        KW_ADDRESS,
        KW_BOOL,
        KW_FIELD,
        KW_GROUP,
        KW_SCALAR,
        KW_SIGNATURE,
        KW_STRING,
        KW_I8,
        KW_I16,
        KW_I32,
        KW_I64,
        KW_I128,
        KW_U8,
        KW_U16,
        KW_U32,
        KW_U64,
        KW_U128,
    ];

    /// Parse a type expression.
    ///
    /// Returns `None` if the current token cannot start a type.
    pub fn parse_type(&mut self) -> Option<CompletedMarker> {
        self.parse_type_with_opts(TypeOpts::default().allow_optional())
    }

    /// Parse a type expression with options.
    pub fn parse_type_with_opts(&mut self, opts: TypeOpts) -> Option<CompletedMarker> {
        let ty = self.parse_type_inner()?;

        // Optional suffix: Type?
        if opts.allow_optional && self.at(QUESTION) {
            let m = ty.precede(self);
            self.bump_any(); // ?
            return Some(m.complete(self, TYPE_OPTIONAL));
        }

        Some(ty)
    }

    /// Parse the core type expression (without optional suffix).
    fn parse_type_inner(&mut self) -> Option<CompletedMarker> {
        // Skip leading trivia before starting the type node
        self.skip_trivia();

        match self.current() {
            // Unit or Tuple: ()  or (T1, T2, ...)
            L_PAREN => self.parse_tuple_type(),
            // Array or Vector: [T; N] or [T]
            L_BRACKET => self.parse_array_or_vector_type(),
            // Final: Final or Final<...>
            KW_FINAL_UPPER => self.parse_future_type(),
            // Mapping type (storage context): mapping key => value
            KW_MAPPING => self.parse_mapping_type(),
            // Primitive type keywords
            _ if self.at_primitive_type() => self.parse_primitive_type(),
            // Named/Composite type: Foo, Foo::[N], program.aleo/Type
            IDENT => self.parse_named_type(),
            _ => None,
        }
    }

    /// Check if the current token is a primitive type keyword.
    fn at_primitive_type(&self) -> bool {
        Self::PRIMITIVE_TYPE_KINDS.contains(&self.current())
    }

    /// Parse a cast type (primitive types only, matching the LALRPOP grammar).
    pub fn parse_cast_type(&mut self) -> Option<CompletedMarker> {
        if self.at_primitive_type() { self.parse_primitive_type() } else { None }
    }

    /// Parse a primitive type keyword.
    fn parse_primitive_type(&mut self) -> Option<CompletedMarker> {
        if !self.at_primitive_type() {
            return None;
        }

        // Skip trivia before starting the node so leading whitespace
        // isn't captured inside the TYPE_PRIMITIVE.
        self.skip_trivia();
        let m = self.start();
        self.bump_raw();
        Some(m.complete(self, TYPE_PRIMITIVE))
    }

    /// Parse a tuple type: `(T1, T2, ...)` or unit `()`.
    fn parse_tuple_type(&mut self) -> Option<CompletedMarker> {
        if !self.at(L_PAREN) {
            return None;
        }

        let m = self.start();
        self.bump_any(); // (

        // Check for unit: ()
        if self.eat(R_PAREN) {
            return Some(m.complete(self, TYPE_TUPLE));
        }

        // Parse first element
        self.parse_type_with_opts(TypeOpts::default().allow_optional());

        // Parse remaining elements
        while self.eat(COMMA) {
            if self.at(R_PAREN) {
                // Trailing comma
                break;
            }
            self.parse_type_with_opts(TypeOpts::default().allow_optional());
        }

        self.expect(R_PAREN);
        Some(m.complete(self, TYPE_TUPLE))
    }

    /// Parse an array type `[T; N]` or vector type `[T]`.
    fn parse_array_or_vector_type(&mut self) -> Option<CompletedMarker> {
        if !self.at(L_BRACKET) {
            return None;
        }

        let m = self.start();
        self.bump_any(); // [

        // Parse element type
        self.parse_type_with_opts(TypeOpts::default().allow_optional());

        // Check for array length: ; N
        if self.eat(SEMICOLON) {
            // Array with explicit length: [T; N]
            let len = self.start();
            self.parse_array_length();
            len.complete(self, ARRAY_LENGTH);
            self.expect(R_BRACKET);
            return Some(m.complete(self, TYPE_ARRAY));
        }

        // If we see ',' instead of ';' or ']', suggest the array syntax.
        if self.at(COMMA) {
            self.error("expected ';' for array type, found ','");
            self.recover(&[R_BRACKET]);
            self.eat(R_BRACKET);
            return Some(m.complete(self, TYPE_ARRAY));
        }

        // Vector: [T]
        self.expect(R_BRACKET);
        Some(m.complete(self, TYPE_VECTOR))
    }

    /// Parse an array length expression.
    ///
    /// Supports integer literals (`[T; 10]`), identifiers (`[T; N]`),
    /// paths (`[T; Foo::SIZE]`), and arbitrary expressions (`[T; N + M]`).
    fn parse_array_length(&mut self) {
        self.parse_expr();
    }

    /// Parse a Final type: `Final` or `Final<fn(T1, T2) -> R>`.
    fn parse_future_type(&mut self) -> Option<CompletedMarker> {
        if !self.at(KW_FINAL_UPPER) {
            return None;
        }

        let m = self.start();
        self.bump_any(); // Future

        // Check for explicit Future signature: Future<fn(T) -> R>
        if self.eat(LT) {
            // Parse fn(...) -> R
            self.expect(KW_FN_UPPER);
            self.expect(L_PAREN);

            // Parse parameter types
            if !self.at(R_PAREN) {
                self.parse_type_with_opts(TypeOpts::default().allow_optional());
                while self.eat(COMMA) {
                    if self.at(R_PAREN) {
                        break;
                    }
                    self.parse_type_with_opts(TypeOpts::default().allow_optional());
                }
            }

            self.expect(R_PAREN);

            // Return type
            if self.eat(ARROW) {
                self.parse_type_with_opts(TypeOpts::default().allow_optional());
            }

            self.expect(GT);
        }

        Some(m.complete(self, TYPE_FINAL))
    }

    /// Parse a mapping type: `mapping key => value`.
    fn parse_mapping_type(&mut self) -> Option<CompletedMarker> {
        if !self.at(KW_MAPPING) {
            return None;
        }

        let m = self.start();
        self.bump_any(); // mapping

        // Parse key type
        self.parse_type();

        // Expect =>
        self.expect(FAT_ARROW);

        // Parse value type
        self.parse_type();

        Some(m.complete(self, TYPE_MAPPING))
    }

    /// Parse a named/composite type.
    ///
    /// This handles:
    /// - Simple names: `Foo`
    /// - Paths: `Foo::Bar`
    /// - Const generics: `Foo::[N]` or `Foo::<N>`
    /// - Locators: `program.aleo/Type`
    fn parse_named_type(&mut self) -> Option<CompletedMarker> {
        if !self.at(IDENT) {
            return None;
        }

        let m = self.start();
        self.bump_any(); // first identifier

        // Check for locator: name.aleo/Type
        if self.at(DOT) && self.nth(1) == KW_ALEO {
            self.bump_any(); // .
            self.bump_any(); // aleo

            if self.eat(SLASH) {
                // Locator path: program.aleo/Type
                if self.at(IDENT) {
                    self.bump_any();
                } else {
                    self.error("expected type name after /");
                }
            }

            // Optional const generic args after locator: child.aleo/Bar::[4]
            if self.at(COLON_COLON) && self.nth(1) == L_BRACKET {
                self.bump_any(); // ::
                self.parse_const_generic_args_bracket();
            }

            return Some(m.complete(self, TYPE_LOCATOR));
        }

        // Check for path or const generics: Foo::Bar or Foo::[N]
        while self.eat(COLON_COLON) {
            if self.at(L_BRACKET) {
                // Const generics with brackets: Foo::[N]
                self.parse_const_generic_args_bracket();
                break;
            } else if self.at(LT) {
                // Const generics with angle brackets: Foo::<N>
                self.parse_const_generic_args_angle();
                break;
            } else if self.at(IDENT) {
                self.bump_any();
            } else {
                self.error("expected identifier, [, or < after ::");
                break;
            }
        }

        Some(m.complete(self, TYPE_PATH))
    }

    /// Parse a const generic parameter list (declaration site): `::[N: u32, M: u32]`.
    ///
    /// Wraps the list in a `CONST_PARAM_LIST` node. Each parameter is
    /// wrapped in a `CONST_PARAM` node containing `name: Type`.
    pub fn parse_const_param_list(&mut self) {
        let m = self.start();

        if !self.eat(L_BRACKET) {
            m.abandon(self);
            return;
        }

        // Parse comma-separated const params
        if !self.at(R_BRACKET) {
            self.parse_const_param();
            while self.eat(COMMA) {
                if self.at(R_BRACKET) {
                    break;
                }
                self.parse_const_param();
            }
        }

        self.expect(R_BRACKET);
        m.complete(self, CONST_PARAM_LIST);
    }

    /// Parse a single const generic parameter: `N: u32`.
    fn parse_const_param(&mut self) {
        let m = self.start();
        self.skip_trivia();

        if self.at(IDENT) {
            self.bump_any(); // param name
        } else {
            self.error("expected const parameter name");
        }

        self.expect(COLON);
        self.parse_type();

        m.complete(self, CONST_PARAM);
    }

    /// Parse const generic arguments with bracket syntax (use site): `::[N]` or `::[N + 1, u32]`.
    ///
    /// Each argument may be an expression or a type (for intrinsics).
    /// Wraps the list in a `CONST_ARG_LIST` node.
    pub fn parse_const_generic_args_bracket(&mut self) {
        let m = self.start();

        if !self.eat(L_BRACKET) {
            m.abandon(self);
            return;
        }

        // Parse comma-separated arguments
        if !self.at(R_BRACKET) {
            self.parse_const_generic_arg();
            while self.eat(COMMA) {
                if self.at(R_BRACKET) {
                    break;
                }
                // Clear error state so each argument gets fresh error reporting.
                self.erroring = false;
                self.parse_const_generic_arg();
            }
        }

        self.expect(R_BRACKET);
        m.complete(self, CONST_ARG_LIST);
    }

    /// Parse const generic arguments with angle bracket syntax: `::<N>` or `::<N, M>`.
    ///
    /// Only accepts simple IDENT/INTEGER arguments because `>` conflicts
    /// with the expression parser's greater-than operator. Angle bracket
    /// generics are low priority (the LALRPOP grammar doesn't use them).
    pub fn parse_const_generic_args_angle(&mut self) {
        let m = self.start();

        if !self.eat(LT) {
            m.abandon(self);
            return;
        }

        // Simple arg parser — avoids expression parser which would consume `>`.
        if self.at(IDENT) || self.at(INTEGER) {
            self.bump_any();
        }

        while self.eat(COMMA) {
            if self.at(GT) {
                break;
            }
            if self.at(IDENT) || self.at(INTEGER) {
                self.bump_any();
            } else {
                self.error("expected const generic argument");
                break;
            }
        }

        self.expect(GT);
        m.complete(self, CONST_ARG_LIST);
    }

    /// Parse a single const generic argument (expression or type).
    ///
    /// At use sites, `::[]` can contain either expressions (`N + 1`, `5`)
    /// or types (`u32`, `[u8; 4]`). The heuristic: if the current token
    /// is a primitive type keyword, or `[` followed by a primitive type
    /// keyword (array type arg), parse as a type. Otherwise parse as an
    /// expression.
    fn parse_const_generic_arg(&mut self) {
        self.skip_trivia();
        if self.at_primitive_type() {
            // Type argument (e.g. `u32` in `Deserialize::[u32]`)
            self.parse_type();
        } else if self.at(L_BRACKET) && self.nth(1).is_type_keyword() {
            // Array type argument (e.g. `[u8; 4]` in `Deserialize::[[u8; 4]]`)
            self.parse_type();
        } else {
            // Expression argument (e.g. `N + 1`, `5`, `N`)
            self.parse_expr();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexer::lex, parser::Parse};
    use expect_test::{Expect, expect};

    fn check_type(input: &str, expect: Expect) {
        let (tokens, _) = lex(input);
        let mut parser = Parser::new(input, &tokens);
        let root = parser.start();
        parser.parse_type();
        parser.skip_trivia();
        root.complete(&mut parser, ROOT);
        let parse: Parse = parser.finish(vec![]);
        let output = format!("{:#?}", parse.syntax());
        expect.assert_eq(&output);
    }

    fn check_type_optional(input: &str, expect: Expect) {
        let (tokens, _) = lex(input);
        let mut parser = Parser::new(input, &tokens);
        let root = parser.start();
        parser.parse_type_with_opts(TypeOpts::default().allow_optional());
        parser.skip_trivia();
        root.complete(&mut parser, ROOT);
        let parse: Parse = parser.finish(vec![]);
        let output = format!("{:#?}", parse.syntax());
        expect.assert_eq(&output);
    }

    // =========================================================================
    // Primitive Types
    // =========================================================================

    #[test]
    fn parse_type_bool() {
        check_type("bool", expect![[r#"
            ROOT@0..4
              TYPE_PRIMITIVE@0..4
                KW_BOOL@0..4 "bool"
        "#]]);
    }

    #[test]
    fn parse_type_field() {
        check_type("field", expect![[r#"
            ROOT@0..5
              TYPE_PRIMITIVE@0..5
                KW_FIELD@0..5 "field"
        "#]]);
    }

    #[test]
    fn parse_type_group() {
        check_type("group", expect![[r#"
            ROOT@0..5
              TYPE_PRIMITIVE@0..5
                KW_GROUP@0..5 "group"
        "#]]);
    }

    #[test]
    fn parse_type_address() {
        check_type("address", expect![[r#"
            ROOT@0..7
              TYPE_PRIMITIVE@0..7
                KW_ADDRESS@0..7 "address"
        "#]]);
    }

    #[test]
    fn parse_type_scalar() {
        check_type("scalar", expect![[r#"
            ROOT@0..6
              TYPE_PRIMITIVE@0..6
                KW_SCALAR@0..6 "scalar"
        "#]]);
    }

    #[test]
    fn parse_type_signature() {
        check_type("signature", expect![[r#"
            ROOT@0..9
              TYPE_PRIMITIVE@0..9
                KW_SIGNATURE@0..9 "signature"
        "#]]);
    }

    #[test]
    fn parse_type_string() {
        check_type("string", expect![[r#"
            ROOT@0..6
              TYPE_PRIMITIVE@0..6
                KW_STRING@0..6 "string"
        "#]]);
    }

    #[test]
    fn parse_type_u32() {
        check_type("u32", expect![[r#"
            ROOT@0..3
              TYPE_PRIMITIVE@0..3
                KW_U32@0..3 "u32"
        "#]]);
    }

    #[test]
    fn parse_type_i128() {
        check_type("i128", expect![[r#"
            ROOT@0..4
              TYPE_PRIMITIVE@0..4
                KW_I128@0..4 "i128"
        "#]]);
    }

    // =========================================================================
    // Tuple Types
    // =========================================================================

    #[test]
    fn parse_type_unit() {
        check_type("()", expect![[r#"
                ROOT@0..2
                  TYPE_TUPLE@0..2
                    L_PAREN@0..1 "("
                    R_PAREN@1..2 ")"
            "#]]);
    }

    #[test]
    fn parse_type_tuple_single() {
        check_type("(u32)", expect![[r#"
            ROOT@0..5
              TYPE_TUPLE@0..5
                L_PAREN@0..1 "("
                TYPE_PRIMITIVE@1..4
                  KW_U32@1..4 "u32"
                R_PAREN@4..5 ")"
        "#]]);
    }

    #[test]
    fn parse_type_tuple_pair() {
        check_type("(u32, field)", expect![[r#"
            ROOT@0..12
              TYPE_TUPLE@0..12
                L_PAREN@0..1 "("
                TYPE_PRIMITIVE@1..4
                  KW_U32@1..4 "u32"
                COMMA@4..5 ","
                WHITESPACE@5..6 " "
                TYPE_PRIMITIVE@6..11
                  KW_FIELD@6..11 "field"
                R_PAREN@11..12 ")"
        "#]]);
    }

    #[test]
    fn parse_type_tuple_trailing_comma() {
        check_type("(u32, field,)", expect![[r#"
            ROOT@0..13
              TYPE_TUPLE@0..13
                L_PAREN@0..1 "("
                TYPE_PRIMITIVE@1..4
                  KW_U32@1..4 "u32"
                COMMA@4..5 ","
                WHITESPACE@5..6 " "
                TYPE_PRIMITIVE@6..11
                  KW_FIELD@6..11 "field"
                COMMA@11..12 ","
                R_PAREN@12..13 ")"
        "#]]);
    }

    // =========================================================================
    // Array Types
    // =========================================================================

    #[test]
    fn parse_type_array_fixed() {
        check_type("[u32; 10]", expect![[r#"
            ROOT@0..9
              TYPE_ARRAY@0..9
                L_BRACKET@0..1 "["
                TYPE_PRIMITIVE@1..4
                  KW_U32@1..4 "u32"
                SEMICOLON@4..5 ";"
                ARRAY_LENGTH@5..8
                  WHITESPACE@5..6 " "
                  LITERAL_INT@6..8
                    INTEGER@6..8 "10"
                R_BRACKET@8..9 "]"
        "#]]);
    }

    #[test]
    fn parse_type_array_const_generic() {
        check_type("[field; N]", expect![[r#"
            ROOT@0..10
              TYPE_ARRAY@0..10
                L_BRACKET@0..1 "["
                TYPE_PRIMITIVE@1..6
                  KW_FIELD@1..6 "field"
                SEMICOLON@6..7 ";"
                ARRAY_LENGTH@7..9
                  WHITESPACE@7..8 " "
                  PATH_EXPR@8..9
                    IDENT@8..9 "N"
                R_BRACKET@9..10 "]"
        "#]]);
    }

    #[test]
    fn parse_type_vector() {
        check_type("[u8]", expect![[r#"
            ROOT@0..4
              TYPE_VECTOR@0..4
                L_BRACKET@0..1 "["
                TYPE_PRIMITIVE@1..3
                  KW_U8@1..3 "u8"
                R_BRACKET@3..4 "]"
        "#]]);
    }

    #[test]
    fn parse_type_nested_array() {
        check_type("[[u32; 3]; 2]", expect![[r#"
            ROOT@0..13
              TYPE_ARRAY@0..13
                L_BRACKET@0..1 "["
                TYPE_ARRAY@1..9
                  L_BRACKET@1..2 "["
                  TYPE_PRIMITIVE@2..5
                    KW_U32@2..5 "u32"
                  SEMICOLON@5..6 ";"
                  ARRAY_LENGTH@6..8
                    WHITESPACE@6..7 " "
                    LITERAL_INT@7..8
                      INTEGER@7..8 "3"
                  R_BRACKET@8..9 "]"
                SEMICOLON@9..10 ";"
                ARRAY_LENGTH@10..12
                  WHITESPACE@10..11 " "
                  LITERAL_INT@11..12
                    INTEGER@11..12 "2"
                R_BRACKET@12..13 "]"
        "#]]);
    }

    // =========================================================================
    // Optional Types
    // =========================================================================

    #[test]
    fn parse_type_optional() {
        check_type_optional("u32?", expect![[r#"
            ROOT@0..4
              TYPE_OPTIONAL@0..4
                TYPE_PRIMITIVE@0..3
                  KW_U32@0..3 "u32"
                QUESTION@3..4 "?"
        "#]]);
    }

    #[test]
    fn parse_type_optional_named() {
        check_type_optional("Token?", expect![[r#"
                ROOT@0..6
                  TYPE_OPTIONAL@0..6
                    TYPE_PATH@0..5
                      IDENT@0..5 "Token"
                    QUESTION@5..6 "?"
            "#]]);
    }

    // =========================================================================
    // Named/Composite Types
    // =========================================================================

    #[test]
    fn parse_type_named_simple() {
        check_type("Token", expect![[r#"
                ROOT@0..5
                  TYPE_PATH@0..5
                    IDENT@0..5 "Token"
            "#]]);
    }

    #[test]
    fn parse_type_named_path() {
        check_type("Foo::Bar", expect![[r#"
                ROOT@0..8
                  TYPE_PATH@0..8
                    IDENT@0..3 "Foo"
                    COLON_COLON@3..5 "::"
                    IDENT@5..8 "Bar"
            "#]]);
    }

    #[test]
    fn parse_type_named_const_generic_bracket() {
        check_type("Poseidon::[N]", expect![[r#"
            ROOT@0..13
              TYPE_PATH@0..13
                IDENT@0..8 "Poseidon"
                COLON_COLON@8..10 "::"
                CONST_ARG_LIST@10..13
                  L_BRACKET@10..11 "["
                  PATH_EXPR@11..12
                    IDENT@11..12 "N"
                  R_BRACKET@12..13 "]"
        "#]]);
    }

    #[test]
    fn parse_type_named_const_generic_angle() {
        check_type("Poseidon::<4>", expect![[r#"
            ROOT@0..13
              TYPE_PATH@0..13
                IDENT@0..8 "Poseidon"
                COLON_COLON@8..10 "::"
                CONST_ARG_LIST@10..13
                  LT@10..11 "<"
                  INTEGER@11..12 "4"
                  GT@12..13 ">"
        "#]]);
    }

    #[test]
    fn parse_type_locator() {
        check_type("credits.aleo/Token", expect![[r#"
            ROOT@0..18
              TYPE_LOCATOR@0..18
                IDENT@0..7 "credits"
                DOT@7..8 "."
                KW_ALEO@8..12 "aleo"
                SLASH@12..13 "/"
                IDENT@13..18 "Token"
        "#]]);
    }

    #[test]
    fn parse_type_program_id_without_type() {
        // Just program.aleo without /Type
        check_type("credits.aleo", expect![[r#"
            ROOT@0..12
              TYPE_LOCATOR@0..12
                IDENT@0..7 "credits"
                DOT@7..8 "."
                KW_ALEO@8..12 "aleo"
        "#]]);
    }

    // =========================================================================
    // Future Types
    // =========================================================================

    #[test]
    fn parse_type_final_simple() {
        check_type("Final", expect![[r#"
                ROOT@0..5
                  TYPE_FINAL@0..5
                    KW_FINAL_UPPER@0..5 "Final"
            "#]]);
    }

    #[test]
    fn parse_type_final_explicit() {
        check_type("Final<Fn(u32) -> field>", expect![[r#"
            ROOT@0..23
              TYPE_FINAL@0..23
                KW_FINAL_UPPER@0..5 "Final"
                LT@5..6 "<"
                KW_FN_UPPER@6..8 "Fn"
                L_PAREN@8..9 "("
                TYPE_PRIMITIVE@9..12
                  KW_U32@9..12 "u32"
                R_PAREN@12..13 ")"
                WHITESPACE@13..14 " "
                ARROW@14..16 "->"
                WHITESPACE@16..17 " "
                TYPE_PRIMITIVE@17..22
                  KW_FIELD@17..22 "field"
                GT@22..23 ">"
        "#]]);
    }

    #[test]
    fn parse_type_final_no_params() {
        check_type("Final<Fn() -> ()>", expect![[r#"
                ROOT@0..17
                  TYPE_FINAL@0..17
                    KW_FINAL_UPPER@0..5 "Final"
                    LT@5..6 "<"
                    KW_FN_UPPER@6..8 "Fn"
                    L_PAREN@8..9 "("
                    R_PAREN@9..10 ")"
                    WHITESPACE@10..11 " "
                    ARROW@11..13 "->"
                    WHITESPACE@13..14 " "
                    TYPE_TUPLE@14..16
                      L_PAREN@14..15 "("
                      R_PAREN@15..16 ")"
                    GT@16..17 ">"
            "#]]);
    }

    // =========================================================================
    // Mapping Types
    // =========================================================================

    #[test]
    fn parse_type_mapping() {
        check_type("mapping address => u64", expect![[r#"
            ROOT@0..22
              TYPE_MAPPING@0..22
                KW_MAPPING@0..7 "mapping"
                WHITESPACE@7..8 " "
                TYPE_PRIMITIVE@8..15
                  KW_ADDRESS@8..15 "address"
                WHITESPACE@15..16 " "
                FAT_ARROW@16..18 "=>"
                WHITESPACE@18..19 " "
                TYPE_PRIMITIVE@19..22
                  KW_U64@19..22 "u64"
        "#]]);
    }

    // =========================================================================
    // Const Generic Arguments (Use Sites) in Types
    // =========================================================================

    fn check_type_no_errors(input: &str) {
        let (tokens, _) = lex(input);
        let mut parser = Parser::new(input, &tokens);
        let root = parser.start();
        parser.parse_type();
        parser.skip_trivia();
        root.complete(&mut parser, ROOT);
        let parse: Parse = parser.finish(vec![]);
        if !parse.errors().is_empty() {
            for err in parse.errors() {
                eprintln!("error at {:?}: {}", err.range, err.message);
            }
            eprintln!("tree:\n{:#?}", parse.syntax());
            panic!("type parse had {} error(s)", parse.errors().len());
        }
    }

    #[test]
    fn parse_type_const_generic_expr_simple() {
        // Expression arg: integer literal
        check_type_no_errors("Foo::[3]");
    }

    #[test]
    fn parse_type_const_generic_expr_add() {
        // Expression arg: binary expression
        check_type_no_errors("Foo::[N + 1]");
    }

    #[test]
    fn parse_type_const_generic_expr_mul() {
        check_type_no_errors("Foo::[2 * N]");
    }

    #[test]
    fn parse_type_const_generic_multi_args() {
        check_type_no_errors("Matrix::[M, K, N]");
    }

    #[test]
    fn parse_type_const_generic_type_arg() {
        // Type arg: primitive type keyword (used by intrinsics)
        check_type_no_errors("Deserialize::[u32]");
    }

    #[test]
    fn parse_type_const_generic_array_type_arg() {
        // Array type arg: [u8; 4] inside ::[]
        check_type_no_errors("Deserialize::[[u8; 4]]");
    }

    #[test]
    fn parse_type_locator_const_generic() {
        // Locator + const generic args
        check_type_no_errors("child.aleo/Bar::[4]");
    }

    // =========================================================================
    // Nested Tuple Types (4a)
    // =========================================================================

    #[test]
    fn parse_type_tuple_nested() {
        check_type("((u32, u64), bool)", expect![[r#"
            ROOT@0..18
              TYPE_TUPLE@0..18
                L_PAREN@0..1 "("
                TYPE_TUPLE@1..11
                  L_PAREN@1..2 "("
                  TYPE_PRIMITIVE@2..5
                    KW_U32@2..5 "u32"
                  COMMA@5..6 ","
                  WHITESPACE@6..7 " "
                  TYPE_PRIMITIVE@7..10
                    KW_U64@7..10 "u64"
                  R_PAREN@10..11 ")"
                COMMA@11..12 ","
                WHITESPACE@12..13 " "
                TYPE_PRIMITIVE@13..17
                  KW_BOOL@13..17 "bool"
                R_PAREN@17..18 ")"
        "#]]);
    }

    // =========================================================================
    // Optional of Composite Types (4b)
    // =========================================================================

    #[test]
    fn parse_type_optional_array() {
        check_type_optional("[u32; 3]?", expect![[r#"
            ROOT@0..9
              TYPE_OPTIONAL@0..9
                TYPE_ARRAY@0..8
                  L_BRACKET@0..1 "["
                  TYPE_PRIMITIVE@1..4
                    KW_U32@1..4 "u32"
                  SEMICOLON@4..5 ";"
                  ARRAY_LENGTH@5..7
                    WHITESPACE@5..6 " "
                    LITERAL_INT@6..7
                      INTEGER@6..7 "3"
                  R_BRACKET@7..8 "]"
                QUESTION@8..9 "?"
        "#]]);
    }

    #[test]
    fn parse_type_optional_tuple() {
        check_type_optional("(u32, u64)?", expect![[r#"
            ROOT@0..11
              TYPE_OPTIONAL@0..11
                TYPE_TUPLE@0..10
                  L_PAREN@0..1 "("
                  TYPE_PRIMITIVE@1..4
                    KW_U32@1..4 "u32"
                  COMMA@4..5 ","
                  WHITESPACE@5..6 " "
                  TYPE_PRIMITIVE@6..9
                    KW_U64@6..9 "u64"
                  R_PAREN@9..10 ")"
                QUESTION@10..11 "?"
        "#]]);
    }

    // =========================================================================
    // Final with Multiple Parameters (4c)
    // =========================================================================

    #[test]
    fn parse_type_final_multi_param() {
        check_type("Final<Fn(u32, field, group) -> (u32, u64)>", expect![[r#"
            ROOT@0..42
              TYPE_FINAL@0..42
                KW_FINAL_UPPER@0..5 "Final"
                LT@5..6 "<"
                KW_FN_UPPER@6..8 "Fn"
                L_PAREN@8..9 "("
                TYPE_PRIMITIVE@9..12
                  KW_U32@9..12 "u32"
                COMMA@12..13 ","
                WHITESPACE@13..14 " "
                TYPE_PRIMITIVE@14..19
                  KW_FIELD@14..19 "field"
                COMMA@19..20 ","
                WHITESPACE@20..21 " "
                TYPE_PRIMITIVE@21..26
                  KW_GROUP@21..26 "group"
                R_PAREN@26..27 ")"
                WHITESPACE@27..28 " "
                ARROW@28..30 "->"
                WHITESPACE@30..31 " "
                TYPE_TUPLE@31..41
                  L_PAREN@31..32 "("
                  TYPE_PRIMITIVE@32..35
                    KW_U32@32..35 "u32"
                  COMMA@35..36 ","
                  WHITESPACE@36..37 " "
                  TYPE_PRIMITIVE@37..40
                    KW_U64@37..40 "u64"
                  R_PAREN@40..41 ")"
                GT@41..42 ">"
        "#]]);
    }

    // =========================================================================
    // Const Generic Expression Arguments (4d)
    // =========================================================================

    #[test]
    fn parse_type_const_generic_expr_sub() {
        check_type("Foo::[N - 1]", expect![[r#"
            ROOT@0..12
              TYPE_PATH@0..12
                IDENT@0..3 "Foo"
                COLON_COLON@3..5 "::"
                CONST_ARG_LIST@5..12
                  L_BRACKET@5..6 "["
                  BINARY_EXPR@6..11
                    PATH_EXPR@6..8
                      IDENT@6..7 "N"
                      WHITESPACE@7..8 " "
                    MINUS@8..9 "-"
                    WHITESPACE@9..10 " "
                    LITERAL_INT@10..11
                      INTEGER@10..11 "1"
                  R_BRACKET@11..12 "]"
        "#]]);
    }
}
