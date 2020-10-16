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
    /// Resolve the type of dividing `lhs / rhs`
    pub(crate) fn div(
        table: &mut SymbolTable,
        expected_type: Option<Type>,
        lhs: UnresolvedExpression,
        rhs: UnresolvedExpression,
        span: Span,
    ) -> Result<Self, ExpressionError> {
        // Resolve lhs and rhs expressions
        let (lhs_resolved, rhs_resolved) = Self::binary(table, expected_type, lhs, rhs, span.clone())?;

        Ok(Expression {
            type_: lhs_resolved.type_.clone(),
            value: ExpressionValue::Div(Box::new(lhs_resolved), Box::new(rhs_resolved), span),
        })
    }
}
