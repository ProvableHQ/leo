// Copyright (C) 2019-2020 Aleo Systems Inc.
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

//! Evaluates a macro in a compiled Leo program.

use crate::{errors::ConsoleError, program::ConstrainedProgram, statement::get_indicator_value, GroupType};
use leo_ast::{ConsoleFunction, ConsoleStatement};

use snarkvm_models::{
    curves::{Field, PrimeField},
    gadgets::{r1cs::ConstraintSystem, utilities::boolean::Boolean},
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn evaluate_console_function_call<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: &str,
        function_scope: &str,
        indicator: &Boolean,
        console: ConsoleStatement,
    ) -> Result<(), ConsoleError> {
        match console.function {
            ConsoleFunction::Assert(expression) => {
                self.evaluate_console_assert(cs, file_scope, function_scope, indicator, expression, &console.span)?;
            }
            ConsoleFunction::Debug(string) => {
                let string = self.format(cs, file_scope, function_scope, string)?;

                if get_indicator_value(indicator) {
                    tracing::debug!("{}", string);
                }
            }
            ConsoleFunction::Error(string) => {
                let string = self.format(cs, file_scope, function_scope, string)?;

                if get_indicator_value(indicator) {
                    tracing::error!("{}", string);
                }
            }
            ConsoleFunction::Log(string) => {
                let string = self.format(cs, file_scope, function_scope, string)?;

                if get_indicator_value(indicator) {
                    tracing::info!("{}", string);
                }
            }
        }

        Ok(())
    }
}
