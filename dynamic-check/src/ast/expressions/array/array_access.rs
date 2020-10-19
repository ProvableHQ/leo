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
//
// use crate::{
//     ast::expressions::array::RangeOrExpression, Expression, ExpressionError, ExpressionValue, Frame, ResolvedNode,
// };
// use leo_static_check::Type;
// use leo_typed::{Expression as UnresolvedExpression, RangeOrExpression as UnresolvedRangeOrExpression, Span};
//
// impl Expression {
//     ///
//     /// Returns a new array `Expression` from a given array access.
//     ///
//     /// Performs a lookup in the given function body's variable table if the expression contains
//     /// user-defined variables.
//     ///
//     pub(crate) fn array_access(
//         frame: &Frame,
//         type_: &Type,
//         array: Box<UnresolvedExpression>,
//         range: Box<UnresolvedRangeOrExpression>,
//         span: Span,
//     ) -> Result<Self, ExpressionError> {
//         // Lookup the array in the variable table.
//         let array_resolved = Expression::new(frame, type_, *array)?;
//
//         // Resolve the range or expression
//         let range_resolved = RangeOrExpression::new(frame, type_, *range)?;
//
//         Ok(Expression {
//             type_: range_resolved.type_().clone(),
//             value: ExpressionValue::ArrayAccess(Box::new(array_resolved), Box::new(range_resolved), span),
//         })
//     }
// }
