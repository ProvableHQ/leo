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
    Expression,
    ExpressionReconstructor,
    IterationStatement,
    Literal,
    Node,
    NodeID,
    Statement,
    StatementReconstructor,
    Type,
    Value,
};

use leo_errors::loop_unroller::LoopUnrollerError;
use leo_span::{Span, Symbol};

use super::{Clusivity, LoopBound, RangeIterator};
use crate::CompilerState;

pub struct UnrollingVisitor<'a> {
    pub state: &'a mut CompilerState,
    /// Are we in the midst of unrolling a loop?
    pub is_unrolling: bool,
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
    pub fn unroll_iteration_statement<I: LoopBound>(&mut self, input: IterationStatement) -> Statement {
        let start: Value = input.start_value.borrow().as_ref().expect("Failed to get start value").clone();
        let stop: Value = input.stop_value.borrow().as_ref().expect("Failed to get stop value").clone();

        // Closure to check that the constant values are valid u128.
        // We already know these are integers since loop unrolling occurs after type checking.
        let cast_to_number = |v: Value| -> Result<I, Statement> {
            match v.try_into() {
                Ok(val_as_u128) => Ok(val_as_u128),
                Err(err) => {
                    self.state.handler.emit_err(err);
                    Err(Statement::dummy())
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

        let new_block_id = self.state.node_builder.next_id();

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
        let const_id = self.state.node_builder.next_id();
        // Update the type table.
        self.state.type_table.insert(const_id, input.type_.clone());

        let outer_block_id = self.state.node_builder.next_id();

        // Reconstruct `iteration_count` as a `Literal`.
        let Type::Integer(integer_type) = &input.type_ else {
            unreachable!("Type checking enforces that the iteration variable is of integer type");
        };

        self.in_scope(outer_block_id, |slf| {
            let value = Literal::Integer(*integer_type, iteration_count.to_string(), Default::default(), const_id);

            // Add the loop variable as a constant for the current scope.
            slf.state.symbol_table.insert_const(slf.program, input.variable.name, Expression::Literal(value));

            let duplicated_body =
                super::duplicate::duplicate(input.block.clone(), &mut slf.state.symbol_table, &slf.state.node_builder);

            let prior_is_unrolling = slf.is_unrolling;
            slf.is_unrolling = true;

            let result = Statement::Block(slf.reconstruct_block(duplicated_body).0);

            slf.is_unrolling = prior_is_unrolling;

            Statement::Block(Block { statements: vec![result], span: input.span(), id: outer_block_id })
        })
    }
}

impl ExpressionReconstructor for UnrollingVisitor<'_> {
    type AdditionalOutput = ();
}
