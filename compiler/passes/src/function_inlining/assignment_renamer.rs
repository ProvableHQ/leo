// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use crate::{Assigner, RenameTable};
use leo_ast::{
    AssignStatement,
    ConditionalStatement,
    ConsoleStatement,
    DefinitionStatement,
    Expression,
    ExpressionReconstructor,
    Identifier,
    IterationStatement,
    NodeID,
    ProgramReconstructor,
    Statement,
    StatementReconstructor,
    StructExpression,
    StructVariableInitializer,
};
use leo_span::Symbol;

// TODO: Generalize the functionality of this reconstructor to be used in other passes.
/// An `AssignmentRenamer` renames the left-hand side of all assignment statements in an AST node.
/// The new names are propagated to all following identifiers.
pub struct AssignmentRenamer<'a> {
    pub assigner: &'a Assigner,
    pub rename_table: RenameTable,
    pub is_lhs: bool,
}

impl<'a> AssignmentRenamer<'a> {
    /// Initialize a new `AssignmentRenamer`.
    pub fn new(assigner: &'a Assigner) -> Self {
        Self { assigner, rename_table: RenameTable::new(None), is_lhs: false }
    }

    /// Load the internal rename table with a set of entries.
    pub fn load(&mut self, entries: impl Iterator<Item = (Symbol, Symbol, NodeID)>) {
        for (key, value, id) in entries {
            self.rename_table.update(key, value, id);
        }
    }

    /// Clear the internal rename table.
    pub fn clear(&mut self) {
        self.rename_table = RenameTable::new(None);
    }
}

impl ExpressionReconstructor for AssignmentRenamer<'_> {
    type AdditionalOutput = ();

    /// Rename the identifier if it is the left-hand side of an assignment, otherwise look up for a new name in the internal rename table.
    fn reconstruct_identifier(&mut self, input: Identifier) -> (Expression, Self::AdditionalOutput) {
        let name = match self.is_lhs {
            // If consuming the left-hand side of an assignment, a new unique name is introduced.
            true => {
                let new_name = self.assigner.unique_symbol(input.name, "$");
                self.rename_table.update(input.name, new_name, input.id);
                new_name
            }
            // Otherwise, we look up the previous name in the `RenameTable`.
            // Note that we do not panic if the identifier is not found in the rename table.
            // Variables that do not exist in the rename table are ones that have been introduced during the SSA pass.
            // These variables are never re-assigned, and will never have an entry in the rename-table.
            false => *self.rename_table.lookup(input.name).unwrap_or(&input.name),
        };

        (Expression::Identifier(Identifier { name, span: input.span, id: input.id }), Default::default())
    }

    /// Rename the variable initializers in the struct expression.
    fn reconstruct_struct_init(&mut self, input: StructExpression) -> (Expression, Self::AdditionalOutput) {
        (
            Expression::Struct(StructExpression {
                name: input.name,
                members: input
                    .members
                    .into_iter()
                    .map(|member| StructVariableInitializer {
                        identifier: member.identifier,
                        expression: match member.expression {
                            Some(expression) => Some(self.reconstruct_expression(expression).0),
                            None => unreachable!(
                                "SSA guarantees that all struct members are always of the form `<id> : <expr>`."
                            ),
                        },
                        span: member.span,
                        id: member.id,
                    })
                    .collect(),
                span: input.span,
                id: input.id,
            }),
            Default::default(),
        )
    }
}

impl StatementReconstructor for AssignmentRenamer<'_> {
    /// Rename the left-hand side of the assignment statement.
    fn reconstruct_assign(&mut self, input: AssignStatement) -> (Statement, Self::AdditionalOutput) {
        // First rename the right-hand-side of the assignment.
        let value = self.reconstruct_expression(input.value).0;

        // Then assign a new unique name to the left-hand-side of the assignment.
        // Note that this order is necessary to ensure that the right-hand-side uses the correct name when consuming a complex assignment.
        self.is_lhs = true;
        let place = self.reconstruct_expression(input.place).0;
        self.is_lhs = false;

        (
            Statement::Assign(Box::new(AssignStatement { place, value, span: input.span, id: input.id })),
            Default::default(),
        )
    }

    /// Flattening removes conditional statements from the program.
    fn reconstruct_conditional(&mut self, _: ConditionalStatement) -> (Statement, Self::AdditionalOutput) {
        unreachable!("`ConditionalStatement`s should not be in the AST at this phase of compilation.")
    }

    /// Parsing guarantees that console statements are not present in the program.
    fn reconstruct_console(&mut self, _: ConsoleStatement) -> (Statement, Self::AdditionalOutput) {
        unreachable!("`ConsoleStatement`s should not be in the AST at this phase of compilation.")
    }

    /// Static single assignment replaces definition statements with assignment statements.
    fn reconstruct_definition(&mut self, _: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        unreachable!("`DefinitionStatement`s should not exist in the AST at this phase of compilation.")
    }

    /// Loop unrolling unrolls and removes iteration statements from the program.
    fn reconstruct_iteration(&mut self, _: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        unreachable!("`IterationStatement`s should not be in the AST at this phase of compilation.");
    }
}

impl ProgramReconstructor for AssignmentRenamer<'_> {}
