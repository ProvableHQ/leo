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

//! Enforces a unary negate `-` operator in a resolved Leo program.

use crate::{value::ConstrainedValue, GroupType};
use leo_errors::{new_backtrace, CompilerError, Result, Span};

use snarkvm_fields::PrimeField;
use snarkvm_r1cs::ConstraintSystem;

pub fn enforce_negate<'a, F: PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    value: ConstrainedValue<'a, F, G>,
    span: &Span,
) -> Result<ConstrainedValue<'a, F, G>> {
    match value {
        ConstrainedValue::Integer(integer) => Ok(ConstrainedValue::Integer(integer.negate(cs, span)?)),
        ConstrainedValue::Field(field) => Ok(ConstrainedValue::Field(field.negate(cs, span)?)),
        ConstrainedValue::Group(group) => Ok(ConstrainedValue::Group(group.negate(cs, span)?)),
        value => return Err(CompilerError::incompatible_types(format!("-{}", value), span, new_backtrace()).into()),
    }
}
