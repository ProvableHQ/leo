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

use leo_ast::{
    Block,
    Expression,
    IntegerType,
    IterationStatement,
    Literal,
    NodeBuilder,
    Statement,
    StatementReconstructor,
    Type,
    Value,
};
use std::cell::RefCell;

use leo_errors::{emitter::Handler, loop_unroller::LoopUnrollerError};

use crate::{
    constant_propagation_table::ConstantPropagationTable,
    Clusivity,
    LoopBound,
    RangeIterator,
    SymbolTable,
    TypeTable,
};

pub struct Unroller<'a> {
    /// A table of constant variables.
    pub(crate) constant_propagation_table: RefCell<ConstantPropagationTable>,
    /// The symbol table for the function being processed.
    pub(crate) symbol_table: RefCell<SymbolTable>,
    /// A mapping from node IDs to their types.
    pub(crate) type_table: &'a TypeTable,
    /// The index of the current scope.
    pub(crate) scope_index: usize,
    /// An error handler used for any errors found during unrolling.
    pub(crate) handler: &'a Handler,
    /// A counter used to generate unique node IDs.
    pub(crate) node_builder: &'a NodeBuilder,
    /// Are we in the midst of unrolling a loop?
    pub(crate) is_unrolling: bool,
}

impl<'a> Unroller<'a> {
    pub(crate) fn new(
        symbol_table: SymbolTable,
        type_table: &'a TypeTable,
        handler: &'a Handler,
        node_builder: &'a NodeBuilder,
    ) -> Self {
        Self {
            constant_propagation_table: RefCell::new(ConstantPropagationTable::default()),
            symbol_table: RefCell::new(symbol_table),
            type_table,
            scope_index: 0,
            handler,
            node_builder,
            is_unrolling: false,
        }
    }

    /// Returns the index of the current scope.
    /// Note that if we are in the midst of unrolling an IterationStatement, a new scope is created.
    pub(crate) fn current_scope_index(&mut self) -> usize {
        if self.is_unrolling { self.symbol_table.borrow_mut().insert_block() } else { self.scope_index }
    }

    /// Enters a child scope.
    pub(crate) fn enter_scope(&mut self, index: usize) -> usize {
        let previous_symbol_table = std::mem::take(&mut self.symbol_table);
        self.symbol_table.swap(previous_symbol_table.borrow().lookup_scope_by_index(index).unwrap());
        self.symbol_table.borrow_mut().parent = Some(Box::new(previous_symbol_table.into_inner()));

        // Can assume that CPT has not constructed any scoping yet, since have never seen this scope before
        self.constant_propagation_table.borrow_mut().insert_block();

        // Build CPT recursive scoping structure just like symbol table
        let previous_constant_propagation_table = std::mem::take(&mut self.constant_propagation_table);
        self.constant_propagation_table
            .swap(previous_constant_propagation_table.borrow().lookup_scope_by_index(index).unwrap());
        self.constant_propagation_table.borrow_mut().parent =
            Some(Box::new(previous_constant_propagation_table.into_inner()));
        core::mem::replace(&mut self.scope_index, 0)
    }

    /// Exits the current block scope.
    pub(crate) fn exit_scope(&mut self, index: usize) {
        let prev_ct = *self.constant_propagation_table.borrow_mut().parent.take().unwrap();
        self.constant_propagation_table.swap(prev_ct.lookup_scope_by_index(index).unwrap());
        self.constant_propagation_table = RefCell::new(prev_ct);
        let prev_st = *self.symbol_table.borrow_mut().parent.take().unwrap();
        self.symbol_table.swap(prev_st.lookup_scope_by_index(index).unwrap());
        self.symbol_table = RefCell::new(prev_st);
        self.scope_index = index + 1;
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

        // Get the index of the current scope.
        let scope_index = self.current_scope_index();

        // Enter the scope of the loop body.
        let previous_scope_index = self.enter_scope(scope_index);

        // Clear the symbol table and constant propagation table for the loop body.
        // This is necessary because loop unrolling transforms the program, which requires reconstructing the tables
        self.symbol_table.borrow_mut().variables.clear();
        self.symbol_table.borrow_mut().scopes.clear();
        self.symbol_table.borrow_mut().scope_index = 0;

        // Create a block statement to replace the iteration statement.
        // Creates a new block per iteration inside the outer block statement.
        let iter_blocks = Statement::Block(Block {
            span: input.span,
            statements: match input.inclusive {
                true => {
                    let iter = RangeIterator::new(start, stop, Clusivity::Inclusive);
                    iter.map(|iteration_count| self.unroll_single_iteration(&input, iteration_count)).collect()
                }
                false => {
                    let iter = RangeIterator::new(start, stop, Clusivity::Exclusive);
                    iter.map(|iteration_count| self.unroll_single_iteration(&input, iteration_count)).collect()
                }
            },
            id: input.id,
        });

        // Exit the scope of the loop body.
        self.exit_scope(previous_scope_index);

        iter_blocks
    }

