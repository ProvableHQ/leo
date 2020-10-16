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
use leo_static_check::{SymbolTable, Type};
use leo_typed::{Expression as UnresolvedExpression, Span};

impl Expression {
    /// Resolve the type of `lhs >= rhs`
    pub(crate) fn ge(
        table: &mut SymbolTable,
        expected_type: Option<Type>,
        lhs: UnresolvedExpression,
        rhs: UnresolvedExpression,
        span: Span,
    ) -> Result<Self, ExpressionError> {
        // This expression results in a boolean type
        let type_ = Type::Boolean;

        // Check the expected type if given
        Type::check_type(&expected_type, &type_, span.clone())?;

        // Resolve lhs and rhs expressions
        let (lhs_resolved, rhs_resolved) = Self::binary(table, None, lhs, rhs, span.clone())?;

        // Check that expressions are integer type
        lhs_resolved.check_type_integer()?;
        rhs_resolved.check_type_integer()?;

        Ok(Expression {
            // This expression results in a boolean type
            type_,
            value: ExpressionValue::Ge(Box::new(lhs_resolved), Box::new(rhs_resolved), span),
        })
    }
}
