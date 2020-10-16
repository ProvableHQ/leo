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

use crate::{Expression, ExpressionError, ExpressionValue, VariableTable};
use leo_static_check::{SymbolTable, Type};
use leo_typed::Identifier;

impl Expression {
    ///
    /// Returns a new variable expression from a given `UnresolvedExpression`.
    ///
    /// Performs a lookup in the given variable table to find the variable's type.
    ///
    pub(crate) fn variable(variable_table: &VariableTable, identifier: Identifier) -> Result<Self, ExpressionError> {
        // Lookup the type of the given variable.
        let type_ = variable_table
            .get(&identifier.name)
            .ok_or(ExpressionError::undefined_identifier(identifier.clone()))?;

        Ok(Expression {
            type_: type_.clone(),
            value: ExpressionValue::Variable(identifier),
        })
    }
}
