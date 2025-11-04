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
use leo_parser_lossless::{StatementKind, SyntaxKind, SyntaxNode};

use crate::{context::EarlyContext, passes::EarlyLintPass};

use std::ops::Add;

/// A lint to check for unnecessary braces.
pub(super) struct UnnecessaryBraces<'ctx> {
    context: EarlyContext<'ctx>,
}

impl<'ctx> EarlyLintPass<'ctx> for UnnecessaryBraces<'ctx> {
    fn new(context: EarlyContext<'ctx>) -> Box<dyn EarlyLintPass<'ctx> + 'ctx> {
        Box::new(Self { context })
    }

    fn get_name(&self) -> &str {
        "unnecessary braces"
    }

    fn check_node(&mut self, node: &SyntaxNode) {
        if let Err(lint) = check_block_inner(node) {
            self.context.emit_lint(lint);
        }
    }
}

fn check_block_inner(node: &SyntaxNode<'_>) -> Result<(), Lint> {
    if SyntaxKind::Statement(StatementKind::Block) == node.kind {
        if let [left, one_stmt, right] = &node.children[..]
            && one_stmt.kind == SyntaxKind::Statement(StatementKind::Block)
        {
            return Err(Lint::useless_braces(left.span.add(right.span)));
        }

        if let [left, right] = &node.children[..] {
            return Err(Lint::empty_braces(left.span.add(right.span)));
        }
    }

    Ok(())
}
