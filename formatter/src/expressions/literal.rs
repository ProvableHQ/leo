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

use leo_parser_lossless::{ExpressionKind, SyntaxKind, SyntaxNode};

use crate::{Formatter, Output, impl_tests};

impl Formatter<'_, '_> {
    pub(super) fn format_literal(&mut self, node: &SyntaxNode) -> Output {
        let SyntaxKind::Expression(ExpressionKind::Literal(kind)) = node.kind else { panic!("Can't happen") };
        let kind_text = match kind {
            leo_parser_lossless::LiteralKind::Address => "",
            leo_parser_lossless::LiteralKind::Boolean => "",
            leo_parser_lossless::LiteralKind::Field => "field",
            leo_parser_lossless::LiteralKind::Group => "group",
            leo_parser_lossless::LiteralKind::Integer(integer_literal_kind) => match integer_literal_kind {
                leo_parser_lossless::IntegerLiteralKind::U8 => "u8",
                leo_parser_lossless::IntegerLiteralKind::U16 => "u16",
                leo_parser_lossless::IntegerLiteralKind::U32 => "u32",
                leo_parser_lossless::IntegerLiteralKind::U64 => "u64",
                leo_parser_lossless::IntegerLiteralKind::U128 => "u128",
                leo_parser_lossless::IntegerLiteralKind::I8 => "i8",
                leo_parser_lossless::IntegerLiteralKind::I16 => "i16",
                leo_parser_lossless::IntegerLiteralKind::I32 => "i32",
                leo_parser_lossless::IntegerLiteralKind::I64 => "i64",
                leo_parser_lossless::IntegerLiteralKind::I128 => "i128",
            },
            leo_parser_lossless::LiteralKind::Scalar => "scalar",
            leo_parser_lossless::LiteralKind::Unsuffixed => "",
            leo_parser_lossless::LiteralKind::String => "",
            leo_parser_lossless::LiteralKind::None => "",
        };

        self.push_snippet(format!("{}{}", node.text, kind_text).as_str())?;
        self.consolidate_trivia(&node.children, 0)?;

        Ok(())
    }
}

impl_tests!(
    test_format_literal_boolean,
    src = "false",
    exp = "false",
    Kind::Expression,
    test_format_literal_integer,
    src = "4u32",
    exp = "4u32",
    Kind::Expression,
    test_format_literal_scalar,
    src = "123scalar",
    exp = "123scalar",
    Kind::Expression,
    test_format_literal_unsuffixed,
    src = "123",
    exp = "123",
    Kind::Expression
);
