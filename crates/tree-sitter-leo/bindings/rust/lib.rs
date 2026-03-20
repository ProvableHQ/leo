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

//! This crate provides Rust access to the Leo [tree-sitter] grammar that lives
//! in the repository's top-level `tree-sitter/` directory.
//!
//! Typically, you will use the [`LANGUAGE`] constant to add this language to a
//! tree-sitter [`Parser`], and then use the parser to parse some code:
//!
//! ```
//! let code = r#"
//! "#;
//! let mut parser = tree_sitter::Parser::new();
//! let language = tree_sitter_leo::LANGUAGE;
//! parser
//!     .set_language(&language.into())
//!     .expect("Error loading Leo parser");
//! let tree = parser.parse(code, None).unwrap();
//! assert!(!tree.root_node().has_error());
//! ```
//!
//! [`Parser`]: https://docs.rs/tree-sitter/0.25.10/tree_sitter/struct.Parser.html
//! [tree-sitter]: https://tree-sitter.github.io/

use tree_sitter_language::LanguageFn;

unsafe extern "C" {
    fn tree_sitter_leo() -> *const ();
}

/// The tree-sitter [`LanguageFn`] for this grammar.
pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_leo) };

/// The content of the [`node-types.json`] file for this grammar.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers/6-static-node-types
pub const NODE_TYPES: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../tree-sitter/src/node-types.json"));

pub const FOLDS_QUERY: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../tree-sitter/queries/folds.scm"));
pub const HIGHLIGHTS_QUERY: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../tree-sitter/queries/highlights.scm"));
pub const INDENTS_QUERY: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../tree-sitter/queries/indents.scm"));
pub const LOCALS_QUERY: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../tree-sitter/queries/locals.scm"));

#[cfg(test)]
mod tests {
    #[test]
    fn test_can_load_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&super::LANGUAGE.into()).expect("Error loading Leo parser");
    }
}
