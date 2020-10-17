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

use crate::{
    check_tuple_type,
    Expression,
    ExpressionValue,
    Frame,
    ResolvedNode,
    Statement,
    StatementError,
    VariableTable,
    VariableTableError,
};
use leo_static_check::{Attribute, ParameterType, SymbolTable, Type};
use leo_typed::{Declare, Expression as UnresolvedExpression, Span, VariableName, Variables};

use serde::{Deserialize, Serialize};

/// A `let` or `const` definition statement.
/// Defines one or more variables with resolved types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Definition {
    pub declare: Declare,
    pub variables: DefinitionVariables,
    pub span: Span,
}

/// One or more variables with resolved types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DefinitionVariables {
    Single(VariableName, Expression),
    Tuple(VariableName, Expression),
    MultipleVariable(Vec<VariableName>, Vec<Expression>),
    MultipleVariableTuple(Vec<VariableName>, Expression),
}

impl DefinitionVariables {
    ///
    /// Returns a new statement that defines a single variable with a single expression.
    ///
    /// Performs a lookup in the given variable table if the `UnresolvedExpression` contains user-defined variables.
    ///
    fn single(
        function_body: &Frame,
        variable_name: VariableName,
        unresolved_expression: UnresolvedExpression,
        span: &Span,
    ) -> Result<Self, StatementError> {
        // Get the type of the variable.
        let type_ = function_body.variable_table.get(variable_name.name_string(), span)?;

        // Create a new `Expression` from the given `UnresolvedExpression`.
        let expression = Expression::new(function_body, type_, unresolved_expression)?;

        Ok(DefinitionVariables::Single(variable_name, expression))
    }

    ///
    /// Returns a new statement that defines a tuple (single variable with multiple expressions).
    ///
    /// Performs a lookup in the given variable table if an `UnresolvedExpression` contains user-defined variables.
    ///
    fn tuple(
        function_body: &Frame,
        variable: VariableName,
        unresolved_expressions: Vec<UnresolvedExpression>,
        span: &Span,
    ) -> Result<Self, StatementError> {
        // Get the type of the variable.
        let type_ = function_body.variable_table.get(variable.name_string(), span)?;

        // Create a new tuple of `Expression`s  from the given vector of `UnresolvedExpression's.
        let tuple = Expression::tuple(function_body, type_, unresolved_expressions, span.clone())?;

        Ok(DefinitionVariables::Tuple(variable, tuple))
    }

    /// Resolves multiple variables for multiple expressions
    fn multiple_variable(
        function_body: &Frame,
        variables: Variables,
        unresolved_expressions: Vec<UnresolvedExpression>,
        span: &Span,
    ) -> Result<Self, StatementError> {
        // Check number of variables == expressions
        if variables.names.len() != unresolved_expressions.len() {
            return Err(StatementError::multiple_variable_expressions(
                variables.names.len(),
                unresolved_expressions.len(),
                span,
            ));
        }

        // Get the type of each variable.
        let variable_types = variables
            .names
            .iter()
            .map(|variable_name| function_body.variable_table.get(variable_name.name_string(), span))
            .collect::<Result<Vec<Type>, VariableTableError>>()?;

        // Create a new vector of `Expression`s from the given vector of `UnresolvedExpression`s.
        let mut expressions_resolved = vec![];

        for (unresolved_expression, variable_type) in unresolved_expressions.into_iter().zip(variable_types) {
            let expression_resolved = Expression::new(function_body, &variable_type, unresolved_expression)?;

            expressions_resolved.push(expression_resolved);
        }

        Ok(DefinitionVariables::MultipleVariable(
            variables.names,
            expressions_resolved,
        ))
    }

    /// Resolves multiple variables for an expression that returns a tuple
    fn multiple_variable_tuple(
        function_body: &Frame,
        variables: Variables,
        unresolved_expression: UnresolvedExpression,
        span: &Span,
    ) -> Result<Self, StatementError> {
        // Get the type of each variable.
        let variable_types = variables
            .names
            .iter()
            .map(|variable_name| function_body.variable_table.get(variable_name.name_string(), span))
            .collect::<Result<Vec<Type>, VariableTableError>>()?;

        // Create a new tuple type from the vector of variable types.
        let tuple_type = Type::Tuple(variable_types);

        // Create a new `Expression` from the given `UnresolvedExpression`.
        // This expression should return a tuple.
        let expression = Expression::new(function_body, &tuple_type, unresolved_expression)?;

        Ok(DefinitionVariables::MultipleVariableTuple(variables.names, expression))
    }
}

/// Inserts a variable definition into the given symbol table
fn insert_defined_variable(
    table: &mut SymbolTable,
    variable: &VariableName,
    type_: &Type,
    span: Span,
) -> Result<(), StatementError> {
    let attributes = if variable.mutable {
        vec![Attribute::Mutable]
    } else {
        vec![]
    };

    // Insert variable into symbol table
    let key = variable.identifier.name.clone();
    let value = ParameterType {
        identifier: variable.identifier.clone(),
        type_: type_.clone(),
        attributes,
    };

    // Check that variable name was not defined twice
    let duplicate = table.insert_name(key, value);

    if duplicate.is_some() {
        return Err(StatementError::duplicate_variable(
            variable.identifier.name.clone(),
            span,
        ));
    }

    Ok(())
}

impl Statement {
    ///
    /// Returns a new `let` or `const` definition statement from a given `UnresolvedExpression`.
    ///
    /// Performs a lookup in the given variable table if the statement contains user-defined variables.
    ///
    pub(crate) fn definition(
        function_body: &Frame,
        declare: Declare,
        variables: Variables,
        unresolved_expressions: Vec<UnresolvedExpression>,
        span: Span,
    ) -> Result<Self, StatementError> {
        let num_variables = variables.names.len();
        let num_values = unresolved_expressions.len();

        let variables = if num_variables == 1 && num_values == 1 {
            // Define a single variable with a single value

            DefinitionVariables::single(
                function_body,
                variables.names[0].clone(),
                unresolved_expressions[0].clone(),
                &span,
            )
        } else if num_variables == 1 && num_values > 1 {
            // Define a tuple (single variable with multiple values)

            DefinitionVariables::tuple(function_body, variables.names[0].clone(), unresolved_expressions, &span)
        } else if num_variables > 1 && num_values == 1 {
            // Define multiple variables for an expression that returns a tuple

            DefinitionVariables::multiple_variable_tuple(
                function_body,
                variables,
                unresolved_expressions[0].clone(),
                &span,
            )
        } else {
            // Define multiple variables for multiple expressions

            DefinitionVariables::multiple_variable(function_body, variables, unresolved_expressions, &span)
        }?;

        Ok(Statement::Definition(Definition {
            declare,
            variables,
            span,
        }))
    }
}
