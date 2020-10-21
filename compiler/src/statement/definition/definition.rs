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
use leo_typed::{Declare, Expression, Span, Type, VariableName, Variables};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    fn enforce_single_definition<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        function_scope: String,
        is_constant: bool,
        variable_name: VariableName,
        mut value: ConstrainedValue<F, G>,
        span: Span,
    ) -> Result<(), StatementError> {
        if is_constant && variable_name.mutable {
            return Err(StatementError::immutable_assign(variable_name.to_string(), span));
        } else {
            value.allocate_value(cs, span)?
        }

        self.store_definition(function_scope, variable_name.mutable, variable_name.identifier, value);

        Ok(())
    }

    fn enforce_expressions<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        type_: Option<Type>,
        expressions: Vec<Expression>,
        span: Span,
    ) -> Result<Vec<ConstrainedValue<F, G>>, StatementError> {
        let types = match type_ {
            Some(Type::Tuple(types)) => types,
            Some(type_) => return Err(StatementError::tuple_type(type_.to_string(), span)),
            None => vec![],
        };

        let implicit_types = types.is_empty();
        let mut expected_types = Vec::with_capacity(expressions.len());

        for ty in types.iter().take(expressions.len()) {
            let expected_type = if implicit_types { None } else { Some(ty.clone()) };

            expected_types.push(expected_type);
        }

        let mut values = Vec::with_capacity(expressions.len());

        for (expression, expected_type) in expressions.into_iter().zip(expected_types.into_iter()) {
            let value = self.enforce_expression(
                cs,
                file_scope.clone(),
                function_scope.clone(),
                expected_type,
                expression,
            )?;

            values.push(value);
        }

        Ok(values)
    }

    #[allow(clippy::too_many_arguments)]
    fn enforce_tuple_definition<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        is_constant: bool,
        variables: Variables,
        expressions: Vec<Expression>,
        span: Span,
    ) -> Result<(), StatementError> {
        let values = self.enforce_expressions(
            cs,
            file_scope,
            function_scope.clone(),
            variables.type_.clone(),
            expressions,
            span.clone(),
        )?;

        let tuple = ConstrainedValue::Tuple(values);
        let variable = variables.names[0].clone();

        self.enforce_single_definition(cs, function_scope, is_constant, variable, tuple, span)
    }

    fn enforce_multiple_definition<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        function_scope: String,
        is_constant: bool,
        variables: Variables,
        values: Vec<ConstrainedValue<F, G>>,
        span: Span,
    ) -> Result<(), StatementError> {
        if values.len() != variables.names.len() {
            return Err(StatementError::invalid_number_of_definitions(
                values.len(),
                variables.names.len(),
                span,
            ));
        }

        for (variable, value) in variables.names.into_iter().zip(values.into_iter()) {
            self.enforce_single_definition(cs, function_scope.clone(), is_constant, variable, value, span.clone())?;
        }

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn enforce_definition_statement<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        declare: Declare,
        variables: Variables,
        expressions: Vec<Expression>,
        span: Span,
    ) -> Result<(), StatementError> {
        let num_variables = variables.names.len();
        let num_values = expressions.len();
        let is_constant = match declare {
            Declare::Let => false,
            Declare::Const => true,
        };

        if num_variables == 1 && num_values == 1 {
            // Define a single variable with a single value
            let variable = variables.names[0].clone();
            let expression = self.enforce_expression(
                cs,
                file_scope,
                function_scope.clone(),
                variables.type_,
                expressions[0].clone(),
            )?;

            self.enforce_single_definition(cs, function_scope, is_constant, variable, expression, span)
        } else if num_variables == 1 && num_values > 1 {
            // Define a tuple (single variable with multiple values)

            self.enforce_tuple_definition(
                cs,
                file_scope,
                function_scope,
                is_constant,
                variables,
                expressions,
                span,
            )
        } else if num_variables > 1 && num_values == 1 {
            // Define multiple variables for an expression that returns multiple results (multiple definition)

            let values = match self.enforce_expression(
                cs,
                file_scope,
                function_scope.clone(),
                variables.type_.clone(),
                expressions[0].clone(),
            )? {
                // ConstrainedValue::Return(values) => values,
                ConstrainedValue::Tuple(values) => values,
                value => return Err(StatementError::multiple_definition(value.to_string(), span)),
            };

            self.enforce_multiple_definition(cs, function_scope, is_constant, variables, values, span)
        } else {
            // Define multiple variables for multiple expressions
            let values = self.enforce_expressions(
                cs,
                file_scope,
                function_scope.clone(),
                variables.type_.clone(),
                expressions,
                span.clone(),
            )?;

            self.enforce_multiple_definition(cs, function_scope, is_constant, variables, values, span)
        }
    }
}
