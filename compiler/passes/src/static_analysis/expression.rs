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

use super::StaticAnalyzingVisitor;

use leo_ast::*;

impl ExpressionVisitor for StaticAnalyzingVisitor<'_> {
    type AdditionalInput = ();
    type Output = ();

    fn visit_associated_function(
        &mut self,
        input: &AssociatedFunctionExpression,
        _additional: &Self::AdditionalInput,
    ) -> Self::Output {
        // Get the core function.
        let Some(core_function) = CoreFunction::from_symbols(input.variant.name, input.name.name) else {
            panic!("Typechecking guarantees that this function exists.");
        };

        // Check that the future was awaited correctly.
        if core_function == CoreFunction::FutureAwait {
            self.assert_future_await(&input.arguments.first(), input.span());
        }
    }

    fn visit_call(&mut self, input: &CallExpression, _: &Self::AdditionalInput) -> Self::Output {
        let Expression::Identifier(ident) = &input.function else {
            panic!("Parsing guarantees that a function name is always an identifier.");
        };

        let caller_program = self.current_program;
        let callee_program = input.program.unwrap_or(caller_program);

        // If the function call is an external async transition, then for all async calls that follow a non-async call,
        // we must check that the async call is not an async function that takes a future as an argument.
        if self.non_async_external_call_seen
            && self.variant == Some(Variant::AsyncTransition)
            && callee_program != caller_program
        {
            self.assert_simple_async_transition_call(callee_program, ident.name, input.span());
        }

        // Look up the function and check if it is a non-async call.
        let function_program = input.program.unwrap_or(self.current_program);

        let func_symbol = self
            .state
            .symbol_table
            .lookup_function(Location::new(function_program, ident.name))
            .expect("Type checking guarantees functions exist.");

        if func_symbol.function.variant == Variant::Transition {
            self.non_async_external_call_seen = true;
        }
    }
}
