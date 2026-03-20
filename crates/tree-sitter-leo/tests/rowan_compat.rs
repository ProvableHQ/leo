// Copyright (C) 2019-2026 Provable Inc.
// This file is part of the Leo library.
//
// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use leo_parser_rowan::parse_file;
use std::{fs, path::Path};
use tree_sitter::{Node, Parser};

const FIXTURES: &[&str] = &[
    "tests/fixtures/annotations.leo",
    "tests/fixtures/const_generics.leo",
    "tests/fixtures/interfaces.leo",
    "tests/fixtures/locators.leo",
];

#[test]
fn focused_rowan_fixtures_parse_without_tree_sitter_errors() {
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_leo::LANGUAGE.into())
        .unwrap_or_else(|error| panic!("failed to load tree-sitter language: {error}"));

    for relative_path in FIXTURES {
        let fixture_path = crate_dir.join(relative_path);
        let source = fs::read_to_string(&fixture_path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", fixture_path.display()));

        let rowan_parse = parse_file(&source);
        assert!(
            rowan_parse.is_ok(),
            "rowan rejected {}:\nparse errors:\n{}\nlex errors:\n{}",
            fixture_path.display(),
            format_rowan_errors(rowan_parse.errors()),
            format_lex_errors(rowan_parse.lex_errors()),
        );

        let tree = parser
            .parse(&source, None)
            .unwrap_or_else(|| panic!("tree-sitter returned no parse tree for {}", fixture_path.display()));

        let root = tree.root_node();
        assert!(
            !root.has_error(),
            "tree-sitter produced ERROR nodes for {}:\n{}",
            fixture_path.display(),
            root.to_sexp(),
        );
        assert!(
            !contains_missing(root),
            "tree-sitter produced MISSING nodes for {}:\n{}",
            fixture_path.display(),
            root.to_sexp(),
        );
    }
}

fn contains_missing(node: Node<'_>) -> bool {
    if node.is_missing() {
        return true;
    }

    let mut cursor = node.walk();
    let has_missing = node.children(&mut cursor).any(contains_missing);
    has_missing
}

fn format_rowan_errors(errors: &[leo_parser_rowan::ParseError]) -> String {
    if errors.is_empty() {
        return "none".to_string();
    }

    errors
        .iter()
        .map(|error| {
            format!("{} @ {:?} (found: {:?}, expected: {:?})", error.message, error.range, error.found, error.expected)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_lex_errors(errors: &[leo_parser_rowan::LexError]) -> String {
    if errors.is_empty() {
        return "none".to_string();
    }

    errors.iter().map(|error| format!("{error:?}")).collect::<Vec<_>>().join("\n")
}
