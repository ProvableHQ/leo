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

use crate::{Expression, ResolvedNode, Statement, StatementError};
use leo_static_check::{FunctionOutputType, SymbolTable, Type};
use leo_typed::{
    ConditionalNestedOrEndStatement as UnresolvedNestedOrEnd,
    ConditionalStatement as UnresolvedConditional,
    Span,
    Statement as UnresolvedStatement,
};

use serde::{Deserialize, Serialize};

/// A nested `else if` or an ending `else` clause
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionalNestedOrEndStatement {
    Nested(Box<Conditional>),
    End(Vec<Statement>),
}

/// An `if else` statement with resolved inner statements
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Conditional {
    pub condition: Expression,
    pub statements: Vec<Statement>,
    pub next: Option<ConditionalNestedOrEndStatement>,
    pub span: Span,
}

impl Conditional {
    ///
    /// Resolves a conditional statement.
    ///
    pub(crate) fn from_unresolved(
        table: &mut SymbolTable,
        return_type: FunctionOutputType,
        conditional: UnresolvedConditional,
        span: Span,
    ) -> Result<Self, StatementError> {
        // Create child symbol table.
        let mut child_table = SymbolTable::new(Some(Box::new(table.clone())));

        // Resolve the condition to a boolean.
        let type_boolean = Some(Type::Boolean);
        let condition_resolved = Expression::resolve(&mut child_table, (type_boolean, conditional.condition))?;

        // Resolve all statements.
        let statements_resolved = resolve_statements(&mut child_table, return_type.clone(), conditional.statements)?;

        // Check for an `else if` or `else` clause.
        let nested_or_end = match conditional.next {
            Some(nested_or_end) => nested_or_end,
            None => {
                return Ok(Conditional {
                    condition: condition_resolved,
                    statements: statements_resolved,
                    next: None,
                    span,
                });
            }
        };

        // Evaluate the `else if` or `else` clause.
        let next_resolved = match nested_or_end {
            UnresolvedNestedOrEnd::Nested(conditional) => {
                // Type check the `else if` clause.
                let conditional_resolved =
                    Self::from_unresolved(table, return_type.clone(), *conditional, span.clone())?;

                ConditionalNestedOrEndStatement::Nested(Box::new(conditional_resolved))
            }
            UnresolvedNestedOrEnd::End(statements) => {
                // Create child symbol table.
                let mut child_table = SymbolTable::new(Some(Box::new(table.clone())));

                // Type check the `else` clause.
                let statements_resolved = resolve_statements(&mut child_table, return_type, statements)?;

                ConditionalNestedOrEndStatement::End(statements_resolved)
            }
        };

        Ok(Conditional {
            condition: condition_resolved,
            statements: statements_resolved,
            next: Some(next_resolved),
            span,
        })
    }
}

/// Resolve an array of statements.
fn resolve_statements(
    table: &mut SymbolTable,
    return_type: FunctionOutputType,
    statements: Vec<UnresolvedStatement>,
) -> Result<Vec<Statement>, StatementError> {
    Ok(statements
        .into_iter()
        .map(|statement| Statement::resolve(table, (return_type.clone(), statement)))
        .collect::<Result<Vec<_>, _>>()?)
}

impl Statement {
    /// Resolves a conditional statement.
    pub(crate) fn conditional(
        function_body: &function_body,
        return_type: FunctionOutputType,
        conditional: UnresolvedConditional,
        span: Span,
    ) -> Result<Self, StatementError> {
        let conditional = Conditional::from_unresolved(function_body, return_type, conditional, span)?;

        Ok(Statement::Conditional(conditional))
    }
}
