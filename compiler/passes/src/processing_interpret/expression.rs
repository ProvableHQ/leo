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

use super::ProcessingInterpretVisitor;

use leo_ast::{CallExpression, Expression, ExpressionReconstructor, Location, Variant};
use leo_errors::TypeCheckerError;

impl ExpressionReconstructor for ProcessingInterpretVisitor<'_> {
    type AdditionalOutput = ();

    fn reconstruct_call(&mut self, input: CallExpression) -> (Expression, Self::AdditionalOutput) {
        if !matches!(self.current_variant, Variant::Interpret) {
            // Get the function symbol.
            let Expression::Identifier(ident) = &input.function else {
                panic!("Parsing guarantees that a function name is always an identifier.");
            };

            let callee_program = input.program.unwrap_or(self.program_name);

            let Some(func_symbol) = self.state.symbol_table.lookup_function(Location::new(callee_program, ident.name))
            else {
                panic!("Type checking should have prevented this.");
            };

            if matches!(func_symbol.function.variant, Variant::Interpret) {
                self.state.handler.emit_err(TypeCheckerError::non_interpret_calls_interpret(ident, input.span))
            }
        }
        (input.into(), Default::default())
    }
}
