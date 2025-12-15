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

use leo_errors::Lint;
use leo_parser_lossless::{ExpressionKind, LiteralKind, SyntaxKind, SyntaxNode};

use crate::{context::EarlyContext, passes::EarlyLintPass};

/// A lint to check for numeric types starting with zero.
pub(super) struct ZeroPrefixedLiteral<'ctx> {
    context: EarlyContext<'ctx>,
}

impl<'ctx> EarlyLintPass<'ctx> for ZeroPrefixedLiteral<'ctx> {
    fn new(context: EarlyContext<'ctx>) -> Box<dyn EarlyLintPass<'ctx> + 'ctx> {
        Box::new(Self { context })
    }

    fn get_name(&self) -> &str {
        "zero prefixed literal"
    }

    fn check_node(&mut self, node: &SyntaxNode<'_>) {
        if let SyntaxKind::Expression(ExpressionKind::Literal(LiteralKind::Integer(_) | LiteralKind::Unsuffixed)) =
            node.kind
        {
            let text = node.text.replace("_", "");
            if text.starts_with("0") && text.parse::<u128>() != Ok(0) {
                self.context.emit_lint(Lint::zero_prefixed_literal(node.text, node.span))
            }
        }
    }
}
