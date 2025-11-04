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

use biome_formatter::{BufferExtensions, FormatElement};
use leo_parser_lossless::{SyntaxKind, SyntaxNode};

use crate::{Formatter, Output};

impl Formatter<'_, '_> {
    pub(crate) fn consolidate_trivia(&mut self, nodes: &[SyntaxNode], line_breaks: u32) -> Output {
        let mut args = Vec::new();
        let mut nodes_iter = nodes.iter().filter(|n| !matches!(n.kind, SyntaxKind::Whitespace)).peekable();
        while let Some(node) = nodes_iter.next() {
            match node.kind {
                SyntaxKind::Whitespace => continue, // we don't allow arbitary white spaces, they are handled manually.
                SyntaxKind::Linebreak => {
                    match (self.last_lines, line_breaks) {
                        (_, 0) => continue,
                        (1, 1) => continue,
                        (x, _) if x >= 2 => continue,
                        _ => {}
                    }
                    args.push(FormatElement::Line(biome_formatter::prelude::LineMode::Hard));
                    self.bump_lines();
                    if line_breaks == 2 && matches!(nodes_iter.peek().map(|n| n.kind), Some(SyntaxKind::Linebreak)) {
                        args.push(FormatElement::Line(biome_formatter::prelude::LineMode::Empty));
                        let _ = nodes_iter.next();
                        self.bump_lines();
                    }
                    while matches!(nodes_iter.peek().map(|n| n.kind), Some(SyntaxKind::Linebreak)) {
                        let _ = nodes_iter.next();
                    }

                    continue;
                }
                SyntaxKind::CommentBlock => {
                    args.push(FormatElement::Space);
                    args.push(FormatElement::DynamicText {
                        text: node.text.into(),
                        source_position: Default::default(),
                    });
                    args.push(FormatElement::Space);
                }
                SyntaxKind::CommentLine => {
                    args.push(FormatElement::Space);
                    args.push(FormatElement::DynamicText {
                        text: node.text.into(),
                        source_position: Default::default(),
                    });
                    args.push(FormatElement::Line(biome_formatter::prelude::LineMode::Hard));
                    let _ = nodes_iter.next();
                }
                _ => panic!("Can't happen"),
            }

            self.reset_lines();
        }

        self.group(|slf| slf.formatter.write_elements(args))
    }
}
