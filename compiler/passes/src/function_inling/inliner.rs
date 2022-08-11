// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use crate::{CallType, SymbolTable};

use leo_ast::{CallExpression, Expression, ExpressionReconstructor, ProgramReconstructor, StatementReconstructor};
use leo_errors::emitter::Handler;

use std::cell::RefCell;

pub struct Inliner<'a> {
    /// The symbol table for the function being processed.
    pub(crate) symbol_table: RefCell<SymbolTable>,
    /// The index of the current block scope.
    pub(crate) _block_index: usize,
    /// An error handler used for any errors found during unrolling.
    pub(crate) _handler: &'a Handler,
}

impl<'a> Inliner<'a> {
    pub(crate) fn new(symbol_table: SymbolTable, _handler: &'a Handler) -> Self {
        Self {
            symbol_table: RefCell::new(symbol_table),
            _block_index: 0,
            _handler,
        }
    }
}

impl ExpressionReconstructor for Inliner<'_> {
    type AdditionalOutput = ();

    fn reconstruct_call(&mut self, input: CallExpression) -> (Expression, Self::AdditionalOutput) {
        // Reconstruct the function expression.
        let function = self.reconstruct_expression(*input.function).0;

        // If the function being called must be inlined, then inline it.
        match &function {
            Expression::Identifier(identifier) => {
                // Lookup the function's call type.
                // Note that the `unwrap` is safe, since type checking ensures that the function is defined.
                let call_type = self
                    .symbol_table
                    .borrow()
                    .lookup_fn_symbol(identifier.name)
                    .unwrap()
                    .call_type;
                if call_type == CallType::Inlined {
                    todo!("Function inlining is not yet implemented.");
                }
            }
            _ => unreachable!("The function must always be an identifier."),
        }

        (
            Expression::Call(CallExpression {
                function: Box::new(function),
                arguments: input
                    .arguments
                    .into_iter()
                    .map(|arg| self.reconstruct_expression(arg).0)
                    .collect(),
                span: input.span,
            }),
            Default::default(),
        )
    }
}

impl StatementReconstructor for Inliner<'_> {}

impl ProgramReconstructor for Inliner<'_> {}
