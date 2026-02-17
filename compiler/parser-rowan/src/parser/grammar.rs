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

//! Grammar entry points for the Leo parser.
//!
//! This module provides the public parsing functions that dispatch to
//! the appropriate grammar rules.

use super::{Parse, Parser};
use crate::{lexer::lex, syntax_kind::SyntaxKind::*};

/// Parse a complete Leo source file.
///
/// This handles imports followed by a program declaration.
pub fn parse_file(source: &str) -> Parse {
    let (tokens, _lex_errors) = lex(source);
    let mut parser = Parser::new(source, &tokens);

    let root = parser.start();
    parser.parse_file_items();
    root.complete(&mut parser, ROOT);

    parser.finish()
}

/// Parse a single expression.
pub fn parse_expression_entry(source: &str) -> Parse {
    let (tokens, _lex_errors) = lex(source);
    let mut parser = Parser::new(source, &tokens);

    let root = parser.start();
    parser.parse_expr();
    root.complete(&mut parser, ROOT);

    parser.finish()
}

/// Parse module contents (const, struct, inline declarations only).
pub fn parse_module_entry(source: &str) -> Parse {
    let (tokens, _lex_errors) = lex(source);
    let mut parser = Parser::new(source, &tokens);

    let root = parser.start();
    parser.parse_module_items();
    root.complete(&mut parser, ROOT);

    parser.finish()
}

/// Parse a single statement.
pub fn parse_statement_entry(source: &str) -> Parse {
    let (tokens, _lex_errors) = lex(source);
    let mut parser = Parser::new(source, &tokens);

    let root = parser.start();
    parser.parse_stmt();
    root.complete(&mut parser, ROOT);

    parser.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::{Expect, expect};

    fn check_file(input: &str, expect: Expect) {
        let parse = parse_file(input);
        let output = format!("{:#?}", parse.syntax());
        expect.assert_eq(&output);
    }

    #[test]
    fn parse_file_empty() {
        check_file("", expect![[r#"
            ROOT@0..0
        "#]]);
    }

    #[test]
    fn parse_file_trivial() {
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

    fn check_module(input: &str, expect: Expect) {
        let parse = parse_module_entry(input);
        let output = format!("{:#?}", parse.syntax());
        expect.assert_eq(&output);
    }

    fn check_module_no_errors(input: &str) {
        let parse = parse_module_entry(input);
        if !parse.errors().is_empty() {
            for err in parse.errors() {
                eprintln!("error at {:?}: {}", err.range, err.message);
            }
            eprintln!("tree:\n{:#?}", parse.syntax());
            panic!("module parse had {} error(s)", parse.errors().len());
        }
    }

    #[test]
    fn parse_module_empty() {
        check_module("", expect![[r#"
            ROOT@0..0
        "#]]);
    }

    #[test]
    fn parse_module_const() {
        check_module_no_errors("const X: u32 = 32u32;");
    }

    #[test]
    fn parse_module_struct() {
        check_module_no_errors("struct Data { values: u32, }");
    }

    #[test]
    fn parse_module_inline_fn() {
        check_module_no_errors("fn helper() -> u32 { return 0u32; }");
    }

    #[test]
    fn parse_module_mixed_items() {
        check_module_no_errors(
            "const X: u32 = 3;\n\
             struct Data { values: u32, }\n\
             fn helper() -> u32 { return 0u32; }",
        );
    }

    #[test]
    fn parse_module_with_comments() {
        // Module sections in test files are separated by comments.
        // Comments are trivia and should be skipped.
        check_module_no_errors(
            "// --- Next Module: dep.leo --- //\n\
             const X: u32 = 32u32;\n\
             // --- Next Module: dep/inner.leo --- //\n\
             const Y: u32 = 64u32;",
        );
    }

    #[test]
    fn parse_file_with_module_sections() {
        // Multi-section test files: program block followed by module items.
        let source = "\
program test.aleo {
    fn foo() -> u32 { return 0u32; }
}

// --- Next Module: dep.leo --- //

const X: u32 = 32u32;

// --- Next Module: dep/inner.leo --- //

const Y: u32 = 64u32;";

        let parse = parse_file(source);
        if !parse.errors().is_empty() {
            for err in parse.errors() {
                eprintln!("error at {:?}: {}", err.range, err.message);
            }
            eprintln!("tree:\n{:#?}", parse.syntax());
            panic!("file parse had {} error(s)", parse.errors().len());
        }
    }
}