    /// A helper function to unroll a single iteration an IterationStatement.
    fn unroll_single_iteration<I: LoopBound>(&mut self, input: &IterationStatement, iteration_count: I) -> Statement {
        // Create a scope for a single unrolling of the `IterationStatement`.
        let scope_index = self.symbol_table.borrow_mut().insert_block();
        let previous_scope_index = self.enter_scope(scope_index);

        let prior_is_unrolling = self.is_unrolling;
        self.is_unrolling = true;

        // Construct a new node ID.
        let id = self.node_builder.next_id();
        // Update the type table.
        self.type_table.insert(id, input.type_.clone());

        // Reconstruct `iteration_count` as a `Literal`.
        let value = match input.type_ {
            Type::Integer(IntegerType::I8) => {
                Literal::Integer(IntegerType::I8, iteration_count.to_string(), Default::default(), id)
            }
            Type::Integer(IntegerType::I16) => {
                Literal::Integer(IntegerType::I16, iteration_count.to_string(), Default::default(), id)
            }
            Type::Integer(IntegerType::I32) => {
                Literal::Integer(IntegerType::I32, iteration_count.to_string(), Default::default(), id)
            }
            Type::Integer(IntegerType::I64) => {
                Literal::Integer(IntegerType::I64, iteration_count.to_string(), Default::default(), id)
            }
            Type::Integer(IntegerType::I128) => {
                Literal::Integer(IntegerType::I128, iteration_count.to_string(), Default::default(), id)
            }
            Type::Integer(IntegerType::U8) => {
                Literal::Integer(IntegerType::U8, iteration_count.to_string(), Default::default(), id)
            }
            Type::Integer(IntegerType::U16) => {
                Literal::Integer(IntegerType::U16, iteration_count.to_string(), Default::default(), id)
            }
            Type::Integer(IntegerType::U32) => {
                Literal::Integer(IntegerType::U32, iteration_count.to_string(), Default::default(), id)
            }
            Type::Integer(IntegerType::U64) => {
                Literal::Integer(IntegerType::U64, iteration_count.to_string(), Default::default(), id)
            }
            Type::Integer(IntegerType::U128) => {
                Literal::Integer(IntegerType::U128, iteration_count.to_string(), Default::default(), id)
            }
            _ => unreachable!(
                "The iteration variable must be an integer type. This should be enforced by type checking."
            ),
        };

        // Add the loop variable as a constant for the current scope
        self.constant_propagation_table
            .borrow_mut()
            .insert_constant(input.variable.name, Expression::Literal(value.clone()))
            .expect("Failed to insert constant into CPT");

        // Reconstruct the statements in the loop body.
        let statements: Vec<_> = input
            .block
            .statements
            .clone()
            .into_iter()
            .filter_map(|s| {
                let (reconstructed_statement, additional_output) = self.reconstruct_statement(s);
                if additional_output {
                    None // Exclude this statement from the block since it is a constant variable definition
                } else {
                    Some(reconstructed_statement)
                }
            })
            .collect();

        let block = Statement::Block(Block { statements, span: input.block.span, id: input.block.id });

        self.is_unrolling = prior_is_unrolling;

        // Exit the scope.
        self.exit_scope(previous_scope_index);

        block
    }
}
