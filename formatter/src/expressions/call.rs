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
    pub(super) fn format_call(&mut self, node: &SyntaxNode<'_>) -> Output {
        assert_eq!(node.kind, SyntaxKind::Expression(ExpressionKind::Call));

        let [name, rest @ ..] = &node.children[..] else { panic!("Can't happen") };

        self.node_with_trivia(name, 1)?;

        let mut args_len = 0;
        if matches!(rest[0].kind, SyntaxKind::ConstArgumentList) {
            self.format_const_list(&rest[0])?;
            args_len += 1;
        }

        self.format_collection(&rest[args_len..], false, false, Self::format_expression)?;

        Ok(())
    }
}

impl_tests!(test_format_call, src = "a\t\t(a,\n\n\n\n\nb)", exp = "a(a, b)", Kind::Expression);
