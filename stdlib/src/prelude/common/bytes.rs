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

use super::unwrap_u8_array_argument;
use crate::{ConstrainedValue, GroupType, Integer};

use leo_errors::{Result, Span};

use snarkvm_fields::PrimeField;

pub fn to_bytes<'a, F: PrimeField, G: GroupType<F>>(
    value: ConstrainedValue<'a, F, G>,
    span: &Span,
) -> Result<ConstrainedValue<'a, F, G>> {
    let bytes = value.to_bytes(span)?;

    Ok(ConstrainedValue::Array(
        bytes
            .into_iter()
            .map(Integer::U8)
            .map(ConstrainedValue::Integer)
            .collect(),
    ))
}

pub fn from_bytes<'a, F: PrimeField, G: GroupType<F>>(
    arg: ConstrainedValue<'a, F, G>,
    output: leo_asg::Type<'a>,
    span: &Span,
) -> Result<ConstrainedValue<'a, F, G>> {
    let bytes = unwrap_u8_array_argument(arg);

    ConstrainedValue::from_bytes(output, &bytes, span)
}
