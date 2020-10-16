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
    /// Resolves a tuple access expression
    pub(crate) fn tuple_access(
        table: &mut SymbolTable,
        expected_type: Option<Type>,
        tuple: Box<UnresolvedExpression>,
        index: usize,
        span: Span,
    ) -> Result<Self, ExpressionError> {
        // Lookup the tuple in the symbol table.
        // We do not know the length from this context so `expected_type = None`.
        let tuple_resolved = Expression::resolve(table, (None, *tuple))?;

        // Resolve the tuple index type
        let type_tuple = tuple_resolved.type_().get_type_tuple(span.clone())?;

        // Throw a tuple out of bounds error for an index that does not exist
        if index > type_tuple.len() {
            return Err(ExpressionError::invalid_index_tuple(
                index,
                type_tuple.len(),
                span.clone(),
            ));
        }

        let type_ = type_tuple[index].clone();

        // Check that expected type matches
        Type::check_type(&expected_type, &type_, span.clone())?;

        Ok(Expression {
            type_,
            value: ExpressionValue::TupleAccess(Box::new(tuple_resolved), index, span),
        })
    }
}
