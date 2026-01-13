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

use super::ProcessingScriptVisitor;

use leo_ast::{AstReconstructor, CallExpression, Expression, Variant};
use leo_errors::TypeCheckerError;

impl AstReconstructor for ProcessingScriptVisitor<'_> {
    type AdditionalInput = ();
    type AdditionalOutput = ();

    /* Expressions */
    fn reconstruct_call(&mut self, input: CallExpression, _additional: &()) -> (Expression, Self::AdditionalOutput) {
        if !matches!(self.current_variant, Variant::Script) {
            let Some(func_symbol) = self.state.symbol_table.lookup_function(input.function.expect_global_location())
            else {
                panic!("Type checking should have prevented this.");
            };

            if matches!(func_symbol.function.variant, Variant::Script) {
                self.state
                    .handler
                    .emit_err(TypeCheckerError::non_script_calls_script(input.function.clone(), input.span))
            }
        }
        (input.into(), Default::default())
    }
}
