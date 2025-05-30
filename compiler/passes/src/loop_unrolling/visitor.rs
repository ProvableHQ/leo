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

use leo_ast::{
    Block,
    ExpressionReconstructor,
    IterationStatement,
    Literal,
    Node,
    NodeID,
    Statement,
    StatementReconstructor,
    Type,
    interpreter_value::LeoValue,
};

use leo_errors::LoopUnrollerError;
use leo_span::{Span, Symbol};

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

    /// Emits a Loop Unrolling Error
    pub fn emit_err(&self, err: LoopUnrollerError) {
        self.state.handler.emit_err(err);
    }

    /// Unrolls an IterationStatement.
    pub fn unroll_iteration_statement(
        &mut self,
        input: IterationStatement,
        start: LeoValue,
        stop: LeoValue,
    ) -> Statement {
        let Some(start) = start.try_as_i128() else {
            self.state.handler.emit_err(LoopUnrollerError::invalid_loop_bound(start, input.span()));
            return Statement::dummy();
        };
        let Some(stop) = stop.try_as_i128() else {
            self.state.handler.emit_err(LoopUnrollerError::invalid_loop_bound(stop, input.span()));
            return Statement::dummy();
        };

        let new_block_id = self.state.node_builder.next_id();

        // Create a block statement to replace the iteration statement.
        self.in_scope(new_block_id, |slf| {
            let unroll_one = |iteration_count| slf.unroll_single_iteration(&input, iteration_count);
            Block {
                span: input.span,
                statements: if input.inclusive {
                    (start..=stop).map(unroll_one).collect()
                } else {
                    (start..stop).map(unroll_one).collect()
                },
                id: new_block_id,
            }
            .into()
        })
    }

    /// A helper function to unroll a single iteration an IterationStatement.
    fn unroll_single_iteration(&mut self, input: &IterationStatement, iteration_index: i128) -> Statement {
        // Construct a new node ID.
        let const_id = self.state.node_builder.next_id();

        let iterator_type =
            self.state.type_table.get(&input.variable.id()).expect("guaranteed to have a type after type checking");

        // Update the type table.
        self.state.type_table.insert(const_id, iterator_type.clone());

        let outer_block_id = self.state.node_builder.next_id();

        let Type::Integer(integer_type) = &iterator_type else {
            panic!("Type checking enforces that the iteration variable is of integer type");
        };
        // Reconstruct `iteration_index` as a `Literal`.
        let value = Literal::integer(*integer_type, iteration_index.to_string(), Default::default(), const_id);

        self.in_scope(outer_block_id, |slf| {
            // Add the loop variable as a constant for the current scope.
            slf.state.symbol_table.insert_const(slf.program, input.variable.name, value.into());

            let duplicated_body =
                super::duplicate::duplicate(input.block.clone(), &mut slf.state.symbol_table, &slf.state.node_builder);

            let result = slf.reconstruct_block(duplicated_body).0.into();

            Block { statements: vec![result], span: input.span(), id: outer_block_id }.into()
        })
    }
}

impl ExpressionReconstructor for UnrollingVisitor<'_> {
    type AdditionalOutput = ();
}
