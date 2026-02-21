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

//! Rowan-based lossless syntax tree and parser for Leo.
//!
//! This crate provides a lossless parser using the rowan library, designed for
//! IDE-grade error recovery.

mod lexer;
mod parser;
mod syntax_kind;

use leo_errors::Result;
pub use lexer::{LexError, LexErrorKind, Token, lex};
pub use parser::{
    Parse,
    ParseError,
    Parser,
    parse_expression_entry,
    parse_file,
    parse_module_entry,
    parse_statement_entry,
};
pub use rowan::TextRange;
pub use syntax_kind::{SyntaxKind, syntax_kind_from_raw};

/// The Leo language type for rowan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LeoLanguage {}

impl rowan::Language for LeoLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        syntax_kind_from_raw(raw)
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

/// A syntax node in the Leo syntax tree.
pub type SyntaxNode = rowan::SyntaxNode<LeoLanguage>;

/// A syntax token in the Leo syntax tree.
pub type SyntaxToken = rowan::SyntaxToken<LeoLanguage>;

/// Either a syntax node or token.
pub type SyntaxElement = rowan::SyntaxElement<LeoLanguage>;

/// Parse an expression from source code.
///
/// # Arguments
/// * `source` - The source code to parse.
///
/// # Returns
/// A syntax node representing the parsed expression.
pub fn parse_expression(source: &str) -> Result<SyntaxNode> {
    let parse = parse_expression_entry(source);
    // TODO: Convert ParseErrors to leo_errors::Result when needed
    Ok(parse.syntax())
}

/// Parse a statement from source code.
///
/// # Arguments
/// * `source` - The source code to parse.
///
/// # Returns
/// A syntax node representing the parsed statement.
pub fn parse_statement(source: &str) -> Result<SyntaxNode> {
    let parse = parse_statement_entry(source);
    // TODO: Convert ParseErrors to leo_errors::Result when needed
    Ok(parse.syntax())
}

/// Parse a module from source code.
///
/// # Arguments
/// * `source` - The source code to parse.
///
/// # Returns
/// A syntax node representing the parsed module.
pub fn parse_module(source: &str) -> Result<SyntaxNode> {
    let parse = parse_module_entry(source);
    // TODO: Convert ParseErrors to leo_errors::Result when needed
    Ok(parse.syntax())
}

/// Parse a main program file from source code.
///
/// # Arguments
/// * `source` - The source code to parse.
///
/// # Returns
/// A syntax node representing the parsed program.
pub fn parse_main(source: &str) -> Result<SyntaxNode> {
    let parse = parse_file(source);
    // TODO: Convert ParseErrors to leo_errors::Result when needed
    Ok(parse.syntax())
}
