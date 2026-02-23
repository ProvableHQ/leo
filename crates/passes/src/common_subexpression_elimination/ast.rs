// Copyright (C) 2019-2026 Provable Inc.
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

use super::CommonSubexpressionEliminatingVisitor;

use leo_ast::*;

impl AstReconstructor for CommonSubexpressionEliminatingVisitor<'_> {
    type AdditionalInput = ();
    type AdditionalOutput = ();

    fn reconstruct_expression(&mut self, input: Expression, _additional: &()) -> (Expression, Self::AdditionalOutput) {
        // We simply forward every expression to `try_expr` rather than using the individual reconstruct
        // functions from the `AstReconstructor` trait.
        (self.try_expr(input, None).expect("CSE Error: Error reconstructing expression").0, Default::default())
    }

    fn reconstruct_block(&mut self, mut block: Block) -> (Block, Self::AdditionalOutput) {
        self.in_scope(|slf| {
            block.statements = block.statements.into_iter().map(|s| slf.reconstruct_statement(s).0).collect();
            (block, Default::default())
        })
    }

    fn reconstruct_definition(&mut self, mut input: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        match input.place {
            DefinitionPlace::Single(place) => {
                let (value, definition_not_needed) = self
                    .try_expr(input.value, Some(place.name))
                    .expect("CSE Error: Error reconstructing single definition");

                if definition_not_needed {
                    // We don't need this definition - everywhere its variable is referred to, we'll map it to some other
                    // Path.
                    (Statement::dummy(), Default::default())
                } else {
                    input.value = value;
                    (input.into(), Default::default())
                }
            }
            DefinitionPlace::Multiple(_) => {
                let (value, _) =
                    self.try_expr(input.value, None).expect("CSE Error: Error reconstructing multiple definitions");
                input.value = value;
                (input.into(), Default::default())
            }
        }
    }

    fn reconstruct_iteration(&mut self, _: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        panic!("`IterationStatement`s should not be in the AST at this phase of compilation.");
    }
}
