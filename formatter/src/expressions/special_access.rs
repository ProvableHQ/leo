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
    pub(super) fn format_special_access(&mut self, node: &SyntaxNode<'_>) -> Output {
        assert_eq!(node.kind, SyntaxKind::Expression(ExpressionKind::SpecialAccess));

        let [qualifier, dot, name] = &node.children[..] else {
            panic!("Can't happen");
        };

        self.node_with_trivia(qualifier, 0)?;
        self.node_with_trivia(dot, 0)?;
        self.node_with_trivia(name, 0)?;

        Ok(())
    }
}

impl_tests!(test_format_special_access, src = "block.height", exp = "block.height", Kind::Expression);
