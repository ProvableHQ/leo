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
    pub(super) fn format_program(&mut self, node: &SyntaxNode<'_>) -> Output {
        assert_eq!(node.kind, SyntaxKind::ProgramDeclaration);
        let [prgm, pid, left] = &node.children[..3] else { panic!("Can't happen") };

        let right = node.children.last().unwrap();

        self.push_snippet(prgm.text)?;
        self.space()?;
        self.push_snippet(pid.text)?;
        self.space()?;
        self.push_snippet(left.text)?;

        if prgm
            .children
            .iter()
            .chain(pid.children.iter())
            .chain(left.children.iter())
            .any(|trivia| matches!(trivia.kind, SyntaxKind::CommentBlock | SyntaxKind::CommentLine))
        {
            self.space()?;
            self.consolidate_trivia(&prgm.children[..], 0)?;
            self.consolidate_trivia(&pid.children[..], 0)?;
        }

        let mut cons = Vec::new();
        let mut funcs = Vec::new();
        let mut consts = Vec::new();
        let mut maps = Vec::new();
        let mut structs = Vec::new();
        let mut storages = Vec::new();

        let mut children = false;
        for i in 3..node.children.len() - 1 {
            children = true;
            let child = &node.children[i];
            match child.kind {
                SyntaxKind::Constructor => cons.push(child),
                SyntaxKind::GlobalConst => consts.push(child),
                SyntaxKind::Mapping => maps.push(child),
                SyntaxKind::StructDeclaration => structs.push(child),
                SyntaxKind::Function => funcs.push(child),
                SyntaxKind::Storage => storages.push(child),
                _ => panic!("Can't happen"),
            }
        }

        let mut written = false;

        let func = |slf: &mut Formatter<'_, '_>| {
            slf.consolidate_trivia(&left.children[..], 2)?;
            for child in consts {
                written = true;
                slf.format_global_const(child)?;
                slf.maybe_bump_line_else_ignore()?;
            }

            for (i, child) in maps.into_iter().enumerate() {
                if i == 0 && written {
                    slf.maybe_bump_lines()?;
                }
                slf.format_mapping(child)?;
                slf.maybe_bump_line_else_ignore()?;
                written = true;
            }

            for (i, child) in storages.into_iter().enumerate() {
                if i == 0 && written {
                    slf.maybe_bump_lines()?;
                }
                slf.format_storage(child)?;
                slf.maybe_bump_line_else_ignore()?;
                written = true;
            }

            for child in structs {
                if written {
                    slf.maybe_bump_lines()?
                }
                slf.format_composite(child)?;
                written = true;
            }

            for child in cons {
                if written {
                    slf.maybe_bump_lines()?
                }
                slf.format_constructor(child)?;
                written = true;
            }

            for child in funcs {
                if written {
                    slf.maybe_bump_lines()?
                }
                slf.format_function(child)?;
                written = true;
            }

            Ok(())
        };

        if children {
            self.scope(func)?;
        }

        self.node_with_trivia(right, 2)?;

        Ok(())
    }
}
