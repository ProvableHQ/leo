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

use leo_ast::{AstReconstructor, Block, IterationStatement, Literal, Node, NodeID, Statement, Type, const_eval::Value};
use leo_errors::LoopUnrollerError;
use leo_span::{Span, Symbol};

use itertools::Either;

use crate::CompilerState;

pub struct UnrollingVisitor<'a> {
    pub state: &'a mut CompilerState,
    /// The current program name.
    pub program: Symbol,
    /// If we've encountered a loop that was not unrolled, here's it's spanned.
    pub loop_not_unrolled: Option<Span>,
    /// Have we unrolled any loop?
    pub loop_unrolled: bool,
}

impl UnrollingVisitor<'_> {
    pub fn in_scope<T>(&mut self, id: NodeID, func: impl FnOnce(&mut Self) -> T) -> T {
        self.state.symbol_table.enter_scope(Some(id));
        let result = func(self);
        self.state.symbol_table.enter_parent();
        result
    }

    /// Unrolls an IterationStatement.
    pub fn unroll_iteration_statement(&mut self, input: IterationStatement, start: Value, stop: Value) -> Statement {
        // We already know these are integers since loop unrolling occurs after type checking.
        let cast_to_number = |v: Value| -> Result<i128, Statement> {
            match v.as_i128() {
                Some(val_as_i128) => Ok(val_as_i128),
                None => {
                    self.state.handler.emit_err(LoopUnrollerError::value_out_of_i128_bounds(v, input.span()));
                    Err(Statement::dummy())
                }
            }
        };

        // Cast `start` to `i128`.
        let start = match cast_to_number(start) {
            Ok(v) => v,
            Err(s) => return s,
        };
        // Cast `stop` to `i128`.
        let stop = match cast_to_number(stop) {
            Ok(v) => v,
            Err(s) => return s,
        };

        let new_block_id = self.state.node_builder.next_id();

        let iter = if input.inclusive { Either::Left(start..=stop) } else { Either::Right(start..stop) };

        // Create a block statement to replace the iteration statement.
        self.in_scope(new_block_id, |slf| {
            Block {
                span: input.span,
                statements: iter.map(|iteration_count| slf.unroll_single_iteration(&input, iteration_count)).collect(),
                id: new_block_id,
            }
            .into()
        })
    }

    /// A helper function to unroll a single iteration an IterationStatement.
    fn unroll_single_iteration(&mut self, input: &IterationStatement, iteration_count: i128) -> Statement {
        // Construct a new node ID.
        let const_id = self.state.node_builder.next_id();

        let iterator_type =
            self.state.type_table.get(&input.variable.id()).expect("guaranteed to have a type after type checking");

        // Update the type table.
        self.state.type_table.insert(const_id, iterator_type.clone());

        let outer_block_id = self.state.node_builder.next_id();

        // Reconstruct `iteration_count` as a `Literal`.
        let Type::Integer(integer_type) = &iterator_type else {
            unreachable!("Type checking enforces that the iteration variable is of integer type");
        };

        self.in_scope(outer_block_id, |slf| {
            let value = Literal::integer(*integer_type, iteration_count.to_string(), Default::default(), const_id);

            // Add the loop variable as a constant for the current scope.
            slf.state.symbol_table.insert_local_const(input.variable.name, value.into());

            let duplicated_body =
                super::duplicate::duplicate(input.block.clone(), &mut slf.state.symbol_table, &slf.state.node_builder);

            let result = slf.reconstruct_block(duplicated_body).0.into();

            Block { statements: vec![result], span: input.span(), id: outer_block_id }.into()
        })
    }
}
