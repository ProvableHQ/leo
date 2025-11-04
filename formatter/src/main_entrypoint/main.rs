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

use crate::{Formatter, Output};

impl Formatter<'_, '_> {
    pub(crate) fn format_main(&mut self, node: &SyntaxNode<'_>) -> Output {
        assert_eq!(node.kind, SyntaxKind::MainContents);

        let mut trivia_start = 0;
        let mut trivia_end = 0;
        let mut empty_lines = true;

        for child in &node.children {
            match child.kind {
                SyntaxKind::Linebreak | SyntaxKind::Whitespace if empty_lines => {
                    trivia_start += 1;
                    trivia_end += 1;
                }
                SyntaxKind::Linebreak | SyntaxKind::Whitespace | SyntaxKind::CommentBlock | SyntaxKind::CommentLine => {
                    empty_lines = false;
                    trivia_end += 1;
                }
                _ => break,
            }
        }

        if !empty_lines && trivia_end > trivia_start {
            self.consolidate_trivia(&node.children[trivia_start..trivia_end], 2)?;
            self.maybe_bump_line()?;
        }

        let [imports @ .., program_node] = &node.children[trivia_end..] else { panic!("can't happen") };

        let mut import_empty_line = false;
        for import in imports {
            self.format_import(import)?;
            self.maybe_bump_line()?;
            import_empty_line = true;
        }

        if import_empty_line {
            self.maybe_bump_lines()?;
        }

        self.format_program(program_node)?;
        self.maybe_bump_line()
    }
}
