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

use leo_parser_lossless::{StatementKind, SyntaxKind, SyntaxNode};

use crate::{Formatter, Output, impl_tests};

impl Formatter<'_, '_> {
    pub(super) fn format_assert_neq(&mut self, node: &SyntaxNode<'_>, trailing_empty_lines: bool) -> Output {
        assert_eq!(node.kind, SyntaxKind::Statement(StatementKind::AssertNeq));

        let [a, left, e0, c, e1, right, s] = &node.children[..] else {
            panic!("Can't happen");
        };

        self.node_with_trivia(a, 0)?;
        self.node_with_trivia(left, 0)?;
        self.format_expression(e0)?;
        self.node_with_trivia(c, 0)?;
        self.space()?;
        self.format_expression(e1)?;
        self.node_with_trivia(right, 0)?;
        self.node_with_trivia(s, if trailing_empty_lines { 2 } else { 1 })?;

        Ok(())
    }
}

impl_tests!(
    test_assert_neq,
    src = "assert_neq/*dddd*/(true, false); //444",
    exp = "assert_neq /*dddd*/ (true, false); //444\n",
    Kind::Statement
);
