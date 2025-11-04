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
    pub(super) fn format_option_type(&mut self, node: &SyntaxNode<'_>) -> Output {
        assert_eq!(node.kind, SyntaxKind::Type(TypeKind::Optional));
        let [inner_type, q] = &node.children[..] else { panic!("Can't happen") };

        self.format_type(inner_type)?;
        self.node_with_trivia(q, 0)?;

        Ok(())
    }
}

impl_tests!(
    test_format_option_type,
    src = "let a: [(u32,field,group,u32,field,group,u32,field, group, u32, field, group, u32,field,group); 2]? = 1;",
    exp = "let a: [(
    u32,
    field,
    group,
    u32,
    field,
    group,
    u32,
    field,
    group,
    u32,
    field,
    group,
    u32,
    field,
    group,
); 2]? = 1;",
    Kind::Statement
);
