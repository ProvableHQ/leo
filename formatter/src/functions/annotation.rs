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

use biome_formatter::prelude::{if_group_breaks, soft_line_break_or_space, text};
use leo_parser_lossless::{SyntaxKind, SyntaxNode};

use crate::{Formatter, Output, impl_tests};

impl Formatter<'_, '_> {
    pub(crate) fn format_annotation(&mut self, node: &SyntaxNode<'_>) -> Output {
        assert_eq!(node.kind, SyntaxKind::Annotation);
        match node.children.len() {
            2 | 3 => {
                let [at, ann_name] = &node.children[..2] else { panic!("Can't happen") };

                self.push_snippet(at.text)?;
                self.push_snippet(ann_name.text)?;

                if let Some(ann_list) = node.children.get(2).filter(|list| list.children.len() > 2) {
                    assert_eq!(ann_list.kind, SyntaxKind::AnnotationList);

                    let left = ann_list.children.first().unwrap();

                    self.node_with_trivia(left, 0)?;

                    let right = ann_list.children.last().unwrap();

                    self.soft_scope(|slf| {
                        for (index, member) in
                            ann_list.children.iter().filter(|m| m.kind == SyntaxKind::AnnotationMember).enumerate()
                        {
                            let [key, assign, value] = &member.children[..] else {
                                panic!("Can't happen");
                            };

                            if index > 0 {
                                slf.push_snippet(",")?;
                                slf.push(&soft_line_break_or_space())?;
                            }
                            slf.node_with_trivia(key, 0)?;
                            slf.space()?;
                            slf.node_with_trivia(assign, 0)?;
                            slf.space()?;
                            slf.node_with_trivia(value, 0)?;
                        }

                        slf.push(&if_group_breaks(&text(",")))?;

                        Ok(())
                    })?;

                    self.node_with_trivia(right, 1)?;
                }

                self.consolidate_trivia(&at.children, 1)?;
                self.consolidate_trivia(&ann_name.children, 1)?;
            }
            _ => panic!("Can't happen"),
        }
        Ok(())
    }
}

impl_tests!(
    test_format_annotation,
    src = "program a.aleo {
            @checksum(mapping = 
            
            \"test.aleo/expected_checksum\", 
            
            key = \"true\")
            async constructor () {
        let a = 4;
        b = a;
    }
}
",
    exp = "program a.aleo {
    @checksum(mapping = \"test.aleo/expected_checksum\", key = \"true\")
    async constructor() {
        let a = 4;
        b = a;
    }
}
",
    Kind::Main
);
