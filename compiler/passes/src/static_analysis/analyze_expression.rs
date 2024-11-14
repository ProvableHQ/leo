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

use crate::{StaticAnalyzer, VariableSymbol};

use leo_ast::*;
use leo_errors::{StaticAnalyzerError, emitter::Handler};
use leo_span::{Span, Symbol, sym};

use snarkvm::console::network::Network;

use itertools::Itertools;

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
                todo!()
            }
            _ => unreachable!("Parsing guarantees that a function name is always an identifier."),
        }
    }
}
