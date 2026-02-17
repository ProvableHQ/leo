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

//! Top-level item parsing for the Leo language.
//!
//! This module implements parsing for all Leo top-level declarations:
//! - Imports
//! - Program declarations
//! - Functions, transitions, and inline functions
//! - Structs and records
//! - Mappings and storage
//! - Global constants

use super::{CompletedMarker, EXPR_RECOVERY, ITEM_RECOVERY, PARAM_RECOVERY, Parser, TYPE_RECOVERY};
use crate::syntax_kind::SyntaxKind::{self, *};

impl Parser<'_, '_> {
    /// Recovery set for struct/record fields.
    const FIELD_RECOVERY: &'static [SyntaxKind] = &[COMMA, R_BRACE, KW_PUBLIC, KW_PRIVATE, KW_CONSTANT];
    /// Recovery set for return type parsing.
    const RETURN_TYPE_RECOVERY: &'static [SyntaxKind] = &[COMMA, R_PAREN, L_BRACE];

    /// Parse a complete file.
    ///
    /// A file may contain one or more program sections, each consisting of
    /// optional imports followed by a `program` declaration. Multi-program
    /// files appear in Leo test suites where programs are separated by
    /// `// --- Next Program --- //` comments (the comment itself is trivia
    /// and doesn't affect parsing).
    ///
    /// Module-level items (`const`, `struct`, `inline`) are also accepted
    /// at the top level to support multi-section test files that combine
    /// program declarations with module content separated by
    /// `// --- Next Module: path --- //` comments.
    pub fn parse_file_items(&mut self) {
        loop {
            self.skip_trivia();
            if self.at_eof() {
                break;
            }

            match self.current() {
                KW_IMPORT => {
                    self.parse_import();
                }
                KW_PROGRAM => {
                    self.parse_program_decl();
                }
                // Module-level items at top level (for module files and
                // multi-section test files with `// --- Next Module:` separators).
                KW_CONST | KW_STRUCT | KW_FN | KW_FINAL | AT => {
                    if self.parse_module_item().is_none() {
                        self.error_and_bump("expected module item");
                    }
                }
                _ => {
                    self.error_and_bump("expected `import`, `program`, or module item at top level");
                }
            }
        }
    }

    /// Parse module-level items.
    ///
    /// Module files contain only `const`, `struct`, and `inline` declarations
    /// (with optional annotations). No `import` or `program` blocks.
    pub fn parse_module_items(&mut self) {
        loop {
            self.skip_trivia();
            if self.at_eof() {
                break;
            }

            if self.parse_module_item().is_none() {
                self.error_and_bump("expected `const`, `struct`, or `inline` in module");
            }
        }
    }

    /// Parse a single module-level item: `const`, `struct`, or `inline fn`.
    fn parse_module_item(&mut self) -> Option<CompletedMarker> {
        // Handle annotations
        while self.at(AT) {
            self.parse_annotation();
        }

        match self.current() {
            KW_CONST => self.parse_global_const(),
            KW_STRUCT => self.parse_struct_def(),
            KW_FN | KW_FINAL => self.parse_function_or_constructor(),
            _ => None,
        }
    }

    /// Parse an import declaration: `import program.aleo;`
    fn parse_import(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any(); // import

        // Parse program ID: name.aleo
        self.parse_program_id();

        self.expect(SEMICOLON);
        Some(m.complete(self, IMPORT))
    }

    /// Parse a program declaration: `program name.aleo { ... }`
    fn parse_program_decl(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any(); // program

        // Parse program ID: name.aleo
        self.parse_program_id();

        self.expect(L_BRACE);

        // Parse program items
        while !self.at(R_BRACE) && !self.at_eof() {
            if self.parse_program_item().is_none() {
                // Error recovery
                self.error_recover("expected program item", ITEM_RECOVERY);
            }
        }

        self.expect(R_BRACE);
        Some(m.complete(self, PROGRAM_DECL))
    }

    /// Parse a program ID: `name.aleo`
    fn parse_program_id(&mut self) {
        self.skip_trivia();
        if self.at(IDENT) {
            self.bump_any(); // name
            self.expect(DOT);
            self.expect(KW_ALEO);
        } else {
            self.error("expected program name".to_string());
        }
    }

