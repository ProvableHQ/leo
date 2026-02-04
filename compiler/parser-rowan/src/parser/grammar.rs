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
        // Until we implement the full grammar, this just consumes tokens
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
}
