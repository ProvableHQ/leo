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
use leo_static_check::{SymbolTable, Type};
use leo_typed::{Assignee, AssigneeAccess, Expression as UnresolvedExpression, Span};

use serde::{Deserialize, Serialize};

/// A statement that assigns `Assignee = Expression;`.
/// Checks that the expression resolves to the assignee's type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Assign {
    pub assignee: Assignee,
    pub expression: Expression,
    pub span: Span,
}

impl Statement {
    ///
    /// Resolves an assign statement
    ///
    pub(crate) fn assign(
        table: &mut SymbolTable,
        assignee: Assignee,
        expression: UnresolvedExpression,
        span: Span,
    ) -> Result<Self, StatementError> {
        // Lookup variable in symbol table
        let key = &assignee.identifier.name;
        let variable = table
            .get_variable(key)
            .ok_or(StatementError::undefined_variable(key.clone(), span.clone()))?;

        // Throw an error if this variable is not mutable
        if !variable.is_mutable() {
            return Err(StatementError::immutable_assign(variable.identifier.name.clone(), span));
        }

        // Get inner assignee type
        let type_ = get_inner_assignee_type(table, variable.type_.clone(), assignee.accesses.clone(), span.clone())?;

        // Resolve the expression based on the assignee type
        let expression_resolved = Expression::resolve(table, (Some(type_), expression))?;

        Ok(Statement::Assign(Assign {
            assignee,
            expression: expression_resolved,
            span,
        }))
    }
}

///
/// Accesses the inner type of an assignee such as an array, tuple, or circuit member.
/// Returns an error for invalid accesses.
///
fn get_inner_assignee_type(
    table: &SymbolTable,
    type_: Type,
    accesses: Vec<AssigneeAccess>,
    span: Span,
) -> Result<Type, StatementError> {
    match accesses.first() {
        None => Ok(type_),
        Some(access) => {
            // Check that we are correctly accessing the type
            let next_type = match (&type_, access) {
                (Type::Array(next_type, _), AssigneeAccess::Array(_)) => *next_type.clone(),
                (Type::Tuple(types), AssigneeAccess::Tuple(index)) => types[*index].clone(),
                (Type::Circuit(identifier), AssigneeAccess::Member(member)) => {
                    let circuit_type_option = table.get_circuit(&identifier.name);

                    let circuit_type = match circuit_type_option {
                        Some(circuit_type) => circuit_type,
                        None => return Err(StatementError::undefined_circuit(identifier.clone())),
                    };
                    circuit_type.member_type(member)?.clone()
                }
                (type_, _) => return Err(StatementError::invalid_assign(type_, span)),
            };

            return get_inner_assignee_type(table, next_type, accesses[1..].to_vec(), span);
        }
    }
}
