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
use leo_symbol_table::Type;
use leo_typed::{GroupValue, Span};

impl Expression {
    /// Resolve an implicit expression
    pub(crate) fn implicit(
        expected_type: Option<Type>,
        implicit_string: String,
        span: Span,
    ) -> Result<Self, ExpressionError> {
        // TODO: impl type lookahead - need to save this implicit value if there is no expected type
        let type_ = expected_type.unwrap();

        let value = match &type_ {
            Type::Address => ExpressionValue::Address(implicit_string, span),
            Type::Boolean => ExpressionValue::Boolean(implicit_string, span),
            Type::Field => ExpressionValue::Field(implicit_string, span),
            Type::Group => ExpressionValue::Group(GroupValue::Single(implicit_string, span)),
            Type::IntegerType(integer_type) => ExpressionValue::Integer(integer_type.clone(), implicit_string, span),
            Type::Array(_type, _dimensions) => unimplemented!("ERROR: Arrays cannot be implicit"),
            Type::Tuple(_types) => unimplemented!("ERROR: Tuples cannot be implicit"),
            Type::Function(_name) => unimplemented!("ERROR: Functions cannot be implicit"),
            Type::Circuit(_name) => unimplemented!("ERROR: Circuits cannot be implicit"),
        };

        Ok(Expression { type_, value })
    }
}
