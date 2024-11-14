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
    CallGraph,
    StructGraph,
    SymbolTable,
    TypeTable,
    VariableSymbol,
    VariableType,
    static_analysis::await_checker::AwaitChecker,
};

use leo_ast::*;
use leo_errors::{StaticAnalyzerError, StaticAnalyzerWarning, emitter::Handler};
use leo_span::{Span, Symbol};

use snarkvm::console::network::Network;

use indexmap::{IndexMap, IndexSet};
use itertools::Itertools;
use std::{cell::RefCell, marker::PhantomData};

pub struct StaticAnalyzer<'a, N: Network> {
    /// The symbol table for the program.
    // Note that this pass does not use the symbol table in a meaningful way.
    // However, this may be useful in future static analysis passes.
    pub(crate) symbol_table: RefCell<SymbolTable>,
    /// The type table for the program.
    // Note that this pass does not use the type table in a meaningful way.
    // However, this may be useful in future static analysis passes.
    pub(crate) type_table: &'a TypeTable,
    /// The error handler.
    pub(crate) handler: &'a Handler,
    /// Struct to store the state relevant to checking all futures are awaited.
    pub(crate) await_checker: AwaitChecker,
    /// The index of the current scope.
    pub(crate) scope_index: usize,
    /// The current program name.
    pub(crate) current_program: Option<Symbol>,
    /// The variant of the function that we are currently traversing.
    pub(crate) variant: Option<Variant>,
    // Allows the type checker to be generic over the network.
    phantom: PhantomData<N>,
}

impl<'a, N: Network> StaticAnalyzer<'a, N> {
    /// Returns a new static analyzer given a symbol table and error handler.
    pub fn new(
        symbol_table: SymbolTable,
        type_table: &'a TypeTable,
        handler: &'a Handler,
        max_depth: usize,
        disabled: bool,
    ) -> Self {
        Self {
            symbol_table: RefCell::new(symbol_table),
            type_table,
            handler,
            await_checker: AwaitChecker::new(max_depth, !disabled),
            scope_index: 0,
            current_program: None,
            variant: None,
            phantom: Default::default(),
        }
    }

    /// Returns the index of the current scope.
    /// Note that if we are in the midst of unrolling an IterationStatement, a new scope is created.
    pub(crate) fn current_scope_index(&mut self) -> usize {
        self.scope_index
    }

    /// Enters a child scope.
    pub(crate) fn enter_scope(&mut self, index: usize) -> usize {
        let previous_symbol_table = std::mem::take(&mut self.symbol_table);
        self.symbol_table.swap(previous_symbol_table.borrow().lookup_scope_by_index(index).unwrap());
        self.symbol_table.borrow_mut().parent = Some(Box::new(previous_symbol_table.into_inner()));

        core::mem::replace(&mut self.scope_index, 0)
    }

    /// Exits the current block scope.
    pub(crate) fn exit_scope(&mut self, index: usize) {
        let prev_st = *self.symbol_table.borrow_mut().parent.take().unwrap();
        self.symbol_table.swap(prev_st.lookup_scope_by_index(index).unwrap());
        self.symbol_table = RefCell::new(prev_st);

        self.scope_index = index + 1;
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
        match self.symbol_table.borrow().lookup_variable(Location::new(None, future_variable.name)) {
            Some(var) => {
                if !matches!(&var.type_, &Type::Future(_)) {
                    self.emit_err(StaticAnalyzerError::expected_future(future_variable.name, future_variable.span()));
                }
                // Mark the future as consumed.
                // If the call returns false, it means that a future was not awaited in the order of the input list, emit a warning.
                if !self.await_checker.remove(future_variable) {
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
}
