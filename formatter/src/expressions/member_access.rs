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

use crate::{Formatter, Output, impl_tests, utils::IndentMode};

impl Formatter<'_, '_> {
    pub(super) fn format_member_access(&mut self, node: &SyntaxNode<'_>) -> Output {
        assert_eq!(node.kind, SyntaxKind::Expression(ExpressionKind::MemberAccess));

        let [struct_, dot, name] = &node.children[..] else {
            panic!("Can't happen.");
        };

        self.format_expression(struct_)?;
        self.block(IndentMode::SoftLine, |slf| {
            slf.node_with_trivia(dot, 0)?;
            slf.node_with_trivia(name, 0)?;
            Ok(())
        })?;

        Ok(())
    }
}

impl_tests!(
    test_format_member_access,
    src = "return a[i]\n\n\n.\n\n\t\naaaaaaaaaaaa\
        .aaaaaaaaaaaaaaa\
        .aaaaaaaaaaaaaaaa.aaaaaaaaaaaaaaaaaaaa.\
        aaaaaaaaaaaaaaaaaaa.aaaaaaaaaaaaaaaaaaaaaaaa.\
        aaaaaaaaaaaaaaaaaaaaaaaa.aaaaaaaaaaaaaaaaaaaaaaaa;",
    exp = "return a[i]
    .aaaaaaaaaaaa
    .aaaaaaaaaaaaaaa
    .aaaaaaaaaaaaaaaa
    .aaaaaaaaaaaaaaaaaaaa
    .aaaaaaaaaaaaaaaaaaa
    .aaaaaaaaaaaaaaaaaaaaaaaa
    .aaaaaaaaaaaaaaaaaaaaaaaa
    .aaaaaaaaaaaaaaaaaaaaaaaa;",
    Kind::Statement
);
