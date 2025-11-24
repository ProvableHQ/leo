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

use leo_parser_lossless::{StatementKind, SyntaxKind, SyntaxNode};

use crate::{Formatter, Output, impl_tests};

impl Formatter<'_, '_> {
    pub(crate) fn format_block(&mut self, node: &SyntaxNode<'_>, trailing_empty_lines: bool) -> Output {
        assert_eq!(node.kind, SyntaxKind::Statement(StatementKind::Block));
        let [l, stms @ .., r] = &node.children[..] else { panic!("Can't happen") };
        self.node_with_trivia(l, 1)?;

        if !stms.is_empty() {
            self.scope(|slf| slf.format_statements(stms))?;
        }

        self.node_with_trivia(r, if trailing_empty_lines { 2 } else { 1 })?;

        Ok(())
    }
}

impl_tests!(
    test_format_block,
    src = "{
                let a = b;
        }",
    exp = "{
    let a = b;
}",
    Kind::Statement
);
