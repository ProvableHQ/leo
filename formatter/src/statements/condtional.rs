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
    pub(super) fn format_conditional(&mut self, node: &SyntaxNode<'_>, trailing_empty_lines: bool) -> Output {
        assert_eq!(node.kind, SyntaxKind::Statement(StatementKind::Conditional));
        match &node.children[..] {
            [if_, c, block] => {
                self.node_with_trivia(if_, 1)?;
                self.space()?;
                self.format_expression(c)?;
                self.space()?;
                self.format_block(block, trailing_empty_lines)?;
            }
            [if_, c, block, else_, otherwise] => {
                self.node_with_trivia(if_, 1)?;
                self.space()?;
                self.format_expression(c)?;
                self.space()?;
                self.format_block(block, trailing_empty_lines)?;
                self.space()?;
                self.node_with_trivia(else_, 1)?;
                self.space()?;
                self.format_block(otherwise, trailing_empty_lines)?;
            }

            _ => panic!("Can't happen"),
        }

        Ok(())
    }
}

impl_tests!(
    test_format_conditional,
    src = "if /*ddd*/a {
            ///dddd
        
            let a = 5;
        } //ddd
         else //ddddd
    {
    
    b = 6    ;
    }",
    exp = "if /*ddd*/ a {
///dddd
    let a = 5;
} //ddd
else //ddddd
{
    b = 6;
}",
    Kind::Statement
);
