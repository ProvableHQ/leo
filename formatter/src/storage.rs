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

use leo_parser_lossless::{SyntaxKind, SyntaxNode};

use crate::{Formatter, Output, impl_tests};

impl Formatter<'_, '_> {
    pub(super) fn format_storage(&mut self, node: &SyntaxNode<'_>) -> Output {
        assert_eq!(node.kind, SyntaxKind::Storage);

        let [storage, name, colon, type_, s] = &node.children[..] else {
            panic!("Can't happen");
        };

        self.node_with_trivia(storage, 0)?;

        self.space()?;
        self.node_with_trivia(name, 0)?;
        self.node_with_trivia(colon, 0)?;

        self.space()?;
        self.format_type(type_)?;
        self.node_with_trivia(s, 2)?;

        Ok(())
    }
}

impl_tests!(
    test_format_storage,
    src = "program a.aleo {
        storage users: [u32];}",
    exp = "program a.aleo {
    storage users: [u32];
}
",
    Kind::Main
);
