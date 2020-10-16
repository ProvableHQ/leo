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

use crate::{Expression, FunctionBody, ResolvedNode, Statement, StatementError, VariableTable};
use leo_static_check::{SymbolTable, Type};
use leo_typed::{Expression as UnresolvedExpression, Span};

impl Statement {
    ///
    /// Returns a new return statement from a given `UnresolvedExpression`.
    ///
    /// Performs a lookup in the given variable table if the statement contains user-defined types.
    ///
    pub(crate) fn resolve_return(
        function_body: &FunctionBody,
        unresolved_expression: UnresolvedExpression,
        span: Span,
    ) -> Result<Self, StatementError> {
        // Create a new `Expression` from the unresolved return expression
        let expression = Expression::new(function_body, unresolved_expression)?;

        Ok(Statement::Return(expression, span))
    }
}
