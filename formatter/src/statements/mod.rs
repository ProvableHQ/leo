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

mod assert;
mod assert_eq;
mod assert_neq;
mod assign;
mod block;
mod condtional;
mod const_;
mod definition;
mod expression;
mod iteration;
mod return_statement;

use leo_parser_lossless::{StatementKind, SyntaxKind, SyntaxNode};

use crate::{Formatter, Output};

impl Formatter<'_, '_> {
    pub(crate) fn format_statements(&mut self, statements: &[SyntaxNode<'_>]) -> Output {
        for (i, statement) in statements.iter().enumerate() {
            assert!(matches!(statement.kind, SyntaxKind::Statement(_)));
            let is_last = i == statements.len() - 1;
            self.format_statement(statement, !is_last)?;
            self.maybe_bump_line_else_ignore()?;
        }

        Ok(())
    }

    pub(crate) fn format_statement(&mut self, statement: &SyntaxNode, trailing_empty_lines: bool) -> Output {
        let SyntaxKind::Statement(kind) = statement.kind else { panic!("Can't happen") };
        match kind {
            StatementKind::Assert => self.format_assert(statement, trailing_empty_lines)?,
            StatementKind::AssertEq => self.format_assert_eq(statement, trailing_empty_lines)?,
            StatementKind::AssertNeq => self.format_assert_neq(statement, trailing_empty_lines)?,
            StatementKind::Assign => self.format_assign(statement, trailing_empty_lines)?,
            StatementKind::Block => self.format_block(statement, trailing_empty_lines)?,
            StatementKind::Conditional => self.format_conditional(statement, trailing_empty_lines)?,
            StatementKind::Const => self.format_const(statement, trailing_empty_lines)?,
            StatementKind::Definition => self.format_definition(statement, trailing_empty_lines)?,
            StatementKind::Expression => self.format_expression_statement(statement, trailing_empty_lines)?,
            StatementKind::Iteration => self.format_iteration(statement, trailing_empty_lines)?,
            StatementKind::Return => self.format_return(statement, trailing_empty_lines)?,
        }

        Ok(())
    }
}
