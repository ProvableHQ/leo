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
    /// Tokens that can start a module-level item (for error recovery).
    const MODULE_ITEM_RECOVERY: &'static [SyntaxKind] = &[KW_CONST, KW_STRUCT, KW_FN, KW_FINAL, AT];
    /// Expected items within a `program { ... }` block.
    const PROGRAM_ITEM_EXPECTED: &'static [SyntaxKind] =
        &[R_BRACE, AT, KW_RECORD, KW_STRUCT, KW_FN, KW_FINAL, KW_CONST, KW_MAPPING, KW_STORAGE, KW_SCRIPT];
    /// Recovery set for return type parsing.
    const RETURN_TYPE_RECOVERY: &'static [SyntaxKind] = &[COMMA, R_PAREN, L_BRACE];
    /// Recovery set for struct/record name errors — skip to the next item or block boundary.
    const STRUCT_NAME_RECOVERY: &'static [SyntaxKind] = &[
        L_BRACE, R_BRACE, SEMICOLON, KW_IMPORT, KW_PROGRAM, KW_CONST, KW_STRUCT, KW_RECORD, KW_FN, KW_FINAL,
        KW_MAPPING, KW_STORAGE, KW_SCRIPT, AT,
    ];

    /// Consume an optional visibility modifier keyword (`public`, `private`, or `constant`).
    /// Returns the keyword kind that was consumed, or `None`.
    fn eat_visibility(&mut self) -> Option<SyntaxKind> {
        if self.eat(KW_PUBLIC) {
            Some(KW_PUBLIC)
        } else if self.eat(KW_PRIVATE) {
            Some(KW_PRIVATE)
        } else if self.eat(KW_CONSTANT) {
            Some(KW_CONSTANT)
        } else {
            None
        }
    }

    /// Parse a complete file.
    ///
    /// A file may contain one or more program sections, each consisting of
    /// optional imports followed by a `program` declaration. Multi-program
    /// files appear in Leo test suites where programs are separated by
    /// `// --- Next Program --- //` comments (the comment itself is trivia
    /// and doesn't affect parsing).
    ///
    /// Module-level items (`const`, `struct`, `fn`) are also accepted
    /// at the top level to support multi-section test files that combine
    /// program declarations with module content separated by
    /// `// --- Next Module: path --- //` comments.
    pub fn parse_file_items(&mut self) {
        loop {
            self.skip_trivia();
            if self.at_eof() {
                break;
            }

            // Clear error state so each top-level item gets fresh error reporting.
            self.erroring = false;

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
                    self.error("expected `import`, `program`, or module item at top level");
                    self.recover(&[KW_IMPORT, KW_PROGRAM, KW_CONST, KW_STRUCT, KW_FN, KW_FINAL, AT]);
                }
            }
        }
    }

    /// Parse module-level items.
    ///
    /// Module files contain only `const`, `struct`, and `fn` declarations
    /// (with optional annotations). No `import` or `program` blocks.
    pub fn parse_module_items(&mut self) {
        loop {
            self.skip_trivia();
            if self.at_eof() {
                break;
            }

            // Clear error state so each module item gets fresh error reporting.
            self.erroring = false;

            if self.parse_module_item().is_none() {
                self.error("expected `const`, `struct`, or `fn` in module");
                self.recover(Self::MODULE_ITEM_RECOVERY);
            }
        }
    }

    /// Parse a single module-level item: `const`, `struct`, or `fn`.
    ///
    /// Annotations are handled inside each item parser.
    fn parse_module_item(&mut self) -> Option<CompletedMarker> {
        match self.current() {
            KW_CONST => self.parse_global_const(),
            KW_STRUCT => self.parse_composite_def(STRUCT_DEF),
            AT | KW_FN | KW_FINAL => self.parse_function_or_constructor(),
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
            // Clear error state so each item gets fresh error reporting.
            self.erroring = false;
            if self.parse_program_item().is_none() {
                // Error was already reported by parse_program_item; just recover.
                self.recover(ITEM_RECOVERY);
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
            if !self.eat(KW_ALEO) {
                if self.at(IDENT) {
                    // Consume the invalid network identifier — the AST converter
                    // will emit a specific `invalid_network` error (EPAR0370028).
                    self.bump_any();
                } else {
                    self.error("expected 'aleo'");
                }
            }
        } else {
            self.error("expected program name");
        }
    }

    /// Parse a single program item (struct, record, mapping, function, etc.)
    ///
    /// Annotations (`@foo`) are parsed as the first step and become children
    /// of the resulting item node rather than siblings.
    fn parse_program_item(&mut self) -> Option<CompletedMarker> {
        // For annotated items, dispatch to function_or_constructor since
        // annotations are only valid on functions/transitions in program blocks.
        // The function parser handles annotations internally.
        match self.current() {
            AT => self.parse_function_or_constructor(),
            KW_STRUCT => self.parse_composite_def(STRUCT_DEF),
            KW_RECORD => self.parse_composite_def(RECORD_DEF),
            KW_MAPPING => self.parse_mapping_def(),
            KW_STORAGE => self.parse_storage_def(),
            KW_CONST => self.parse_global_const(),
            KW_FN | KW_FINAL | KW_SCRIPT | KW_CONSTRUCTOR => self.parse_function_or_constructor(),
            _ => {
                let expected: Vec<&str> = Self::PROGRAM_ITEM_EXPECTED.iter().map(|k| k.user_friendly_name()).collect();
                self.error_unexpected(self.current(), &expected);
                None
            }
        }
    }

    /// Parse an annotation: `@program` or `@foo(args)`
    fn parse_annotation(&mut self) {
        let m = self.start();
        self.bump_any(); // @

        // Reject space between `@` and the annotation name (e.g. `@ test`).
        if self.current_including_trivia().is_trivia() {
            self.error_unexpected(self.current(), &["an identifier", "'program'"]);
            self.skip_trivia();
            // Still consume the name for recovery.
            if self.at(IDENT) || self.current().is_keyword() {
                self.bump_any();
            }
        } else if self.at(IDENT) || self.current().is_keyword() {
            // Accept identifiers and keywords as annotation names
            // (e.g. `@program`, `@test`, `@noupgrade`).
            self.bump_any();
        } else {
            self.error("expected annotation name");
        }

        // Optional parenthesized arguments: `(key = "value", ...)`.
        // Annotation members must be `identifier = "string"` separated by commas.
        if self.eat(L_PAREN) {
            if !self.at(R_PAREN) {
                self.parse_annotation_member();
                while self.eat(COMMA) {
                    if self.at(R_PAREN) {
                        break;
                    }
                    // Clear error state so each member gets fresh error reporting.
                    self.erroring = false;
                    self.parse_annotation_member();
                }
            }
            // Reject trailing content before `)` (e.g. `oops` in `@foo(k = "v" oops)`).
            if !self.at(R_PAREN) && !self.at_eof() {
                self.error_unexpected(self.current(), &["')'", "','"]);
                // Recovery: skip to closing paren.
                while !self.at(R_PAREN) && !self.at_eof() {
                    self.bump_any();
                }
            }
            self.expect(R_PAREN);
        }

        m.complete(self, ANNOTATION);
    }

    /// Parse a single annotation member: `key = "value"`.
    fn parse_annotation_member(&mut self) {
        let m = self.start();
        // Key must be an identifier, `address`, or `mapping`.
        if self.at(IDENT) || self.at(KW_ADDRESS) || self.at(KW_MAPPING) {
            self.bump_any();
        } else {
            self.error_unexpected(self.current(), &["an identifier", "')'", "'address", "'mapping'"]);
            // Recovery: skip to `,` or `)`.
            while !self.at(COMMA) && !self.at(R_PAREN) && !self.at_eof() {
                self.bump_any();
            }
            m.abandon(self);
            return;
        }
        self.expect(EQ);
        if self.at(STRING) {
            self.bump_any();
        } else {
            self.error("expected string literal for annotation value");
        }
        m.complete(self, ANNOTATION_PAIR);
    }

    /// Parse a struct or record definition: `struct|record Name { field: Type, ... }`
    fn parse_composite_def(&mut self, kind: SyntaxKind) -> Option<CompletedMarker> {
        let m = self.start();
        let label = if kind == STRUCT_DEF { "struct" } else { "record" };
        self.bump_any(); // struct | record

        // Name
        self.skip_trivia();
        if self.at(IDENT) {
            self.bump_any();
        } else {
            self.error(format!("expected {label} name"));
            self.recover(Self::STRUCT_NAME_RECOVERY);
            return Some(m.complete(self, ERROR));
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

        Some(m.complete(self, kind))
    }

    /// Parse struct/record fields.
    fn parse_struct_fields(&mut self) {
        while !self.at(R_BRACE) && !self.at_eof() {
            // Skip trivia before starting marker so member span starts at identifier
            self.skip_trivia();
            let m = self.start();

            // Check for visibility modifier
            let vis = self.eat_visibility();

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

            let kind = match vis {
                Some(KW_PUBLIC) => STRUCT_MEMBER_PUBLIC,
                Some(KW_PRIVATE) => STRUCT_MEMBER_PRIVATE,
                Some(KW_CONSTANT) => STRUCT_MEMBER_CONSTANT,
                _ => STRUCT_MEMBER,
            };
            m.complete(self, kind);

            // Comma or end of fields.
            if !self.eat(COMMA) {
                // If we're at `}` or EOF, the field list is done.
                // Otherwise, there's a missing comma — report it and continue
                // so we can parse more fields rather than cascading.
                if !self.at(R_BRACE) && !self.at_eof() {
                    self.error("expected ','");
                    // Clear erroring so the next field can report its own errors.
                    self.erroring = false;
                }
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
            self.error("expected mapping name");
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
            self.error("expected storage name");
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
            self.error("expected constant name");
            self.recover(&[SEMICOLON]);
            self.eat(SEMICOLON);
            return Some(m.complete(self, GLOBAL_CONST));
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
    /// Parse fn, final fn, script, or constructor variants.
    /// Annotations are parsed as children of the function node.
    fn parse_function_or_constructor(&mut self) -> Option<CompletedMarker> {
        let m = self.start();

        // Parse leading annotations as children of this function node.
        while self.at(AT) {
            self.parse_annotation();
        }

        // Optional final keyword
        let ate_final = self.eat(KW_FINAL);

        // Dispatch based on what follows
        match self.current() {
            KW_FN => {
                self.parse_function_body();
                let kind = if ate_final { FINAL_FN_DEF } else { FUNCTION_DEF };
                Some(m.complete(self, kind))
            }
            KW_SCRIPT => {
                self.parse_function_body();
                Some(m.complete(self, SCRIPT_DEF))
            }
            KW_CONSTRUCTOR => {
                self.parse_constructor_body();
                Some(m.complete(self, CONSTRUCTOR_DEF))
            }
            _ => {
                self.error("expected 'fn', 'script', or 'constructor'");
                m.abandon(self);
                None
            }
        }
    }

    /// Parse function body (after final/fn keyword marker started)
    fn parse_function_body(&mut self) {
        // Function keyword (fn or script)
        if !self.eat(KW_FN) && !self.eat(KW_SCRIPT) {
            self.error("expected 'fn' or 'script'");
        }

        // Function name
        self.skip_trivia();
        if self.at(IDENT) {
            self.bump_any();
        } else {
            self.error("expected function name");
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
                self.eat_visibility();
                if self.parse_type().is_none() {
                    self.error_recover("expected return type", Self::RETURN_TYPE_RECOVERY);
                }
                while self.eat(COMMA) {
                    if self.at(R_PAREN) {
                        break;
                    }
                    self.eat_visibility();
                    if self.parse_type().is_none() {
                        self.error_recover("expected return type", Self::RETURN_TYPE_RECOVERY);
                    }
                }
            }
            self.expect(R_PAREN);
            m.complete(self, RETURN_TYPE);
        } else {
            // Single return type with optional visibility
            self.eat_visibility();
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
            // Clear error state so each parameter gets fresh error reporting.
            self.erroring = false;
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
        let vis = self.eat_visibility();

        // Name
        self.skip_trivia();
        if self.at(IDENT) {
            self.bump_any();
        } else {
            self.error("expected parameter name");
        }

        // Type
        self.expect(COLON);
        if self.parse_type().is_none() {
            self.error_recover("expected parameter type", PARAM_RECOVERY);
        }

        let kind = match vis {
            Some(KW_PUBLIC) => PARAM_PUBLIC,
            Some(KW_PRIVATE) => PARAM_PRIVATE,
            Some(KW_CONSTANT) => PARAM_CONSTANT,
            _ => PARAM,
        };
        m.complete(self, kind);
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
        let parse: Parse = parser.finish(vec![]);
        let output = format!("{:#?}", parse.syntax());
        expect.assert_eq(&output);
    }

    fn check_file_no_errors(input: &str) {
        let (tokens, _) = lex(input);
        let mut parser = Parser::new(input, &tokens);
        let root = parser.start();
        parser.parse_file_items();
        root.complete(&mut parser, ROOT);
        let parse: Parse = parser.finish(vec![]);
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
                  WHITESPACE@34..35 " "
                  STRUCT_MEMBER@35..41
                    IDENT@35..36 "x"
                    COLON@36..37 ":"
                    WHITESPACE@37..38 " "
                    TYPE_PRIMITIVE@38..41
                      KW_U32@38..41 "u32"
                  COMMA@41..42 ","
                  WHITESPACE@42..43 " "
                  STRUCT_MEMBER@43..49
                    IDENT@43..44 "y"
                    COLON@44..45 ":"
                    WHITESPACE@45..46 " "
                    TYPE_PRIMITIVE@46..49
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
                  WHITESPACE@34..35 " "
                  STRUCT_MEMBER@35..49
                    IDENT@35..40 "owner"
                    COLON@40..41 ":"
                    WHITESPACE@41..42 " "
                    TYPE_PRIMITIVE@42..49
                      KW_ADDRESS@42..49 "address"
                  COMMA@49..50 ","
                  WHITESPACE@50..51 " "
                  STRUCT_MEMBER@51..62
                    IDENT@51..57 "amount"
                    COLON@57..58 ":"
                    WHITESPACE@58..59 " "
                    TYPE_PRIMITIVE@59..62
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
                  TYPE_PRIMITIVE@38..45
                    KW_ADDRESS@38..45 "address"
                  WHITESPACE@45..46 " "
                  FAT_ARROW@46..48 "=>"
                  WHITESPACE@48..49 " "
                  TYPE_PRIMITIVE@49..52
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
                      TYPE_PRIMITIVE@30..33
                        KW_U32@30..33 "u32"
                    COMMA@33..34 ","
                    PARAM@34..41
                      WHITESPACE@34..35 " "
                      IDENT@35..36 "b"
                      COLON@36..37 ":"
                      WHITESPACE@37..38 " "
                      TYPE_PRIMITIVE@38..41
                        KW_U32@38..41 "u32"
                    R_PAREN@41..42 ")"
                  WHITESPACE@42..43 " "
                  ARROW@43..45 "->"
                  WHITESPACE@45..46 " "
                  TYPE_PRIMITIVE@46..49
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
              FINAL_FN_DEF@22..63
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
                    LITERAL_INT@49..53
                      INTEGER@49..53 "1u64"
                    COMMA@53..54 ","
                    WHITESPACE@54..55 " "
                    LITERAL_INT@55..59
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
                    PARAM_PUBLIC@28..41
                      KW_PUBLIC@28..34 "public"
                      WHITESPACE@34..35 " "
                      IDENT@35..36 "x"
                      COLON@36..37 ":"
                      WHITESPACE@37..38 " "
                      TYPE_PRIMITIVE@38..41
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

    // =========================================================================
    // Record with Visibility Modifiers (3a)
    // =========================================================================

    #[test]
    fn parse_record_visibility() {
        check_file("program test.aleo { record Token { public owner: address, private amount: u64, } }", expect![[
            r#"
            ROOT@0..82
              PROGRAM_DECL@0..82
                KW_PROGRAM@0..7 "program"
                WHITESPACE@7..8 " "
                IDENT@8..12 "test"
                DOT@12..13 "."
                KW_ALEO@13..17 "aleo"
                WHITESPACE@17..18 " "
                L_BRACE@18..19 "{"
                RECORD_DEF@19..80
                  WHITESPACE@19..20 " "
                  KW_RECORD@20..26 "record"
                  WHITESPACE@26..27 " "
                  IDENT@27..32 "Token"
                  WHITESPACE@32..33 " "
                  L_BRACE@33..34 "{"
                  WHITESPACE@34..35 " "
                  STRUCT_MEMBER_PUBLIC@35..56
                    KW_PUBLIC@35..41 "public"
                    WHITESPACE@41..42 " "
                    IDENT@42..47 "owner"
                    COLON@47..48 ":"
                    WHITESPACE@48..49 " "
                    TYPE_PRIMITIVE@49..56
                      KW_ADDRESS@49..56 "address"
                  COMMA@56..57 ","
                  WHITESPACE@57..58 " "
                  STRUCT_MEMBER_PRIVATE@58..77
                    KW_PRIVATE@58..65 "private"
                    WHITESPACE@65..66 " "
                    IDENT@66..72 "amount"
                    COLON@72..73 ":"
                    WHITESPACE@73..74 " "
                    TYPE_PRIMITIVE@74..77
                      KW_U64@74..77 "u64"
                  COMMA@77..78 ","
                  WHITESPACE@78..79 " "
                  R_BRACE@79..80 "}"
                WHITESPACE@80..81 " "
                R_BRACE@81..82 "}"
        "#
        ]]);
    }

    // =========================================================================
    // Final Fn (3b)
    // =========================================================================

    #[test]
    fn parse_final_fn_with_return() {
        check_file("program test.aleo { final fn main(public x: u32) -> Final { return final {}; } }", expect![[r#"
            ROOT@0..80
              PROGRAM_DECL@0..80
                KW_PROGRAM@0..7 "program"
                WHITESPACE@7..8 " "
                IDENT@8..12 "test"
                DOT@12..13 "."
                KW_ALEO@13..17 "aleo"
                WHITESPACE@17..18 " "
                L_BRACE@18..19 "{"
                FINAL_FN_DEF@19..78
                  WHITESPACE@19..20 " "
                  KW_FINAL@20..25 "final"
                  WHITESPACE@25..26 " "
                  KW_FN@26..28 "fn"
                  WHITESPACE@28..29 " "
                  IDENT@29..33 "main"
                  PARAM_LIST@33..48
                    L_PAREN@33..34 "("
                    PARAM_PUBLIC@34..47
                      KW_PUBLIC@34..40 "public"
                      WHITESPACE@40..41 " "
                      IDENT@41..42 "x"
                      COLON@42..43 ":"
                      WHITESPACE@43..44 " "
                      TYPE_PRIMITIVE@44..47
                        KW_U32@44..47 "u32"
                    R_PAREN@47..48 ")"
                  WHITESPACE@48..49 " "
                  ARROW@49..51 "->"
                  WHITESPACE@51..52 " "
                  TYPE_FINAL@52..58
                    KW_FINAL_UPPER@52..57 "Final"
                    WHITESPACE@57..58 " "
                  BLOCK@58..78
                    L_BRACE@58..59 "{"
                    WHITESPACE@59..60 " "
                    RETURN_STMT@60..76
                      KW_RETURN@60..66 "return"
                      WHITESPACE@66..67 " "
                      FINAL_EXPR@67..75
                        KW_FINAL@67..72 "final"
                        WHITESPACE@72..73 " "
                        BLOCK@73..75
                          L_BRACE@73..74 "{"
                          R_BRACE@74..75 "}"
                      SEMICOLON@75..76 ";"
                    WHITESPACE@76..77 " "
                    R_BRACE@77..78 "}"
                WHITESPACE@78..79 " "
                R_BRACE@79..80 "}"
        "#]]);
    }

    // =========================================================================
    // Return Types with Visibility (3c)
    // =========================================================================

    #[test]
    fn parse_function_return_tuple() {
        check_file(
            "program test.aleo { fn foo() -> (public u32, private field) { return (1u32, 2field); } }",
            expect![[r#"
                ROOT@0..88
                  PROGRAM_DECL@0..88
                    KW_PROGRAM@0..7 "program"
                    WHITESPACE@7..8 " "
                    IDENT@8..12 "test"
                    DOT@12..13 "."
                    KW_ALEO@13..17 "aleo"
                    WHITESPACE@17..18 " "
                    L_BRACE@18..19 "{"
                    FUNCTION_DEF@19..86
                      WHITESPACE@19..20 " "
                      KW_FN@20..22 "fn"
                      WHITESPACE@22..23 " "
                      IDENT@23..26 "foo"
                      PARAM_LIST@26..28
                        L_PAREN@26..27 "("
                        R_PAREN@27..28 ")"
                      WHITESPACE@28..29 " "
                      ARROW@29..31 "->"
                      WHITESPACE@31..32 " "
                      RETURN_TYPE@32..59
                        L_PAREN@32..33 "("
                        KW_PUBLIC@33..39 "public"
                        WHITESPACE@39..40 " "
                        TYPE_PRIMITIVE@40..43
                          KW_U32@40..43 "u32"
                        COMMA@43..44 ","
                        WHITESPACE@44..45 " "
                        KW_PRIVATE@45..52 "private"
                        WHITESPACE@52..53 " "
                        TYPE_PRIMITIVE@53..58
                          KW_FIELD@53..58 "field"
                        R_PAREN@58..59 ")"
                      BLOCK@59..86
                        WHITESPACE@59..60 " "
                        L_BRACE@60..61 "{"
                        WHITESPACE@61..62 " "
                        RETURN_STMT@62..84
                          KW_RETURN@62..68 "return"
                          WHITESPACE@68..69 " "
                          TUPLE_EXPR@69..83
                            L_PAREN@69..70 "("
                            LITERAL_INT@70..74
                              INTEGER@70..74 "1u32"
                            COMMA@74..75 ","
                            WHITESPACE@75..76 " "
                            LITERAL_FIELD@76..82
                              INTEGER@76..82 "2field"
                            R_PAREN@82..83 ")"
                          SEMICOLON@83..84 ";"
                        WHITESPACE@84..85 " "
                        R_BRACE@85..86 "}"
                    WHITESPACE@86..87 " "
                    R_BRACE@87..88 "}"
            "#]],
        );
    }

    // =========================================================================
    // Multiple Annotations (3d)
    // =========================================================================

    #[test]
    fn parse_function_multi_annotation() {
        check_file("program test.aleo { @test @foo(k = \"v\") fn bar() { } }", expect![[r#"
            ROOT@0..54
              PROGRAM_DECL@0..54
                KW_PROGRAM@0..7 "program"
                WHITESPACE@7..8 " "
                IDENT@8..12 "test"
                DOT@12..13 "."
                KW_ALEO@13..17 "aleo"
                WHITESPACE@17..18 " "
                L_BRACE@18..19 "{"
                FUNCTION_DEF@19..52
                  ANNOTATION@19..26
                    WHITESPACE@19..20 " "
                    AT@20..21 "@"
                    IDENT@21..25 "test"
                    WHITESPACE@25..26 " "
                  ANNOTATION@26..39
                    AT@26..27 "@"
                    IDENT@27..30 "foo"
                    L_PAREN@30..31 "("
                    ANNOTATION_PAIR@31..38
                      IDENT@31..32 "k"
                      WHITESPACE@32..33 " "
                      EQ@33..34 "="
                      WHITESPACE@34..35 " "
                      STRING@35..38 "\"v\""
                    R_PAREN@38..39 ")"
                  WHITESPACE@39..40 " "
                  KW_FN@40..42 "fn"
                  WHITESPACE@42..43 " "
                  IDENT@43..46 "bar"
                  PARAM_LIST@46..48
                    L_PAREN@46..47 "("
                    R_PAREN@47..48 ")"
                  WHITESPACE@48..49 " "
                  BLOCK@49..52
                    L_BRACE@49..50 "{"
                    WHITESPACE@50..51 " "
                    R_BRACE@51..52 "}"
                WHITESPACE@52..53 " "
                R_BRACE@53..54 "}"
        "#]]);
    }

    // =========================================================================
    // Storage Declaration (3e)
    // =========================================================================

    #[test]
    fn parse_storage() {
        check_file("program test.aleo { storage state: u64; }", expect![[r#"
            ROOT@0..41
              PROGRAM_DECL@0..41
                KW_PROGRAM@0..7 "program"
                WHITESPACE@7..8 " "
                IDENT@8..12 "test"
                DOT@12..13 "."
                KW_ALEO@13..17 "aleo"
                WHITESPACE@17..18 " "
                L_BRACE@18..19 "{"
                STORAGE_DEF@19..39
                  WHITESPACE@19..20 " "
                  KW_STORAGE@20..27 "storage"
                  WHITESPACE@27..28 " "
                  IDENT@28..33 "state"
                  COLON@33..34 ":"
                  WHITESPACE@34..35 " "
                  TYPE_PRIMITIVE@35..38
                    KW_U64@35..38 "u64"
                  SEMICOLON@38..39 ";"
                WHITESPACE@39..40 " "
                R_BRACE@40..41 "}"
        "#]]);
    }

    // =========================================================================
    // Script Variant (3f)
    // =========================================================================

    #[test]
    fn parse_script() {
        check_file("program test.aleo { script main() { } }", expect![[r#"
            ROOT@0..39
              PROGRAM_DECL@0..39
                KW_PROGRAM@0..7 "program"
                WHITESPACE@7..8 " "
                IDENT@8..12 "test"
                DOT@12..13 "."
                KW_ALEO@13..17 "aleo"
                WHITESPACE@17..18 " "
                L_BRACE@18..19 "{"
                SCRIPT_DEF@19..37
                  WHITESPACE@19..20 " "
                  KW_SCRIPT@20..26 "script"
                  WHITESPACE@26..27 " "
                  IDENT@27..31 "main"
                  PARAM_LIST@31..33
                    L_PAREN@31..32 "("
                    R_PAREN@32..33 ")"
                  WHITESPACE@33..34 " "
                  BLOCK@34..37
                    L_BRACE@34..35 "{"
                    WHITESPACE@35..36 " "
                    R_BRACE@36..37 "}"
                WHITESPACE@37..38 " "
                R_BRACE@38..39 "}"
        "#]]);
    }

    // =========================================================================
    // Program with Nested Items (3g)
    // =========================================================================

    #[test]
    fn parse_program_with_items() {
        check_file("program test.aleo { struct Foo { x: u32, } fn bar() -> u32 { return 1u32; } }", expect![[r#"
            ROOT@0..77
              PROGRAM_DECL@0..77
                KW_PROGRAM@0..7 "program"
                WHITESPACE@7..8 " "
                IDENT@8..12 "test"
                DOT@12..13 "."
                KW_ALEO@13..17 "aleo"
                WHITESPACE@17..18 " "
                L_BRACE@18..19 "{"
                STRUCT_DEF@19..42
                  WHITESPACE@19..20 " "
                  KW_STRUCT@20..26 "struct"
                  WHITESPACE@26..27 " "
                  IDENT@27..30 "Foo"
                  WHITESPACE@30..31 " "
                  L_BRACE@31..32 "{"
                  WHITESPACE@32..33 " "
                  STRUCT_MEMBER@33..39
                    IDENT@33..34 "x"
                    COLON@34..35 ":"
                    WHITESPACE@35..36 " "
                    TYPE_PRIMITIVE@36..39
                      KW_U32@36..39 "u32"
                  COMMA@39..40 ","
                  WHITESPACE@40..41 " "
                  R_BRACE@41..42 "}"
                FUNCTION_DEF@42..75
                  WHITESPACE@42..43 " "
                  KW_FN@43..45 "fn"
                  WHITESPACE@45..46 " "
                  IDENT@46..49 "bar"
                  PARAM_LIST@49..51
                    L_PAREN@49..50 "("
                    R_PAREN@50..51 ")"
                  WHITESPACE@51..52 " "
                  ARROW@52..54 "->"
                  WHITESPACE@54..55 " "
                  TYPE_PRIMITIVE@55..58
                    KW_U32@55..58 "u32"
                  BLOCK@58..75
                    WHITESPACE@58..59 " "
                    L_BRACE@59..60 "{"
                    WHITESPACE@60..61 " "
                    RETURN_STMT@61..73
                      KW_RETURN@61..67 "return"
                      WHITESPACE@67..68 " "
                      LITERAL_INT@68..72
                        INTEGER@68..72 "1u32"
                      SEMICOLON@72..73 ";"
                    WHITESPACE@73..74 " "
                    R_BRACE@74..75 "}"
                WHITESPACE@75..76 " "
                R_BRACE@76..77 "}"
        "#]]);
    }

    // =========================================================================
    // Import Error: Invalid Network (3h)
    // =========================================================================

    #[test]
    fn parse_import_invalid_network() {
        // `foo.bar` — the parser consumes the invalid network identifier for
        // recovery; the AST converter emits the semantic error. The CST parse
        // itself should succeed without errors (the parser is lenient here).
        check_file("import foo.bar;", expect![[r#"
            ROOT@0..15
              IMPORT@0..15
                KW_IMPORT@0..6 "import"
                WHITESPACE@6..7 " "
                IDENT@7..10 "foo"
                DOT@10..11 "."
                IDENT@11..14 "bar"
                SEMICOLON@14..15 ";"
        "#]]);
    }

    // =========================================================================
    // Annotation Error Cases (3i)
    // =========================================================================

    #[test]
    fn parse_annotation_space_after_at() {
        // Space between @ and name should produce an error.
        let (tokens, _) = lex("program test.aleo { @ test fn foo() { } }");
        let mut parser = Parser::new("program test.aleo { @ test fn foo() { } }", &tokens);
        let root = parser.start();
        parser.parse_file_items();
        root.complete(&mut parser, ROOT);
        let parse: Parse = parser.finish(vec![]);
        assert!(!parse.errors().is_empty(), "expected error for space after @, got none");
    }
}
