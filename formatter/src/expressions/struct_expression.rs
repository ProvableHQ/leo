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
    pub(super) fn format_struct(&mut self, node: &SyntaxNode<'_>) -> Output {
        assert_eq!(node.kind, SyntaxKind::Expression(ExpressionKind::Struct));

        let name = &node.children[0];
        self.node_with_trivia(name, 0)?;

        let mut args_len = 1;
        if matches!(node.children[1].kind, SyntaxKind::ConstArgumentList) {
            args_len += 1;
            self.format_const_list(&node.children[1])?;
        }

        self.space()?;

        let format_func = |slf: &mut Formatter<'_, '_>, initializer: &SyntaxNode<'_>| match &initializer.children[..] {
            [init_name] => slf.node_with_trivia(init_name, 0),
            [init_name, c, expr] => {
                slf.node_with_trivia(init_name, 0)?;
                slf.node_with_trivia(c, 0)?;
                slf.space()?;
                slf.format_expression(expr)?;
                Ok(())
            }
            _ => panic!("Can't happen"),
        };
        self.format_collection(&node.children[args_len..], false, true, format_func)?;

        Ok(())
    }
}

impl_tests!(
    test_format_struct,
    src = "a{a, 
    b,c: www,d}",
    exp = "a { a, b, c: www, d }",
    Kind::Expression
);
