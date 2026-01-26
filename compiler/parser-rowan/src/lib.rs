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
//! IDE-grade error recovery. It will eventually replace the LALRPOP-based
//! parser in `leo-parser-lossless`.

use leo_errors::{Handler, Result};

/// Syntax kind enum for the rowan-based parser.
//
// TODO: This enum will be expanded as we implement the parser. Currently
// contains only the essential kinds needed for the initial setup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum SyntaxKind {
    /// Error node for wrapping parse errors.
    ERROR = 0,
    /// End of file marker.
    EOF,
    /// Whitespace trivia (spaces, tabs).
    WHITESPACE,
    /// Comment trivia (line and block comments).
    COMMENT,
    /// Root node of the syntax tree.
    ROOT,
    // TODO: Add remaining syntax kinds as we implement the parser.
    // See parent plan for the full list of kinds to add.
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

/// The Leo language type for rowan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LeoLanguage {}

impl rowan::Language for LeoLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        match raw.0 {
            0 => SyntaxKind::ERROR,
            1 => SyntaxKind::EOF,
            2 => SyntaxKind::WHITESPACE,
            3 => SyntaxKind::COMMENT,
            4 => SyntaxKind::ROOT,
            n => panic!("invalid SyntaxKind: {n}"),
        }
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
/// * `handler` - Error handler for collecting diagnostics.
/// * `source` - The source code to parse.
/// * `start_pos` - The starting byte position in the source file.
///
/// # Returns
/// A syntax node representing the parsed expression.
pub fn parse_expression(_handler: Handler, _source: &str, _start_pos: u32) -> Result<SyntaxNode> {
    todo!("rowan parser: parse_expression")
}

/// Parse a statement from source code.
///
/// # Arguments
/// * `handler` - Error handler for collecting diagnostics.
/// * `source` - The source code to parse.
/// * `start_pos` - The starting byte position in the source file.
///
/// # Returns
/// A syntax node representing the parsed statement.
pub fn parse_statement(_handler: Handler, _source: &str, _start_pos: u32) -> Result<SyntaxNode> {
    todo!("rowan parser: parse_statement")
}

/// Parse a module from source code.
///
/// # Arguments
/// * `handler` - Error handler for collecting diagnostics.
/// * `source` - The source code to parse.
/// * `start_pos` - The starting byte position in the source file.
///
/// # Returns
/// A syntax node representing the parsed module.
pub fn parse_module(_handler: Handler, _source: &str, _start_pos: u32) -> Result<SyntaxNode> {
    todo!("rowan parser: parse_module")
}

/// Parse a main program file from source code.
///
/// # Arguments
/// * `handler` - Error handler for collecting diagnostics.
/// * `source` - The source code to parse.
/// * `start_pos` - The starting byte position in the source file.
///
/// # Returns
/// A syntax node representing the parsed program.
pub fn parse_main(_handler: Handler, _source: &str, _start_pos: u32) -> Result<SyntaxNode> {
    todo!("rowan parser: parse_main")
}
