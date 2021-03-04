// Copyright (C) 2019-2021 Aleo Systems Inc.
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

//! Enforces an arithmetic `-` operator in a resolved Leo program.

use crate::errors::ExpressionError;
use crate::value::ConstrainedValue;
use crate::GroupType;
use leo_ast::Span;

use snarkvm_models::curves::PrimeField;
use snarkvm_models::gadgets::r1cs::ConstraintSystem;

pub fn enforce_sub<'a, F: PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    left: ConstrainedValue<'a, F, G>,
    right: ConstrainedValue<'a, F, G>,
    span: &Span,
) -> Result<ConstrainedValue<'a, F, G>, ExpressionError> {
    match (left, right) {
        (ConstrainedValue::Integer(num_1), ConstrainedValue::Integer(num_2)) => {
            Ok(ConstrainedValue::Integer(num_1.sub(cs, num_2, span)?))
        }
        (ConstrainedValue::Field(field_1), ConstrainedValue::Field(field_2)) => {
            Ok(ConstrainedValue::Field(field_1.sub(cs, &field_2, span)?))
        }
        (ConstrainedValue::Group(point_1), ConstrainedValue::Group(point_2)) => {
            Ok(ConstrainedValue::Group(point_1.sub(cs, &point_2, span)?))
        }
        (val_1, val_2) => Err(ExpressionError::incompatible_types(
            format!("{} - {}", val_1, val_2),
            span.to_owned(),
        )),
    }
}
