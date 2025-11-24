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
    pub(crate) fn format_function(&mut self, node: &SyntaxNode<'_>) -> Output {
        assert_eq!(node.kind, SyntaxKind::Function);

        let mut ann_len = 0;
        while let Some(ann) = node.children.get(ann_len).filter(|child| child.kind == SyntaxKind::Annotation) {
            self.format_annotation(ann)?;
            self.maybe_bump_line()?;
            ann_len += 1;
        }

        let children = &node.children[ann_len..];

        let (a, k, i, c, p, ar, o, b) = match children {
            [k, name, p, b] => (None, k, name, None, p, None, None, b),

            [a, k, name, p, b] if a.text == "async" => (Some(a), k, name, None, p, None, None, b),
            [k, name, c, p, b] => (None, k, name, Some(c), p, None, None, b),

            [a, k, name, c, p, b] if a.text == "async" => (Some(a), k, name, Some(c), p, None, None, b),
            [k, name, p, ar, o, b] => (None, k, name, None, p, Some(ar), Some(o), b),

            [a, k, name, p, ar, o, b] if a.text == "async" => (Some(a), k, name, None, p, Some(ar), Some(o), b),
            [k, name, c, p, ar, o, b] => (None, k, name, Some(c), p, Some(ar), Some(o), b),

            [a, k, name, c, p, ar, o, b] => (Some(a), k, name, Some(c), p, Some(ar), Some(o), b),
            _ => panic!("Unexpected number of children"),
        };

        if let Some(a_) = a {
            self.node_with_trivia(a_, 0)?;
            self.space()?;
        }

        self.node_with_trivia(k, 0)?;
        self.space()?;

        self.node_with_trivia(i, 0)?;

        if let Some(c_) = c {
            self.format_const_list(c_)?;
        }

        let format_func = |slf: &mut Formatter<'_, '_>, node: &SyntaxNode<'_>| {
            assert_eq!(node.kind, SyntaxKind::Parameter);
            let [pk @ .., i, c, t] = &node.children[..] else { panic!("Can't happen") };
            if let Some(pk_) = pk.first() {
                slf.node_with_trivia(pk_, 0)?;
                slf.space()?;
            }

            slf.node_with_trivia(i, 0)?;
            slf.node_with_trivia(c, 0)?;
            slf.space()?;

            slf.format_type(t)?;

            Ok(())
        };

        self.format_collection(&p.children, false, false, format_func)?;

        if let Some(ar) = ar {
            self.space()?;
            self.node_with_trivia(ar, 0)?;
            self.space()?;
            let o = o.unwrap();
            let format_func = |slf: &mut Formatter<'_, '_>, node: &SyntaxNode<'_>| {
                let [pk @ .., t] = &node.children[..] else { panic!("Can't happen") };
                if let Some(pk) = pk.first() {
                    slf.node_with_trivia(pk, 0)?;
                    slf.space()?;
                }
                slf.format_type(t)?;
                Ok(())
            };
            match o.children.len() {
                1 | 2 => format_func(self, o),
                3 => self.format_collection(&o.children, false, false, format_func),
                _ => panic!("Can't happen"),
            }?
        }

        self.space()?;
        self.format_block(b, true)?;

        Ok(())
    }
}

impl_tests!(
    test_format_function,
    src = "program a.aleo {
            transition main() {
            let a = 4u32    ;
            b = a
            
            
            ;

            }
        }",
    exp = "program a.aleo {
    transition main() {
        let a = 4u32;
        b = a;
    }
}
",
    Kind::Main
);
