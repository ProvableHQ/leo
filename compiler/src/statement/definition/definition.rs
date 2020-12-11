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

//! Enforces a definition statement in a compiled Leo program.

use crate::{errors::StatementError, program::ConstrainedProgram, ConstrainedValue, GroupType};
use leo_ast::{Declare, DefinitionStatement, Span, VariableName};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    fn enforce_single_definition<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        function_scope: &str,
        is_constant: bool,
        variable_name: VariableName,
        mut value: ConstrainedValue<F, G>,
        span: &Span,
    ) -> Result<(), StatementError> {
        if is_constant && variable_name.mutable {
            return Err(StatementError::immutable_assign(
                variable_name.to_string(),
                span.to_owned(),
            ));
        } else {
            value.allocate_value(cs, span)?
        }

        self.store_definition(function_scope, variable_name.mutable, variable_name.identifier, value);

        Ok(())
    }

    fn enforce_multiple_definition<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        function_scope: &str,
        is_constant: bool,
        variable_names: Vec<VariableName>,
        values: Vec<ConstrainedValue<F, G>>,
        span: &Span,
    ) -> Result<(), StatementError> {
        if values.len() != variable_names.len() {
            return Err(StatementError::invalid_number_of_definitions(
                values.len(),
                variable_names.len(),
                span.to_owned(),
            ));
        }

        for (variable, value) in variable_names.into_iter().zip(values.into_iter()) {
            self.enforce_single_definition(cs, function_scope, is_constant, variable, value, span)?;
        }

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn enforce_definition_statement<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: &str,
        function_scope: &str,
        statement: DefinitionStatement,
    ) -> Result<(), StatementError> {
        let num_variables = statement.variable_names.len();
        let is_constant = match statement.declaration_type {
            Declare::Let => false,
            Declare::Const => true,
        };
        let expression =
            self.enforce_expression(cs, file_scope, function_scope, statement.type_.clone(), statement.value)?;

        if num_variables == 1 {
            // Define a single variable with a single value
            let variable = statement.variable_names[0].clone();

            self.enforce_single_definition(cs, function_scope, is_constant, variable, expression, &statement.span)
        } else {
            // Define multiple variables for an expression that returns multiple results (multiple definition)
            let values = match expression {
                // ConstrainedValue::Return(values) => values,
                ConstrainedValue::Tuple(values) => values,
                value => return Err(StatementError::multiple_definition(value.to_string(), statement.span)),
            };

            self.enforce_multiple_definition(
                cs,
                function_scope,
                is_constant,
                statement.variable_names,
                values,
                &statement.span,
            )
        }
    }
}
