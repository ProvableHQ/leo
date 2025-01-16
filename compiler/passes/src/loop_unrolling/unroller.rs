// Copyright (C) 2019-2025 Aleo Systems Inc.
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
    Expression,
    ExpressionReconstructor,
    IterationStatement,
    Literal,
    Node,
    NodeBuilder,
    NodeID,
    Statement,
    StatementReconstructor,
    Type,
    Value,
};

use leo_errors::{emitter::Handler, loop_unroller::LoopUnrollerError};
use leo_span::{Span, Symbol};

use crate::{Clusivity, LoopBound, RangeIterator, SymbolTable, TypeTable};

pub struct Unroller<'a> {
    /// The symbol table for the function being processed.
    pub(crate) symbol_table: &'a mut SymbolTable,
    /// A mapping from node IDs to their types.
    pub(crate) type_table: &'a TypeTable,
    /// An error handler used for any errors found during unrolling.
    pub(crate) handler: &'a Handler,
    /// A counter used to generate unique node IDs.
    pub(crate) node_builder: &'a NodeBuilder,
    /// Are we in the midst of unrolling a loop?
    pub(crate) is_unrolling: bool,
    /// The current program name.
    pub(crate) current_program: Option<Symbol>,
    /// If we've encountered a loop that was not unrolled, here's it's spanned.
    pub(crate) loop_not_unrolled: Option<Span>,
    /// Have we unrolled any loop?
    pub(crate) loop_unrolled: bool,
}

impl<'a> Unroller<'a> {
    pub(crate) fn new(
        symbol_table: &'a mut SymbolTable,
        type_table: &'a TypeTable,
        handler: &'a Handler,
        node_builder: &'a NodeBuilder,
    ) -> Self {
        Self {
            symbol_table,
            type_table,
            handler,
            node_builder,
            is_unrolling: false,
            current_program: None,
            loop_not_unrolled: None,
            loop_unrolled: false,
        }
    }

    pub(crate) fn in_scope<T>(&mut self, id: NodeID, func: impl FnOnce(&mut Self) -> T) -> T {
        self.symbol_table.enter_scope(Some(id));
        let result = func(self);
        self.symbol_table.enter_parent();
        result
    }

    /// Emits a Loop Unrolling Error
    pub(crate) fn emit_err(&self, err: LoopUnrollerError) {
        self.handler.emit_err(err);
    }

    /// Unrolls an IterationStatement.
    pub(crate) fn unroll_iteration_statement<I: LoopBound>(&mut self, input: IterationStatement) -> Statement {
        let start: Value = input.start_value.borrow().as_ref().expect("Failed to get start value").clone();
        let stop: Value = input.stop_value.borrow().as_ref().expect("Failed to get stop value").clone();

        // Closure to check that the constant values are valid u128.
        // We already know these are integers since loop unrolling occurs after type checking.
        let cast_to_number = |v: Value| -> Result<I, Statement> {
            match v.try_into() {
                Ok(val_as_u128) => Ok(val_as_u128),
                Err(err) => {
                    self.handler.emit_err(err);
                    Err(Statement::dummy(input.span, self.node_builder.next_id()))
                }
            }
        };

        // Cast `start` to `I`.
        let start = match cast_to_number(start) {
            Ok(v) => v,
            Err(s) => return s,
        };
        // Cast `stop` to `I`.
        let stop = match cast_to_number(stop) {
            Ok(v) => v,
            Err(s) => return s,
        };

        let new_block_id = self.node_builder.next_id();

        // Create a block statement to replace the iteration statement.
        self.in_scope(new_block_id, |slf| {
            Statement::Block(Block {
                span: input.span,
                statements: match input.inclusive {
                    true => {
                        let iter = RangeIterator::new(start, stop, Clusivity::Inclusive);
                        iter.map(|iteration_count| slf.unroll_single_iteration(&input, iteration_count)).collect()
                    }
                    false => {
                        let iter = RangeIterator::new(start, stop, Clusivity::Exclusive);
                        iter.map(|iteration_count| slf.unroll_single_iteration(&input, iteration_count)).collect()
                    }
                },
                id: new_block_id,
            })
        })
    }

    /// A helper function to unroll a single iteration an IterationStatement.
    fn unroll_single_iteration<I: LoopBound>(&mut self, input: &IterationStatement, iteration_count: I) -> Statement {
        // Construct a new node ID.
        let const_id = self.node_builder.next_id();
        // Update the type table.
        self.type_table.insert(const_id, input.type_.clone());

        let outer_block_id = self.node_builder.next_id();

        // Reconstruct `iteration_count` as a `Literal`.
        let Type::Integer(integer_type) = &input.type_ else {
            unreachable!("Type checking enforces that the iteration variable is of integer type");
        };

        self.in_scope(outer_block_id, |slf| {
            let value = Literal::Integer(*integer_type, iteration_count.to_string(), Default::default(), const_id);

            // Add the loop variable as a constant for the current scope.
            slf.symbol_table.insert_const(
                slf.current_program.unwrap(),
                input.variable.name,
                Expression::Literal(value),
            );

            let duplicated_body = super::duplicate::duplicate(input.block.clone(), slf.symbol_table, slf.node_builder);

            let prior_is_unrolling = slf.is_unrolling;
            slf.is_unrolling = true;

            let result = Statement::Block(slf.reconstruct_block(duplicated_body).0);

            slf.is_unrolling = prior_is_unrolling;

            Statement::Block(Block { statements: vec![result], span: input.span(), id: outer_block_id })
        })
    }
}

impl ExpressionReconstructor for Unroller<'_> {
    type AdditionalOutput = ();
}
