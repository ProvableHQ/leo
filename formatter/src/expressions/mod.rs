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

mod array;
mod array_access;
mod associated_constant;
mod associated_function_call;
mod async_expression;
mod binary;
mod call;
mod cast;
mod literal;
mod locator;
mod member_access;
mod method_call;
mod parenthesized;
mod path;
mod repeat;
mod special_access;
mod struct_expression;
mod ternary;
mod tuple;
mod tuple_access;
mod unary;
mod unit;

use leo_parser_lossless::{SyntaxKind, SyntaxNode};

use crate::{Formatter, Output};

impl Formatter<'_, '_> {
    pub(crate) fn format_expression(&mut self, node: &SyntaxNode<'_>) -> Output {
        let SyntaxKind::Expression(expression_kind) = node.kind else { panic!("Can't happen") };

        match expression_kind {
            leo_parser_lossless::ExpressionKind::ArrayAccess => self.format_array_access(node),
            leo_parser_lossless::ExpressionKind::AssociatedConstant => self.format_associated_constant(node),
            leo_parser_lossless::ExpressionKind::AssociatedFunctionCall => self.format_associated_function_call(node),
            leo_parser_lossless::ExpressionKind::Async => self.format_async(node),
            leo_parser_lossless::ExpressionKind::Array => self.format_array(node),
            leo_parser_lossless::ExpressionKind::Binary => self.format_binary(node),
            leo_parser_lossless::ExpressionKind::Call => self.format_call(node),
            leo_parser_lossless::ExpressionKind::Cast => self.format_cast(node),
            leo_parser_lossless::ExpressionKind::Path => self.format_path(node),
            leo_parser_lossless::ExpressionKind::Literal(_) => self.format_literal(node),
            leo_parser_lossless::ExpressionKind::Locator => self.format_locator(node),
            leo_parser_lossless::ExpressionKind::MemberAccess => self.format_member_access(node),
            leo_parser_lossless::ExpressionKind::MethodCall => self.format_method_call(node),
            leo_parser_lossless::ExpressionKind::Parenthesized => self.format_parenthesized(node),
            leo_parser_lossless::ExpressionKind::Repeat => self.format_repeat(node),
            leo_parser_lossless::ExpressionKind::SpecialAccess => self.format_special_access(node),
            leo_parser_lossless::ExpressionKind::Struct => self.format_struct(node),
            leo_parser_lossless::ExpressionKind::Ternary => self.format_ternary(node),
            leo_parser_lossless::ExpressionKind::Tuple => self.format_tuple(node),
            leo_parser_lossless::ExpressionKind::TupleAccess => self.format_tuple_access(node),
            leo_parser_lossless::ExpressionKind::Unary => self.format_unary(node),
            leo_parser_lossless::ExpressionKind::Unit => self.format_unit_expression(node),
        }
    }
}
