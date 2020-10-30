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

//! Enforces a return statement in a compiled Leo program.

use crate::{errors::StatementError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_core_ast::{Expression, Span, Type};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

fn check_return_type(expected: Option<Type>, actual: Type, span: &Span) -> Result<(), StatementError> {
    match expected {
        Some(expected) => {
            if expected.ne(&actual) {
                if (expected.is_self() && actual.is_circuit()) || expected.match_array_types(&actual) {
                    return Ok(());
                } else {
                    return Err(StatementError::arguments_type(&expected, &actual, span.to_owned()));
                }
            }
            Ok(())
        }
        None => Ok(()),
    }
}

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn enforce_return_statement<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: &str,
        function_scope: &str,
        expression: Expression,
        return_type: Option<Type>,
        span: &Span,
    ) -> Result<ConstrainedValue<F, G>, StatementError> {
        // Make sure we return the correct number of values

        let result = self.enforce_operand(cs, file_scope, function_scope, return_type.clone(), expression, span)?;

        check_return_type(return_type, result.to_type(&span)?, span)?;

        Ok(result)
    }
}
