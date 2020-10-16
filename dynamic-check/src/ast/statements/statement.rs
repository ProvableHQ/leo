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
    Assign,
    Conditional,
    Definition,
    Expression,
    FunctionBody,
    Iteration,
    ResolvedNode,
    StatementError,
    VariableTable,
};
use leo_static_check::{FunctionOutputType, FunctionType, SymbolTable};
use leo_typed::{ConsoleFunctionCall, Span, Statement as UnresolvedStatement};

use serde::{Deserialize, Serialize};

/// Stores a type-checked statement in a Leo program
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Statement {
    Return(Expression, Span),
    Definition(Definition),
    Assign(Assign),
    Conditional(Conditional),
    Iteration(Iteration),
    Console(ConsoleFunctionCall),
    Expression(Expression, Span),
}

impl Statement {
    ///
    /// Returns a new `Statement` from a given `UnresolvedStatement`.
    ///
    /// Performs a lookup in the given function body's variable table if the statement contains
    /// user-defined types.
    ///
    pub fn new(
        function_body: &FunctionBody,
        unresolved_statement: UnresolvedStatement,
    ) -> Result<Self, StatementError> {
        match unresolved_statement {
            UnresolvedStatement::Return(expression, span) => Self::resolve_return(function_body, expression, span),
            UnresolvedStatement::Definition(declare, variables, expressions, span) => {
                Self::definition(function_body, declare, variables, expressions, span)
            }
            UnresolvedStatement::Assign(assignee, expression, span) => {
                Self::assign(variable_table, assignee, expression, span)
            }
            UnresolvedStatement::Conditional(conditional, span) => {
                Self::conditional(variable_table, return_type, conditional, span)
            }
            UnresolvedStatement::Iteration(index, start, stop, statements, span) => {
                Self::iteration(variable_table, return_type, index, start, stop, statements, span)
            }
            UnresolvedStatement::Console(console_function_call) => Ok(Statement::Console(console_function_call)),
            UnresolvedStatement::Expression(expression, span) => Ok(Statement::Expression(
                Expression::resolve(variable_table, expression)?,
                span,
            )),
        }
    }
}

impl ResolvedNode for Statement {
    type Error = StatementError;
    type UnresolvedNode = (FunctionOutputType, UnresolvedStatement);

    /// Type check a statement inside a program AST
    fn resolve(table: &mut SymbolTable, unresolved: Self::UnresolvedNode) -> Result<Self, Self::Error> {
        let return_type = unresolved.0;
        let statement = unresolved.1;

        match statement {
            UnresolvedStatement::Return(expression, span) => {
                Self::resolve_return(table, return_type.type_, expression, span)
            }
            UnresolvedStatement::Definition(declare, variables, expressions, span) => {
                Self::definition(table, declare, variables, expressions, span)
            }
            UnresolvedStatement::Assign(assignee, expression, span) => Self::assign(table, assignee, expression, span),
            UnresolvedStatement::Conditional(conditional, span) => {
                Self::conditional(table, return_type, conditional, span)
            }
            UnresolvedStatement::Iteration(index, start, stop, statements, span) => {
                Self::iteration(table, return_type, index, start, stop, statements, span)
            }
            UnresolvedStatement::Console(console_function_call) => Ok(Statement::Console(console_function_call)),
            UnresolvedStatement::Expression(expression, span) => Ok(Statement::Expression(
                Expression::resolve(table, (None, expression))?,
                span,
            )),
        }
    }
}
