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

use crate::{Expression, ExpressionValue};
use leo_static_check::Type;
use leo_typed::Span;

impl Expression {
    ///
    /// Returns a new field expression from a given type and field string.
    ///
    pub(crate) fn field(type_: &Type, field_string: String, span: Span) -> Self {
        Expression {
            type_: type_.clone(),
            value: ExpressionValue::Field(field_string, span),
        }
    }
}
