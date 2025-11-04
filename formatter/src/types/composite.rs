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

use leo_parser_lossless::{SyntaxKind, SyntaxNode, TypeKind};

use crate::{Formatter, Output, impl_tests};

impl Formatter<'_, '_> {
    pub(super) fn format_composite_type(&mut self, node: &SyntaxNode<'_>) -> Output {
        assert_eq!(node.kind, SyntaxKind::Type(TypeKind::Composite));
        let name = &node.children[0];
        if name.text.split_once(".aleo/").is_some() {
            self.node_with_trivia(name, 1)?;
        } else {
            self.push_snippet(name.text)?;
            if let Some(arg_list) = node.children.get(1) {
                self.push_snippet(arg_list.children[0].text)?;
                self.format_collection(&arg_list.children[1..], false, false, Self::format_expression)?;
            }
            self.consolidate_trivia(&name.children, 1)?;
        }
        Ok(())
    }
}

impl_tests!(
    test_format_composite_type,
    src = "let a: type  ::[2   ,    4 ] = 1;",
    exp = "let a: type::[2, 4] = 1;",
    Kind::Statement
);
