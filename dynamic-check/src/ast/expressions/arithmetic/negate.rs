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

use crate::{Expression, ExpressionError, ExpressionValue, Frame};
use leo_static_check::Type;
use leo_typed::{Expression as UnresolvedExpression, Span};

impl Expression {
    /// Resolve the type of negating `-expression`
    pub(crate) fn negate(
        frame: &Frame,
        type_: &Type,
        unresolved_expression: UnresolvedExpression,
        span: Span,
    ) -> Result<Self, ExpressionError> {
        let expression_resolved = Self::new(frame, type_, unresolved_expression)?;

        Ok(Expression {
            type_: type_.clone(),
            value: ExpressionValue::Negate(Box::new(expression_resolved), span),
        })
    }
}
