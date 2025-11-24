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
    pub(super) fn format_composite(&mut self, node: &SyntaxNode<'_>) -> Output {
        assert_eq!(node.kind, SyntaxKind::StructDeclaration);

        let [struct_or_record, i, maybe_const_list @ .., members] = &node.children[..] else {
            panic!("Can't happen");
        };

        self.node_with_trivia(struct_or_record, 0)?;
        self.space()?;
        self.node_with_trivia(i, 0)?;

        if let Some(const_list) = maybe_const_list.first() {
            self.format_const_list(const_list)?;
        }

        let format_func = |slf: &mut Formatter<'_, '_>, member: &SyntaxNode<'_>| {
            assert_eq!(member.kind, SyntaxKind::StructMemberDeclaration);
            let [k @ .., i, c, t] = &member.children[..] else {
                panic!("Can't happen");
            };

            if let Some(k) = k.first() {
                slf.node_with_trivia(k, 0)?;
                slf.space()?;
            }

            slf.node_with_trivia(i, 0)?;
            slf.node_with_trivia(c, 0)?;
            slf.space()?;
            slf.format_type(t)?;

            Ok(())
        };

        self.space()?;
        self.format_collection(&members.children, true, true, format_func)?;

        Ok(())
    }
}

impl_tests!(
    test_format_composite,
    src = "struct A{a:u32}",
    exp = "struct A {
    a: u32,
}",
    Kind::Module
);
