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
    pub(crate) fn format_const_general(&mut self, node: &SyntaxNode<'_>, trailing_empty_lines: bool) -> Output {
        assert!(matches!(node.kind, SyntaxKind::GlobalConst | SyntaxKind::Statement(StatementKind::Const)));
        let [const_, name, c, type_, a, rhs, s] = &node.children[..] else {
            panic!("Can't happen");
        };

        self.node_with_trivia(const_, 1)?;

        self.space()?;

        self.node_with_trivia(name, 1)?;
        self.node_with_trivia(c, 1)?;

        self.space()?;

        self.format_type(type_)?;

        self.space()?;

        self.node_with_trivia(a, 1)?;

        self.group(|slf| {
            slf.soft_indent_or_space(|slf| {
                slf.format_expression(rhs)?;
                Ok(())
            })?;

            slf.node_with_trivia(s, if trailing_empty_lines { 2 } else { 1 })?;

            Ok(())
        })?;

        Ok(())
    }
}

impl<'a, 'b> Formatter<'a, 'b>
where
    'b: 'a,
{
    pub(super) fn format_const_list(&mut self, node: &SyntaxNode<'_>) -> Output {
        let [c, s_ps_r @ ..] = &node.children[..] else { panic!("Can't happen") };
        self.node_with_trivia(c, 0)?;
        let format_func = match node.kind {
            SyntaxKind::ConstParameterList => |slf: &mut Formatter<'a, 'b>, node: &SyntaxNode<'_>| {
                assert_eq!(node.kind, SyntaxKind::ConstParameter);
                let [i, c, t] = node.children[..].as_ref() else { panic!("Can't happen") };
                slf.node_with_trivia(i, 1)?;
                slf.node_with_trivia(c, 1)?;
                slf.space()?;
                slf.format_type(t)?;
                Ok(())
            },
            SyntaxKind::ConstArgumentList => Self::format_expression,
            _ => panic!("Can't happen"),
        };

        self.format_collection(s_ps_r, false, false, format_func)?;
        self.space()?;
        Ok(())
    }
}

impl_tests!(
    test_format_const_param_list,
    src = "struct Example::[M: u32, N: u32] {}",
    exp = "struct Example::[M: u32, N: u32] {}",
    Kind::Module,
    test_format_const_arg_list,
    src = "Example::[M, N] {}",
    exp = "Example::[M, N] {}",
    Kind::Expression
);
