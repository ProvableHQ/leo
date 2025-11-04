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

use crate::passes::EarlyLintPass;

pub(super) struct EarlyLintingVisitor<'ctx> {
    /// All the lints that implement the behaviour for the early linting.
    pub(super) lints: Vec<Box<dyn EarlyLintPass<'ctx> + 'ctx>>,
}

impl EarlyLintingVisitor<'_> {
    pub(super) fn check_node(&mut self, node: &SyntaxNode, pattern: impl Fn(&SyntaxNode) -> bool) {
        assert!(pattern(node));
        for lint in self.lints.iter_mut() {
            lint.check_node(node);
        }
    }

    pub(super) fn visit_nodes(
        &mut self,
        node: &SyntaxNode,
        filter: impl Fn(&SyntaxNode) -> bool,
        mut func: impl FnMut(&mut Self, &SyntaxNode),
    ) {
        for node in &node.children {
            if !filter(node) {
                continue;
            }

            func(self, node);
        }
    }
}

pub(super) fn match_expression(node: &SyntaxNode) -> bool {
    matches!(node.kind, SyntaxKind::Expression(..))
}

pub(super) fn match_statement(node: &SyntaxNode) -> bool {
    matches!(node.kind, SyntaxKind::Statement(..))
}

pub(super) fn match_type(node: &SyntaxNode) -> bool {
    matches!(node.kind, SyntaxKind::Type(..))
}

pub(super) fn match_kind(kind: SyntaxKind) -> impl Fn(&SyntaxNode) -> bool {
    move |node| node.kind == kind
}
