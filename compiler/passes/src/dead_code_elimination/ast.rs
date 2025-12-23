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

use super::DeadCodeEliminatingVisitor;

use leo_ast::*;

impl AstReconstructor for DeadCodeEliminatingVisitor<'_> {
    type AdditionalInput = ();
    type AdditionalOutput = ();

    /* Expressions */
    // Use and reconstruct a path.
    fn reconstruct_path(&mut self, input: Path, _additional: &()) -> (Expression, Self::AdditionalOutput) {
        // At this stage, all `Path`s should refer to local variables or mappings, so it's safe to
        // refer to them using the last symbol in the path.
        self.used_variables.insert(input.identifier().name);
        (input.into(), Default::default())
    }

    // We need to make sure we hit identifiers, so do our own traversal
    // rather than relying on the default.
    fn reconstruct_composite_init(
        &mut self,
        mut input: leo_ast::CompositeExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        for member in input.members.iter_mut() {
            if let Some(expr) = std::mem::take(&mut member.expression) {
                member.expression = Some(self.reconstruct_expression(expr, &()).0);
            } else {
                // We're not actually going to modify it.
                self.reconstruct_path(Path::from(member.identifier).to_local(), &());
            }
        }

        (input.into(), Default::default())
    }

    /* Statements */
    /// Reconstruct an assignment statement by eliminating any dead code.
    fn reconstruct_assign(&mut self, _input: AssignStatement) -> (Statement, Self::AdditionalOutput) {
        panic!("`AssignStatement`s should not exist in the AST at this phase of compilation.")
    }

    /// Reconstructs the statements inside a basic block, eliminating any dead code.
    fn reconstruct_block(&mut self, block: Block) -> (Block, Self::AdditionalOutput) {
        // Don't count empty blocks as statements, as that would be a bit misleading to the user as
        // to how much the code is being transformed.
        self.statements_before += block.statements.iter().filter(|stmt| !stmt.is_empty()).count() as u32;

        // Reconstruct each of the statements in reverse.
        let mut statements: Vec<Statement> =
            block.statements.into_iter().rev().map(|statement| self.reconstruct_statement(statement).0).collect();

        statements.retain(|stmt| !stmt.is_empty());

        // Reverse the direction of `statements`.
        statements.reverse();

        self.statements_after += statements.len() as u32;

        (Block { statements, span: block.span, id: block.id }, Default::default())
    }

    /// Static single assignment replaces definition statements with assignment statements.
    fn reconstruct_definition(&mut self, mut input: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        // Check the lhs of the definition to see any of variables are used.
        let lhs_is_used = match &input.place {
            DefinitionPlace::Single(identifier) => self.used_variables.contains(&identifier.name),
            DefinitionPlace::Multiple(identifiers) => {
                identifiers.iter().any(|identifier| self.used_variables.contains(&identifier.name))
            }
        };

        if !lhs_is_used && self.is_pure(&input.value) {
            // We can eliminate this statement.
            (Statement::dummy(), Default::default())
        } else {
            // We still need it.
            input.value = self.reconstruct_expression(input.value, &()).0;
            (input.into(), Default::default())
        }
    }

    /// Loop unrolling unrolls and removes iteration statements from the program.
    fn reconstruct_iteration(&mut self, _: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        panic!("`IterationStatement`s should not be in the AST at this phase of compilation.");
    }

    fn reconstruct_expression_statement(&mut self, input: ExpressionStatement) -> (Statement, Self::AdditionalOutput) {
        if self.is_pure(&input.expression) {
            (Statement::dummy(), Default::default())
        } else {
            (
                ExpressionStatement { expression: self.reconstruct_expression(input.expression, &()).0, ..input }
                    .into(),
                Default::default(),
            )
        }
    }
}
