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
use leo_ast::{ReturnStatement, Span, Type};

use snarkvm_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

/// Returns `Ok` if the expected type == actual type, returns `Err` otherwise.
pub fn check_return_type(expected: &Type, actual: &Type, span: &Span) -> Result<(), StatementError> {
    if expected.ne(&actual) {
        // If the return type is `SelfType` returning the circuit type is okay.
        return if (expected.is_self() && actual.is_circuit()) || expected.eq_flat(&actual) {
            Ok(())
        } else {
            Err(StatementError::arguments_type(&expected, &actual, span.to_owned()))
        };
    }
    Ok(())
}

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn enforce_return_statement<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: &str,
        function_scope: &str,
        return_type: Option<Type>,
        statement: ReturnStatement,
    ) -> Result<ConstrainedValue<F, G>, StatementError> {
        let result = self.enforce_operand(
            cs,
            file_scope,
            function_scope,
            return_type.clone(),
            statement.expression,
            &statement.span,
        )?;

        // Make sure we return the correct type.
        if let Some(expected) = return_type {
            check_return_type(&expected, &result.to_type(&statement.span)?, &statement.span)?;
        }

        Ok(result)
    }
}
