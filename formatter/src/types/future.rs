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
    pub(super) fn format_future_type(&mut self, node: &SyntaxNode<'_>) -> Output {
        assert_eq!(node.kind, SyntaxKind::Type(TypeKind::Future));
        if node.children.len() == 1 {
            self.node_with_trivia(&node.children[0], 1)?;
        } else {
            let [f, lt, fn_, lp_types_rp @ .., rt] = &node.children[..] else { panic!("Can't happen") };
            self.push_snippet(f.text)?;
            self.push_snippet(lt.text)?;
            self.push_snippet(fn_.text)?;
            self.format_collection(lp_types_rp, false, false, Self::format_type)?;
            self.push_snippet(rt.text)?;
            self.space()?;
            self.consolidate_trivia(&f.children, 1)?;
            self.space()?;
            self.consolidate_trivia(&lt.children, 1)?;
            self.space()?;
            self.consolidate_trivia(&fn_.children, 1)?;
            self.space()?;
            self.consolidate_trivia(&rt.children, 1)?;
        }
        Ok(())
    }
}

impl_tests!(
    test_format_future_type,
    src =
        "let a: Future<Fn(u32,field,group,u32,field,group,u32,field, group, u32, field, group, u32,field,group)> = 1;",
    exp = "let a: Future<Fn(
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
)> = 1;",
    Kind::Statement
);
