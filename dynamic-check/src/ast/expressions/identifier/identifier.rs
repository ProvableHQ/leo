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
use leo_typed::Identifier;

impl Expression {
    /// Resolve the type of an identifier expression
    pub(crate) fn identifier(
        table: &SymbolTable,
        expected_type: Option<Type>,
        identifier: Identifier,
    ) -> Result<Self, ExpressionError> {
        // Lookup identifier in symbol table
        let variable = table
            .get_variable(&identifier.name)
            .ok_or(ExpressionError::undefined_identifier(identifier.clone()))?;

        // Get type of symbol table entry
        let variable_type = variable.type_.clone();
        let span = identifier.span.clone();

        // Check the expected type if given
        Type::check_type(&expected_type, &variable_type, span)?;

        Ok(Expression {
            type_: variable_type,
            value: ExpressionValue::Identifier(identifier),
        })
    }
}