    /// Parse a single program item (struct, record, mapping, function, etc.)
    fn parse_program_item(&mut self) -> Option<CompletedMarker> {
        // Note: Don't skip trivia here - let item parsers include leading whitespace

        // Check for annotations first
        let has_annotations = self.at(AT);
        if has_annotations {
            // Parse annotations
            while self.at(AT) {
                self.parse_annotation();
            }
        }

        // Parse the item
        match self.current() {
            KW_STRUCT => self.parse_struct_def(),
            KW_RECORD => self.parse_record_def(),
            KW_MAPPING => self.parse_mapping_def(),
            KW_STORAGE => self.parse_storage_def(),
            KW_CONST => self.parse_global_const(),
            KW_FN | KW_FINAL => self.parse_function_or_constructor(),
            _ => {
                self.error(format!("expected program item, found {:?}", self.current()));
                None
            }
        }
    }

    /// Parse an annotation: `@program` or `@foo(args)`
    fn parse_annotation(&mut self) {
        let m = self.start();
        self.bump_any(); // @

        self.skip_trivia();
        // Accept identifiers and keywords as annotation names
        // (e.g. `@program`, `@test`, `@noupgrade`).
        if self.at(IDENT) || self.current().is_keyword() {
            self.bump_any();
        } else {
            self.error("expected annotation name".to_string());
        }

        // Optional parenthesized arguments.
        // Consume all tokens between parens, supporting arbitrary content
        // like `@checksum(mapping = "...", key = "...")`.
        if self.eat(L_PAREN) {
            let mut depth: u32 = 1;
            while !self.at_eof() && depth > 0 {
                if self.at(L_PAREN) {
                    depth += 1;
                } else if self.at(R_PAREN) {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                self.bump_any();
            }
            self.expect(R_PAREN);
        }

        m.complete(self, ANNOTATION);
    }

    /// Parse a struct definition: `struct Name { field: Type, ... }`
    fn parse_struct_def(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any(); // struct

        // Name
        self.skip_trivia();
        if self.at(IDENT) {
            self.bump_any();
        } else {
            self.error("expected struct name".to_string());
        }

        // Optional const generic parameters: ::[N: u32]
        if self.at(COLON_COLON) && self.nth(1) == L_BRACKET {
            self.bump_any(); // ::
            self.parse_const_param_list();
        }

        // Fields
        self.expect(L_BRACE);
        self.parse_struct_fields();
        self.expect(R_BRACE);

        Some(m.complete(self, STRUCT_DEF))
    }

    /// Parse a record definition: `record Name { field: Type, ... }`
    fn parse_record_def(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any(); // record

        // Name
        self.skip_trivia();
        if self.at(IDENT) {
            self.bump_any();
        } else {
            self.error("expected record name".to_string());
        }

        // Optional const generic parameters: ::[N: u32]
        if self.at(COLON_COLON) && self.nth(1) == L_BRACKET {
            self.bump_any(); // ::
            self.parse_const_param_list();
        }

        // Fields
        self.expect(L_BRACE);
        self.parse_struct_fields();
        self.expect(R_BRACE);

        Some(m.complete(self, RECORD_DEF))
    }

    /// Parse struct/record fields.
    fn parse_struct_fields(&mut self) {
        while !self.at(R_BRACE) && !self.at_eof() {
            // Start marker first so leading whitespace is inside the field node
            let m = self.start();

            // Check for visibility modifier
            let _ = self.eat(KW_PUBLIC) || self.eat(KW_PRIVATE) || self.eat(KW_CONSTANT);

            // Field name
            self.skip_trivia();
            if self.at(IDENT) {
                self.bump_any();
            } else {
                m.abandon(self);
                // Try to recover to next field or end of struct
                if !self.at(R_BRACE) {
                    self.error_recover("expected field name", Self::FIELD_RECOVERY);
                }
                // If we recovered to a comma, consume it and continue
                if self.eat(COMMA) {
                    continue;
                }
                break;
            }

            // Colon and type
            self.expect(COLON);
            if self.parse_type().is_none() {
                self.error_recover("expected type", Self::FIELD_RECOVERY);
            }

            m.complete(self, STRUCT_MEMBER);

            // Optional comma
            if !self.eat(COMMA) {
                break;
            }
        }
    }

    /// Parse a mapping definition: `mapping name: Key => Value;`
    fn parse_mapping_def(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any(); // mapping

        // Name
        self.skip_trivia();
        if self.at(IDENT) {
            self.bump_any();
        } else {
            self.error("expected mapping name".to_string());
        }

        // Key and value types
        self.expect(COLON);
        if self.parse_type().is_none() {
            self.error_recover("expected key type", TYPE_RECOVERY);
        }
        self.expect(FAT_ARROW);
        if self.parse_type().is_none() {
            self.error_recover("expected value type", TYPE_RECOVERY);
        }

        self.expect(SEMICOLON);
        Some(m.complete(self, MAPPING_DEF))
    }

    /// Parse a storage definition: `storage name: Type;`
    fn parse_storage_def(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any(); // storage

        // Name
        self.skip_trivia();
        if self.at(IDENT) {
            self.bump_any();
        } else {
            self.error("expected storage name".to_string());
        }

        // Type
        self.expect(COLON);
        if self.parse_type().is_none() {
            self.error_recover("expected type", TYPE_RECOVERY);
        }

        self.expect(SEMICOLON);
        Some(m.complete(self, STORAGE_DEF))
    }

    /// Parse a global constant: `const NAME: Type = expr;`
    fn parse_global_const(&mut self) -> Option<CompletedMarker> {
        let m = self.start();
        self.bump_any(); // const

        // Name
        self.skip_trivia();
        if self.at(IDENT) {
            self.bump_any();
        } else {
            self.error("expected constant name".to_string());
        }

        // Type
        self.expect(COLON);
        if self.parse_type().is_none() {
            self.error_recover("expected type", TYPE_RECOVERY);
        }

        // Value
        self.expect(EQ);
        if self.parse_expr().is_none() {
            self.error_recover("expected expression", EXPR_RECOVERY);
        }

        self.expect(SEMICOLON);
        Some(m.complete(self, GLOBAL_CONST))
    }

    /// Parse a function definition.
    /// Parse function, transition, inline, or async variants
    fn parse_function_or_constructor(&mut self) -> Option<CompletedMarker> {
        let m = self.start();

        // Optional async keyword
        self.eat(KW_FINAL);

        // Dispatch based on what follows
        match self.current() {
            KW_FN => {
                self.parse_function_body();
                Some(m.complete(self, FUNCTION_DEF))
            }
            KW_CONSTRUCTOR => {
                self.parse_constructor_body();
                Some(m.complete(self, CONSTRUCTOR_DEF))
            }
            _ => {
                self.error("expected function, transition, inline, or constructor".to_string());
                m.abandon(self);
                None
            }
        }
    }

    /// Parse function body (after async/function keyword marker started)
    fn parse_function_body(&mut self) {
        // Function keyword (function, transition, or inline)
        if !self.eat(KW_FN) {
            self.error("expected fn".to_string());
        }

        // Function name
        self.skip_trivia();
        if self.at(IDENT) {
            self.bump_any();
        } else {
            self.error("expected function name".to_string());
        }

        // Optional const generic parameters: ::[N: u32, M: u32]
        if self.at(COLON_COLON) && self.nth(1) == L_BRACKET {
            self.bump_any(); // ::
            self.parse_const_param_list();
        }

        // Parameters
        self.parse_param_list();

        // Return type: `-> [visibility] Type` or `-> (vis Type, vis Type)`
        if self.eat(ARROW) {
            self.parse_return_type();
        }

        // Body
        self.parse_block();
    }

    /// Parse a function return type.
    ///
    /// Handles both single return types with optional visibility modifiers
    /// (`-> public u32`) and tuple return types
    /// (`-> (public u32, private u32)`).
    fn parse_return_type(&mut self) {
        self.skip_trivia();
        if self.at(L_PAREN) {
            // Tuple return type: (vis Type, vis Type, ...)
            let m = self.start();
            self.bump_any(); // (
            if !self.at(R_PAREN) {
                // First output
                let _ = self.eat(KW_PUBLIC) || self.eat(KW_PRIVATE) || self.eat(KW_CONSTANT);
                if self.parse_type().is_none() {
                    self.error_recover("expected return type", Self::RETURN_TYPE_RECOVERY);
                }
                while self.eat(COMMA) {
                    if self.at(R_PAREN) {
                        break;
                    }
                    let _ = self.eat(KW_PUBLIC) || self.eat(KW_PRIVATE) || self.eat(KW_CONSTANT);
                    if self.parse_type().is_none() {
                        self.error_recover("expected return type", Self::RETURN_TYPE_RECOVERY);
                    }
                }
            }
            self.expect(R_PAREN);
            m.complete(self, RETURN_TYPE);
        } else {
            // Single return type with optional visibility
            let _ = self.eat(KW_PUBLIC) || self.eat(KW_PRIVATE) || self.eat(KW_CONSTANT);
            if self.parse_type().is_none() {
                self.error_recover("expected return type", Self::RETURN_TYPE_RECOVERY);
            }
        }
    }

    /// Parse constructor body: `constructor() { }`
    fn parse_constructor_body(&mut self) {
        self.bump_any(); // constructor

        // Parameters
        self.parse_param_list();

        // Body
        self.parse_block();
    }

    /// Parse a parameter list: `(a: Type, b: Type)`
    fn parse_param_list(&mut self) {
        let m = self.start();
        self.expect(L_PAREN);

        while !self.at(R_PAREN) && !self.at_eof() {
            self.parse_param();
            if !self.eat(COMMA) {
                break;
            }
        }

        self.expect(R_PAREN);
        m.complete(self, PARAM_LIST);
    }

    /// Parse a single parameter: `[visibility] name: Type`
    fn parse_param(&mut self) {
        let m = self.start();
        self.skip_trivia();

        // Optional visibility
        let _ = self.eat(KW_PUBLIC) || self.eat(KW_PRIVATE) || self.eat(KW_CONSTANT);

        // Name
        self.skip_trivia();
        if self.at(IDENT) {
            self.bump_any();
        } else {
            self.error("expected parameter name".to_string());
        }

        // Type
        self.expect(COLON);
        if self.parse_type().is_none() {
            self.error_recover("expected parameter type", PARAM_RECOVERY);
        }

        m.complete(self, PARAM);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexer::lex, parser::Parse};
    use expect_test::{Expect, expect};

    fn check_file(input: &str, expect: Expect) {
        let (tokens, _) = lex(input);
        let mut parser = Parser::new(input, &tokens);
        let root = parser.start();
        parser.parse_file_items();
        root.complete(&mut parser, ROOT);
        let parse: Parse = parser.finish();
        let output = format!("{:#?}", parse.syntax());
        expect.assert_eq(&output);
    }

    fn check_file_no_errors(input: &str) {
        let (tokens, _) = lex(input);
        let mut parser = Parser::new(input, &tokens);
        let root = parser.start();
        parser.parse_file_items();
        root.complete(&mut parser, ROOT);
        let parse: Parse = parser.finish();
        if !parse.errors().is_empty() {
            for err in parse.errors() {
                eprintln!("error at {:?}: {}", err.range, err.message);
            }
            eprintln!("tree:\n{:#?}", parse.syntax());
            panic!("parse had {} error(s)", parse.errors().len());
        }
    }

    // =========================================================================
    // Imports
    // =========================================================================

    #[test]
    fn parse_import() {
        check_file("import credits.aleo;", expect![[r#"
                ROOT@0..20
                  IMPORT@0..20
                    KW_IMPORT@0..6 "import"
                    WHITESPACE@6..7 " "
                    IDENT@7..14 "credits"
                    DOT@14..15 "."
                    KW_ALEO@15..19 "aleo"
                    SEMICOLON@19..20 ";"
            "#]]);
    }

    // =========================================================================
    // Program Declaration
    // =========================================================================

    #[test]
    fn parse_program_empty() {
        check_file("program test.aleo { }", expect![[r#"
                ROOT@0..21
                  PROGRAM_DECL@0..21
                    KW_PROGRAM@0..7 "program"
                    WHITESPACE@7..8 " "
                    IDENT@8..12 "test"
                    DOT@12..13 "."
                    KW_ALEO@13..17 "aleo"
                    WHITESPACE@17..18 " "
                    L_BRACE@18..19 "{"
                    WHITESPACE@19..20 " "
                    R_BRACE@20..21 "}"
            "#]]);
    }

    // =========================================================================
    // Structs
    // =========================================================================

    #[test]
    fn parse_struct() {
        check_file("program test.aleo { struct Point { x: u32, y: u32 } }", expect![[r#"
                ROOT@0..53
                  PROGRAM_DECL@0..53
                    KW_PROGRAM@0..7 "program"
                    WHITESPACE@7..8 " "
                    IDENT@8..12 "test"
                    DOT@12..13 "."
                    KW_ALEO@13..17 "aleo"
                    WHITESPACE@17..18 " "
                    L_BRACE@18..19 "{"
                    STRUCT_DEF@19..51
                      WHITESPACE@19..20 " "
                      KW_STRUCT@20..26 "struct"
                      WHITESPACE@26..27 " "
                      IDENT@27..32 "Point"
                      WHITESPACE@32..33 " "
                      L_BRACE@33..34 "{"
                      STRUCT_MEMBER@34..41
                        WHITESPACE@34..35 " "
                        IDENT@35..36 "x"
                        COLON@36..37 ":"
                        WHITESPACE@37..38 " "
                        TYPE_PATH@38..41
                          KW_U32@38..41 "u32"
                      COMMA@41..42 ","
                      STRUCT_MEMBER@42..49
                        WHITESPACE@42..43 " "
                        IDENT@43..44 "y"
                        COLON@44..45 ":"
                        WHITESPACE@45..46 " "
                        TYPE_PATH@46..49
                          KW_U32@46..49 "u32"
                      WHITESPACE@49..50 " "
                      R_BRACE@50..51 "}"
                    WHITESPACE@51..52 " "
                    R_BRACE@52..53 "}"
            "#]]);
    }

    // =========================================================================
    // Records
    // =========================================================================

    #[test]
    fn parse_record() {
        check_file("program test.aleo { record Token { owner: address, amount: u64 } }", expect![[r#"
                ROOT@0..66
                  PROGRAM_DECL@0..66
                    KW_PROGRAM@0..7 "program"
                    WHITESPACE@7..8 " "
                    IDENT@8..12 "test"
                    DOT@12..13 "."
                    KW_ALEO@13..17 "aleo"
                    WHITESPACE@17..18 " "
                    L_BRACE@18..19 "{"
                    RECORD_DEF@19..64
                      WHITESPACE@19..20 " "
                      KW_RECORD@20..26 "record"
                      WHITESPACE@26..27 " "
                      IDENT@27..32 "Token"
                      WHITESPACE@32..33 " "
                      L_BRACE@33..34 "{"
                      STRUCT_MEMBER@34..49
                        WHITESPACE@34..35 " "
                        IDENT@35..40 "owner"
                        COLON@40..41 ":"
                        WHITESPACE@41..42 " "
                        TYPE_PATH@42..49
                          KW_ADDRESS@42..49 "address"
                      COMMA@49..50 ","
                      STRUCT_MEMBER@50..62
                        WHITESPACE@50..51 " "
                        IDENT@51..57 "amount"
                        COLON@57..58 ":"
                        WHITESPACE@58..59 " "
                        TYPE_PATH@59..62
                          KW_U64@59..62 "u64"
                      WHITESPACE@62..63 " "
                      R_BRACE@63..64 "}"
                    WHITESPACE@64..65 " "
                    R_BRACE@65..66 "}"
            "#]]);
    }

    // =========================================================================
    // Mappings
    // =========================================================================

    #[test]
    fn parse_mapping() {
        check_file("program test.aleo { mapping balances: address => u64; }", expect![[r#"
                ROOT@0..55
                  PROGRAM_DECL@0..55
                    KW_PROGRAM@0..7 "program"
                    WHITESPACE@7..8 " "
                    IDENT@8..12 "test"
                    DOT@12..13 "."
                    KW_ALEO@13..17 "aleo"
                    WHITESPACE@17..18 " "
                    L_BRACE@18..19 "{"
                    MAPPING_DEF@19..53
                      WHITESPACE@19..20 " "
                      KW_MAPPING@20..27 "mapping"
                      WHITESPACE@27..28 " "
                      IDENT@28..36 "balances"
                      COLON@36..37 ":"
                      WHITESPACE@37..38 " "
                      TYPE_PATH@38..45
                        KW_ADDRESS@38..45 "address"
                      WHITESPACE@45..46 " "
                      FAT_ARROW@46..48 "=>"
                      WHITESPACE@48..49 " "
                      TYPE_PATH@49..52
                        KW_U64@49..52 "u64"
                      SEMICOLON@52..53 ";"
                    WHITESPACE@53..54 " "
                    R_BRACE@54..55 "}"
            "#]]);
    }

    // =========================================================================
    // Functions
    // =========================================================================

    #[test]
    fn parse_function() {
        check_file("program test.aleo { fn add(a: u32, b: u32) -> u32 { return a + b; } }", expect![[r#"
                ROOT@0..69
                  PROGRAM_DECL@0..69
                    KW_PROGRAM@0..7 "program"
                    WHITESPACE@7..8 " "
                    IDENT@8..12 "test"
                    DOT@12..13 "."
                    KW_ALEO@13..17 "aleo"
                    WHITESPACE@17..18 " "
                    L_BRACE@18..19 "{"
                    FUNCTION_DEF@19..67
                      WHITESPACE@19..20 " "
                      KW_FN@20..22 "fn"
                      WHITESPACE@22..23 " "
                      IDENT@23..26 "add"
                      PARAM_LIST@26..42
                        L_PAREN@26..27 "("
                        PARAM@27..33
                          IDENT@27..28 "a"
                          COLON@28..29 ":"
                          WHITESPACE@29..30 " "
                          TYPE_PATH@30..33
                            KW_U32@30..33 "u32"
                        COMMA@33..34 ","
                        PARAM@34..41
                          WHITESPACE@34..35 " "
                          IDENT@35..36 "b"
                          COLON@36..37 ":"
                          WHITESPACE@37..38 " "
                          TYPE_PATH@38..41
                            KW_U32@38..41 "u32"
                        R_PAREN@41..42 ")"
                      WHITESPACE@42..43 " "
                      ARROW@43..45 "->"
                      WHITESPACE@45..46 " "
                      TYPE_PATH@46..49
                        KW_U32@46..49 "u32"
                      BLOCK@49..67
                        WHITESPACE@49..50 " "
                        L_BRACE@50..51 "{"
                        WHITESPACE@51..52 " "
                        RETURN_STMT@52..65
                          KW_RETURN@52..58 "return"
                          WHITESPACE@58..59 " "
                          BINARY_EXPR@59..64
                            PATH_EXPR@59..61
                              IDENT@59..60 "a"
                              WHITESPACE@60..61 " "
                            PLUS@61..62 "+"
                            WHITESPACE@62..63 " "
                            PATH_EXPR@63..64
                              IDENT@63..64 "b"
                          SEMICOLON@64..65 ";"
                        WHITESPACE@65..66 " "
                        R_BRACE@66..67 "}"
                    WHITESPACE@67..68 " "
                    R_BRACE@68..69 "}"
            "#]]);
    }

    #[test]
    fn parse_final_function() {
        check_file("program test.aleo { } final fn foo() { assert_eq(1u64, 1u64); }", expect![[r#"
                ROOT@0..63
                  PROGRAM_DECL@0..21
                    KW_PROGRAM@0..7 "program"
                    WHITESPACE@7..8 " "
                    IDENT@8..12 "test"
                    DOT@12..13 "."
                    KW_ALEO@13..17 "aleo"
                    WHITESPACE@17..18 " "
                    L_BRACE@18..19 "{"
                    WHITESPACE@19..20 " "
                    R_BRACE@20..21 "}"
                  WHITESPACE@21..22 " "
                  FUNCTION_DEF@22..63
                    KW_FINAL@22..27 "final"
                    WHITESPACE@27..28 " "
                    KW_FN@28..30 "fn"
                    WHITESPACE@30..31 " "
                    IDENT@31..34 "foo"
                    PARAM_LIST@34..36
                      L_PAREN@34..35 "("
                      R_PAREN@35..36 ")"
                    WHITESPACE@36..37 " "
                    BLOCK@37..63
                      L_BRACE@37..38 "{"
                      WHITESPACE@38..39 " "
                      ASSERT_EQ_STMT@39..61
                        KW_ASSERT_EQ@39..48 "assert_eq"
                        L_PAREN@48..49 "("
                        LITERAL@49..53
                          INTEGER@49..53 "1u64"
                        COMMA@53..54 ","
                        WHITESPACE@54..55 " "
                        LITERAL@55..59
                          INTEGER@55..59 "1u64"
                        R_PAREN@59..60 ")"
                        SEMICOLON@60..61 ";"
                      WHITESPACE@61..62 " "
                      R_BRACE@62..63 "}"
            "#]]);
    }
    // =========================================================================
    // Const Generic Parameters (Declarations)
    // =========================================================================

    #[test]
    fn parse_function_const_generic_single() {
        check_file_no_errors("program test.aleo { fn foo::[N: u32]() {} }");
    }

    #[test]
    fn parse_function_const_generic_multi() {
        check_file_no_errors("program test.aleo { fn bar::[N: u32, M: u32](arr: u32) -> u32 { return 0u32; } }");
    }

    #[test]
    fn parse_function_const_generic_empty() {
        check_file_no_errors("program test.aleo { fn baz::[]() {} }");
    }

    #[test]
    fn parse_final_entry_const_generic() {
        check_file_no_errors("program test.aleo { fn t::[N: u32]() -> Final { return final {}; } }");
    }

    #[test]
    fn parse_struct_const_generic() {
        check_file_no_errors("program test.aleo { struct Foo::[N: u32] { arr: u32, } }");
    }

    #[test]
    fn parse_struct_const_generic_multi() {
        check_file_no_errors("program test.aleo { struct Matrix::[M: u32, N: u32] { data: u32, } }");
    }

    #[test]
    fn parse_record_const_generic() {
        // Syntactically valid, semantically rejected later.
        check_file_no_errors("program test.aleo { record Bar::[N: u32] { owner: address, } }");
    }

    #[test]
    fn parse_transition() {
        check_file("program test.aleo { fn main(public x: u32) { } }", expect![[r#"
                ROOT@0..48
                  PROGRAM_DECL@0..48
                    KW_PROGRAM@0..7 "program"
                    WHITESPACE@7..8 " "
                    IDENT@8..12 "test"
                    DOT@12..13 "."
                    KW_ALEO@13..17 "aleo"
                    WHITESPACE@17..18 " "
                    L_BRACE@18..19 "{"
                    FUNCTION_DEF@19..46
                      WHITESPACE@19..20 " "
                      KW_FN@20..22 "fn"
                      WHITESPACE@22..23 " "
                      IDENT@23..27 "main"
                      PARAM_LIST@27..42
                        L_PAREN@27..28 "("
                        PARAM@28..41
                          KW_PUBLIC@28..34 "public"
                          WHITESPACE@34..35 " "
                          IDENT@35..36 "x"
                          COLON@36..37 ":"
                          WHITESPACE@37..38 " "
                          TYPE_PATH@38..41
                            KW_U32@38..41 "u32"
                        R_PAREN@41..42 ")"
                      WHITESPACE@42..43 " "
                      BLOCK@43..46
                        L_BRACE@43..44 "{"
                        WHITESPACE@44..45 " "
                        R_BRACE@45..46 "}"
                    WHITESPACE@46..47 " "
                    R_BRACE@47..48 "}"
            "#]]);
    }
}
