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

use super::unwrap_boolean_array_argument;
use crate::{ConstrainedValue, GroupType};

use leo_errors::{Result, Span};

use snarkvm_fields::PrimeField;

pub fn to_bits<'a, F: PrimeField, G: GroupType<F>>(
    value: ConstrainedValue<'a, F, G>,
    span: &Span,
) -> Result<ConstrainedValue<'a, F, G>> {
    let bits = value.to_bits_le(span)?;

    Ok(ConstrainedValue::Array(
        bits.into_iter().map(ConstrainedValue::Boolean).collect(),
    ))
}

pub fn from_bits<'a, F: PrimeField, G: GroupType<F>>(
    arg: ConstrainedValue<'a, F, G>,
    output: leo_asg::Type<'a>,
    span: &Span,
) -> Result<ConstrainedValue<'a, F, G>> {
    let bits = unwrap_boolean_array_argument(arg);

    ConstrainedValue::from_bits_le(output, &bits, span)
}
