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
    pub(crate) fn format_constructor(&mut self, node: &SyntaxNode<'_>) -> Output {
        assert_eq!(node.kind, SyntaxKind::Constructor);

        for ann in node.children.iter().filter(|child| child.kind == SyntaxKind::Annotation) {
            self.format_annotation(ann)?;
            self.maybe_bump_line()?;
        }

        let [async_, cons, l, r, b] = &node.children[node.children.len() - 5..] else { panic!("Can't happen") };

        self.node_with_trivia(async_, 0)?;

        self.space()?;
        self.node_with_trivia(cons, 0)?;

        self.node_with_trivia(l, 0)?;
        self.node_with_trivia(r, 0)?;

        self.space()?;
        self.format_block(b, true)?;

        Ok(())
    }
}

impl_tests!(
    test_format_constructor,
    src = "program a.aleo {
            async constructor() {
            let a = 4u32  ;
            b = a
            
            
            ;

            }
        }",
    exp = "program a.aleo {
    async constructor() {
        let a = 4u32;
        b = a;
    }
}
",
    Kind::Main
);
