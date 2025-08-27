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

use leo_ast::{AstReconstructor, Block, Statement, *};

use crate::SymbolTable;

/// Duplicate this block, recursively giving new `NodeID`s into scopes, and duplicating the new scopes
/// in the `SymbolTable`.
pub fn duplicate(block: Block, symbol_table: &mut SymbolTable, node_builder: &NodeBuilder) -> Block {
    Duplicator { symbol_table, node_builder }.reconstruct_block(block).0
}

struct Duplicator<'a> {
    symbol_table: &'a mut SymbolTable,
    node_builder: &'a NodeBuilder,
}

impl Duplicator<'_> {
    fn in_scope_duped<T>(&mut self, new_id: NodeID, old_id: NodeID, func: impl FnOnce(&mut Self) -> T) -> T {
        self.symbol_table.enter_scope_duped(new_id, old_id);
        let result = func(self);
        self.symbol_table.enter_parent();
        result
    }
}

impl AstReconstructor for Duplicator<'_> {
    type AdditionalInput = ();
    type AdditionalOutput = ();

    /* Statements */
    fn reconstruct_statement(&mut self, input: Statement) -> (Statement, Self::AdditionalOutput) {
        match input {
            Statement::Block(stmt) => {
                let (stmt, output) = self.reconstruct_block(stmt);
                (stmt.into(), output)
            }
            Statement::Conditional(stmt) => self.reconstruct_conditional(stmt),
            Statement::Iteration(stmt) => self.reconstruct_iteration(*stmt),
            stmt => (stmt, Default::default()),
        }
    }

    fn reconstruct_block(&mut self, mut input: Block) -> (Block, Self::AdditionalOutput) {
        let next_id = self.node_builder.next_id();
        self.in_scope_duped(next_id, input.id(), |slf| {
            input.id = next_id;
            input.statements = input.statements.into_iter().map(|stmt| slf.reconstruct_statement(stmt).0).collect();
            (input, Default::default())
        })
    }

    fn reconstruct_conditional(&mut self, mut input: ConditionalStatement) -> (Statement, Self::AdditionalOutput) {
        input.then = self.reconstruct_block(input.then).0;
        if let Some(mut otherwise) = input.otherwise {
            *otherwise = self.reconstruct_statement(*otherwise).0;
            input.otherwise = Some(otherwise);
        }

        (input.into(), Default::default())
    }

    fn reconstruct_iteration(&mut self, mut input: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        let next_id = self.node_builder.next_id();
        self.in_scope_duped(next_id, input.id(), |slf| {
            input.id = next_id;
            input.block = slf.reconstruct_block(input.block).0;
            (input.into(), Default::default())
        })
    }
}
