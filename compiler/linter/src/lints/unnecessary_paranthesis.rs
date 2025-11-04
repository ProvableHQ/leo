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
use leo_parser_lossless::{ExpressionKind, SyntaxKind, SyntaxNode};

use crate::{context::EarlyContext, passes::EarlyLintPass};

/// A lint to check for unnecessary parentheses.
pub(super) struct UnnecessaryParens<'ctx> {
    context: EarlyContext<'ctx>,
}

impl<'ctx> EarlyLintPass<'ctx> for UnnecessaryParens<'ctx> {
    fn new(context: EarlyContext<'ctx>) -> Box<dyn EarlyLintPass<'ctx> + 'ctx> {
        Box::new(Self { context })
    }

    fn get_name(&self) -> &str {
        "unneccssary parenthesis"
    }

    fn check_node(&mut self, node: &SyntaxNode<'_>) {
        if let Err(expr) = recursive_check_parens(0, node) {
            self.context.emit_lint(Lint::useless_parens(expr.text, expr.span))
        }
    }
}

fn recursive_check_parens<'a>(depth: usize, node: &'a SyntaxNode<'a>) -> Result<(), &'a SyntaxNode<'a>> {
    if let SyntaxKind::Expression(ExpressionKind::Parenthesized) = node.kind {
        let [_left, expr, _right] = &node.children[..] else {
            panic!("Can't happen");
        };
        return recursive_check_parens(depth + 1, expr);
    }

    let mut error = false;
    if depth > 1 {
        error = true;
    }

    if depth == 1 && matches!(node.kind, SyntaxKind::Expression(ExpressionKind::Literal(_))) {
        error = true
    }

    if error {
        return Err(node);
    }

    Ok(())
}
