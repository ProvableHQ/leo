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

use indexmap::IndexMap;
use leo_errors::Lint;
use leo_parser_lossless::{SyntaxKind, SyntaxNode};
use leo_span::Span;

use crate::{context::EarlyContext, passes::EarlyLintPass};

/// A lint to check for accidental duplicate imports in leo programs.
pub(super) struct DuplicateImportsLint<'ctx> {
    imports: IndexMap<String, Span>,
    context: EarlyContext<'ctx>,
}

impl<'ctx> EarlyLintPass<'ctx> for DuplicateImportsLint<'ctx> {
    fn new(context: EarlyContext<'ctx>) -> Box<dyn EarlyLintPass<'ctx> + 'ctx> {
        Box::new(Self { context, imports: Default::default() })
    }

    fn get_name(&self) -> &str {
        "duplicate imports"
    }

    fn check_node(&mut self, node: &SyntaxNode) {
        if let SyntaxKind::Import = node.kind {
            self.check_import(node);
        }
    }
}

impl DuplicateImportsLint<'_> {
    fn check_import(&mut self, node: &SyntaxNode<'_>) {
        match node.children[1].text.strip_suffix(".aleo") {
            Some(id) => {
                let pid = id.to_string();
                if self.imports.contains_key(&pid) {
                    self.context.emit_lint(Lint::duplicate_import(node.children[1].text, node.span));
                } else {
                    _ = self.imports.insert(pid, node.span)
                }
            }
            None => panic!("{} malformed import: '{}' without '.aleo' suffix", node.span, node.children[1].text),
        }
    }
}
