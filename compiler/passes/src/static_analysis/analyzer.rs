// Copyright (C) 2019-2024 Aleo Systems Inc.
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

use crate::{
    SymbolTable,
    TypeTable,
    static_analysis::await_checker::AwaitChecker,
};

use leo_ast::*;
use leo_errors::{StaticAnalyzerError, StaticAnalyzerWarning, emitter::Handler};
use leo_span::{Span, Symbol};

use snarkvm::console::network::Network;

use std::{marker::PhantomData};

pub struct StaticAnalyzer<'a, N: Network> {
    /// The symbol table for the program.
    pub(crate) symbol_table: &'a SymbolTable,
    /// The type table for the program.
    // Note that this pass does not use the type table in a meaningful way.
    // However, this may be useful in future static analysis passes.
    pub(crate) type_table: &'a TypeTable,
    /// The error handler.
    pub(crate) handler: &'a Handler,
    /// Struct to store the state relevant to checking all futures are awaited.
    pub(crate) await_checker: AwaitChecker,
    /// The current program name.
    pub(crate) current_program: Option<Symbol>,
    /// The variant of the function that we are currently traversing.
    pub(crate) variant: Option<Variant>,
    /// Whether or not a non-async external call has been seen in this function.
    pub(crate) non_async_external_call_seen: bool,
    // Allows the type checker to be generic over the network.
    phantom: PhantomData<N>,
}

impl<'a, N: Network> StaticAnalyzer<'a, N> {
    /// Returns a new static analyzer given a symbol table and error handler.
    pub fn new(
        symbol_table: &'a SymbolTable,
        _type_table: &'a TypeTable,
        handler: &'a Handler,
        max_depth: usize,
        disabled: bool,
    ) -> Self {
        Self {
            symbol_table,
            type_table: _type_table,
            handler,
            await_checker: AwaitChecker::new(max_depth, !disabled),
            current_program: None,
            variant: None,
            non_async_external_call_seen: false,
            phantom: Default::default(),
        }
    }

    /// Emits a type checker error.
    pub(crate) fn emit_err(&self, err: StaticAnalyzerError) {
        self.handler.emit_err(err);
    }

    /// Emits a type checker warning
    pub fn emit_warning(&self, warning: StaticAnalyzerWarning) {
        self.handler.emit_warning(warning.into());
    }

    /// Type checks the awaiting of a future.
    pub(crate) fn assert_future_await(&mut self, future: &Option<&Expression>, span: Span) {
        // Make sure that it is an identifier expression.
        let future_variable = match future {
            Some(Expression::Identifier(name)) => name,
            _ => {
                return self.emit_err(StaticAnalyzerError::invalid_await_call(span));
            }
        };

        // Make sure that the future is defined.
        match self.type_table.get(&future_variable.id) {
            Some(type_) => {
                if !matches!(type_, Type::Future(_)) {
                    self.emit_err(StaticAnalyzerError::expected_future(future_variable.name, future_variable.span()));
                }
                // Mark the future as consumed.
                // If the call returns true, it means that a future was not awaited in the order of the input list, emit a warning.
                if self.await_checker.remove(future_variable) {
                    self.emit_warning(StaticAnalyzerWarning::future_not_awaited_in_order(
                        future_variable.name,
                        future_variable.span(),
                    ));
                }
            }
            None => {
                self.emit_err(StaticAnalyzerError::expected_future(future_variable.name, future_variable.span()));
            }
        }
    }

    /// Assert that an async call is a "simple" one.
    /// Simple is defined as an async transition function which does not return a `Future` that itself takes a `Future` as an argument.
    pub(crate) fn assert_simple_async_transition_call(&self, program: Symbol, function_name: Symbol, span: Span) {
        // Note: The function symbol lookup is performed outside of the `if let Some(func) ...` block to avoid a RefCell lifetime bug in Rust.
        // Do not move it into the `if let Some(func) ...` block or it will keep `self.symbol_table_creation` alive for the entire block and will be very memory inefficient!
        if let Some(function) = self.symbol_table.lookup_fn_symbol(Location::new(Some(program), function_name))
        {
            // Check that the function is an async function.
            if function.variant != Variant::AsyncFunction {
                return;
            }

            // A helper function to check if a `Future` type takes a `Future` as an argument.
            let check_future = |future_type: &FutureType| {
                if future_type.inputs.iter().any(|input| matches!(input, Type::Future(_))) {
                    self.emit_err(StaticAnalyzerError::async_transition_call_with_future_argument(function_name, span));
                }
            };

            // Check the output type of the function.
            match &function.output_type {
                Type::Future(future_type) => check_future(&future_type),
                Type::Tuple(tuple_type) => {
                    for element in tuple_type.elements() {
                        if let Type::Future(future_type) = element {
                            check_future(&future_type);
                        }
                    }
                }
                _ => (), // Do nothing.
            }
        } else {
            unreachable!("Type checking guarantees that this function exists.")
        }
    }
}
