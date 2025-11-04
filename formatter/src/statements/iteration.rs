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
    pub(super) fn format_iteration(&mut self, node: &SyntaxNode<'_>, trailing_empty_lines: bool) -> Output {
        assert_eq!(node.kind, SyntaxKind::Statement(StatementKind::Iteration));
        let [f, id @ .., n, low, d, hi, block] = &node.children[..] else { panic!("Can't happen") };

        self.node_with_trivia(f, 1)?;
        self.space()?;
        self.node_with_trivia(&id[0], 1)?;

        if id.len() != 1 {
            let [_, c, t] = id else { panic!("Can't happen") };
            self.node_with_trivia(c, 1)?;
            self.space()?;
            self.node_with_trivia(t, 1)?;
        }

        self.space()?;
        self.node_with_trivia(n, 1)?;

        self.space()?;
        self.format_expression(low)?;
        self.node_with_trivia(d, 1)?;
        self.format_expression(hi)?;

        self.space()?;
        self.format_block(block, trailing_empty_lines)?;

        Ok(())
    }
}

impl_tests!(test_format_iteration, src = "for i in a..3{}", exp = "for i in a..3 {}", Kind::Statement);
