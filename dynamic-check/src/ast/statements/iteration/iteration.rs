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
use leo_static_check::{FunctionOutputType, ParameterType, SymbolTable, Type};
use leo_typed::{Expression as UnresolvedExpression, Identifier, IntegerType, Span, Statement as UnresolvedStatement};

use serde::{Deserialize, Serialize};

/// A resolved iteration statement
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Iteration {
    pub index: Identifier,
    pub start: Expression,
    pub stop: Expression,
    pub statements: Vec<Statement>,
    pub span: Span,
}

impl Statement {
    /// Resolve an iteration statement
    pub(crate) fn iteration(
        table: &mut SymbolTable,
        return_type: FunctionOutputType,
        index: Identifier,
        start: UnresolvedExpression,
        stop: UnresolvedExpression,
        statements: Vec<UnresolvedStatement>,
        span: Span,
    ) -> Result<Self, StatementError> {
        // TODO: Create child symbol table and add variables from parent

        // Resolve index numbers to a u32 type
        let type_number = Type::IntegerType(IntegerType::U32);

        let start_resolved = Expression::resolve(table, (Some(type_number.clone()), start))?;
        let stop_resolved = Expression::resolve(table, (Some(type_number.clone()), stop))?;

        // Add index to symbol table
        let key = index.name.clone();
        let value = ParameterType {
            identifier: index.clone(),
            type_: type_number,
            attributes: vec![],
        };

        table.insert_name(key, value);

        // Resolve statements
        let statements_resolved = statements
            .into_iter()
            .map(|statement| Statement::resolve(table, (return_type.clone(), statement)))
            .collect::<Result<Vec<Statement>, _>>()?;

        Ok(Statement::Iteration(Iteration {
            index,
            start: start_resolved,
            stop: stop_resolved,
            statements: statements_resolved,
            span,
        }))
    }
}
