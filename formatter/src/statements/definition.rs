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

use biome_formatter::prelude::soft_line_break;
use leo_parser_lossless::{StatementKind, SyntaxKind, SyntaxNode};

use crate::{Formatter, Output, impl_tests};

impl Formatter<'_, '_> {
    pub(super) fn format_definition(&mut self, node: &SyntaxNode<'_>, trailing_empty_lines: bool) -> Output {
        assert_eq!(node.kind, SyntaxKind::Statement(StatementKind::Definition));
        let [l, ids @ .., a, e, s] = &node.children[..] else { panic!("Can't happen") };

        self.node_with_trivia(l, 1)?;
        self.space()?;

        let mut split = ids.split(|c| c.text == ":");
        let maybe_ids = split.next().unwrap();
        if maybe_ids.len() == 1 {
            self.node_with_trivia(&maybe_ids[0], 1)?;
        } else {
            self.format_collection(maybe_ids, false, false, |slf, node| slf.node_with_trivia(node, 1))?;
        }

        if split.next().is_some() {
            let [.., c, t] = ids else { panic!("Can't happen") };
            self.node_with_trivia(c, 1)?;
            self.space()?;
            self.format_type(t)?;
        }

        self.space()?;
        self.node_with_trivia(a, 1)?;

        self.group(|slf| {
            slf.soft_indent_or_space(|slf| {
                slf.format_expression(e)?;
                Ok(())
            })?;

            slf.push(&soft_line_break())?;
            slf.push_snippet(s.text)?;

            Ok(())
        })?;

        self.consolidate_trivia(&s.children, if trailing_empty_lines { 2 } else { 1 })?;

        Ok(())
    }
}

impl_tests!(
    test_format_definition,
    src = "let(aa, bb, cc):u32= (55==4)+2   \n;\n",
    exp = "let (aa, bb, cc): u32 = (55 == 4) + 2;\n",
    Kind::Statement
);
