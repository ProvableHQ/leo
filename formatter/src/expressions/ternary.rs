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

use leo_parser_lossless::{ExpressionKind, SyntaxKind, SyntaxNode};

use crate::{Formatter, Output, impl_tests};

impl Formatter<'_, '_> {
    pub(super) fn format_ternary(&mut self, node: &SyntaxNode<'_>) -> Output {
        assert_eq!(node.kind, SyntaxKind::Expression(ExpressionKind::Ternary));
        let [cond, q, if_, c, then] = &node.children[..] else {
            panic!("Can't happen");
        };

        self.format_expression(cond)?;
        self.space()?;
        self.node_with_trivia(q, 0)?;
        self.space()?;
        self.format_expression(if_)?;
        self.space()?;
        self.node_with_trivia(c, 0)?;
        self.space()?;
        self.format_expression(then)?;

        Ok(())
    }
}

impl_tests!(test_format_ternary, src = "a ? a : b", exp = "a ? a : b", Kind::Expression);
