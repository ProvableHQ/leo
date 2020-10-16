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

use crate::{Expression, ExpressionError, ExpressionValue};
use leo_symbol_table::{ResolvedNode, SymbolTable, Type};
use leo_typed::{Expression as UnresolvedExpression, Span};

impl Expression {
    /// Resolves an `if {cond} ? {first} : {second}` expression
    /// `{cond}` should resolve to a boolean type
    /// `{first}` and `{second}` should have equal types
    pub(crate) fn conditional(
        table: &mut SymbolTable,
        expected_type: Option<Type>,
        cond: UnresolvedExpression,
        first: UnresolvedExpression,
        second: UnresolvedExpression,
        span: Span,
    ) -> Result<Self, ExpressionError> {
        // Resolve the condition to a boolean type
        let cond_type = Some(Type::Boolean);
        let cond_resolved = Expression::resolve(table, (cond_type, cond))?;

        // Resolve the first and second expressions to the expected type
        let (first_resolved, second_resolved) = Expression::binary(table, expected_type, first, second, span.clone())?;

        Ok(Expression {
            type_: first_resolved.type_.clone(),
            value: ExpressionValue::IfElse(
                Box::new(cond_resolved),
                Box::new(first_resolved),
                Box::new(second_resolved),
                span,
            ),
        })
    }
}
