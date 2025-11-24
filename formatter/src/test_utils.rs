// Copyright (C) 2019-2025 Provable Inc.
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

use biome_formatter::{Formatted, SimpleFormatContext, prelude::format_once};
use leo_errors::{Handler, Result};
use leo_parser::conversions::{to_expression, to_main, to_module, to_statement};
use leo_parser_lossless::SyntaxNode;
//use leo_span::create_session_if_not_set_then;

use crate::{Formatter, Output};

#[derive(Clone, Copy)]
pub(crate) enum Kind {
    Expression,
    Statement,
    Module,
    Main,
}

// The idea is that a formatter must be deterministic and as well as idempotent.
// i.e. fmt(x) = fmt(fmt(x)) for all x.
pub(crate) fn run_test(source: &str, expected: &str, kind: Kind) -> Result<()> {
    let tree = parse_cst(source, kind)?;
    let formatted = format_kind(&tree, kind);

    let expected_tree = parse_cst(expected, kind)?;
    let expected_formatted = format_kind(&expected_tree, kind);

    assert!(
        formatted.print().unwrap().as_code() == expected && expected == expected_formatted.print().unwrap().as_code()
    );

    // Todo: uncomment this, we cannot do this yet as the spans will be different.
    // create_session_if_not_set_then( |_| _parse_and_check_asts(&tree, &expected_tree, kind))?;

    Ok(())
}

fn format_kind(tree: &SyntaxNode, kind: Kind) -> Formatted<SimpleFormatContext> {
    match kind {
        Kind::Expression => format(tree, |f, n| f.format_expression(n)),
        Kind::Statement => format(tree, |f, n| f.format_statement(n, false)),
        Kind::Module => format(tree, |f, n| f.format_module(n)),
        Kind::Main => format(tree, |f, n| f.format_main(n)),
    }
}

fn format(
    tree: &SyntaxNode,
    formatter: impl Fn(&mut Formatter, &SyntaxNode) -> Output,
) -> Formatted<SimpleFormatContext> {
    biome_formatter::format!(Formatter::default_format_context(), [&format_once(|f| {
        formatter(&mut Formatter { last_lines: 0, formatter: f }, tree)
    })])
    .unwrap()
}

fn parse_cst(src: &'_ str, kind: Kind) -> Result<SyntaxNode<'_>> {
    match kind {
        Kind::Expression => leo_parser_lossless::parse_expression(Handler::default(), src, 0),
        Kind::Statement => leo_parser_lossless::parse_statement(Handler::default(), src, 0),
        Kind::Module => leo_parser_lossless::parse_module(Handler::default(), src, 0),
        Kind::Main => leo_parser_lossless::parse_main(Handler::default(), src, 0),
    }
}

fn _parse_and_check_asts(node: &SyntaxNode, expected: &SyntaxNode, kind: Kind) -> Result<()> {
    match kind {
        Kind::Expression => {
            let ast = to_expression(node, &Default::default(), &Default::default())?;
            let expected_ast = to_expression(expected, &Default::default(), &Default::default())?;
            assert_eq!(ast, expected_ast);
        }
        Kind::Statement => {
            let ast = to_statement(node, &Default::default(), &Default::default())?;
            let expected_ast = to_statement(expected, &Default::default(), &Default::default())?;
            assert_eq!(ast, expected_ast);
        }
        Kind::Module => {
            let ast =
                to_module(node, &Default::default(), Default::default(), Default::default(), &Default::default())?;
            let expected_ast =
                to_module(expected, &Default::default(), Default::default(), Default::default(), &Default::default())?;
            assert_eq!(ast, expected_ast);
        }
        Kind::Main => {
            let ast = to_main(node, &Default::default(), &Default::default())?;
            let expected_ast = to_main(expected, &Default::default(), &Default::default())?;
            assert_eq!(ast, expected_ast);
        }
    }

    Ok(())
}
