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

use crate::StaticAnalyzer;

use leo_ast::*;

use snarkvm::console::network::Network;

impl<'a, N: Network> ExpressionVisitor<'a> for StaticAnalyzer<'a, N> {
    type AdditionalInput = ();
    type Output = ();

    fn visit_access(&mut self, input: &'a AccessExpression, _: &Self::AdditionalInput) -> Self::Output {
        if let AccessExpression::AssociatedFunction(access) = input {
            // Get the core function.
            let core_function = match CoreFunction::from_symbols(access.variant.name, access.name.name) {
                Some(core_function) => core_function,
                None => unreachable!("Typechecking guarantees that this function exists."),
            };

            // Check that the future was awaited correctly.
            if core_function == CoreFunction::FutureAwait {
                self.assert_future_await(&access.arguments.first(), input.span());
            }
        }
    }

    fn visit_call(&mut self, input: &'a CallExpression, _: &Self::AdditionalInput) -> Self::Output {
        match &*input.function {
            // Note that the parser guarantees that `input.function` is always an identifier.
            Expression::Identifier(ident) => {
                // If the function call is an external async transition, then for all async calls that follow a non-async call,
                // we must check that the async call is not an async function that takes a future as an argument.
                if self.non_async_external_call_seen
                    && self.variant == Some(Variant::AsyncTransition)
                    && input.program.is_some()
                {
                    // Note that this unwrap is safe since we check that `input.program` is `Some` above.
                    self.assert_simple_async_transition_call(input.program.unwrap(), ident.name, input.span());
                }
                // Otherwise look up the function and check if it is a non-async call.
                if let Some(function_symbol) =
                    self.symbol_table.lookup_fn_symbol(Location::new(input.program, ident.name))
                {
                    if function_symbol.variant == Variant::Transition {
                        self.non_async_external_call_seen = true;
                    }
                }
            }
            _ => unreachable!("Parsing guarantees that a function name is always an identifier."),
        }
    }
}
