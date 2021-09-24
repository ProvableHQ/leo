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

use crate::{ConstrainedValue, GroupType, Integer};

use leo_errors::{CompilerError, Result, Span};

use snarkvm_fields::PrimeField;
use snarkvm_gadgets::integers::uint::UInt8;

pub fn to_bytes<'a, F: PrimeField, G: GroupType<F>>(
    value: ConstrainedValue<'a, F, G>,
    output: leo_asg::Type<'a>,
    span: &Span,
) -> Result<ConstrainedValue<'a, F, G>> {
    let bytes = value.to_bytes(span)?;
    // TODO BETTER ERROR
    let expected_len: usize = match output {
        leo_asg::Type::Array(_, size) => size,
        _ => return Err(CompilerError::unknown_built_in_method("", "", span).into()),
    };
    assert_eq!(expected_len, bytes.len());

    Ok(ConstrainedValue::Array(
        bytes
            .into_iter()
            .map(Integer::U8)
            .map(ConstrainedValue::Integer)
            .collect(),
    ))
}

fn unwrap_argument<F: PrimeField, G: GroupType<F>>(arg: ConstrainedValue<F, G>, expected_len: usize) -> Vec<UInt8> {
    if let ConstrainedValue::Array(args) = arg {
        assert_eq!(args.len(), expected_len);
        args.into_iter()
            .map(|item| {
                if let ConstrainedValue::Integer(Integer::U8(u8int)) = item {
                    u8int
                } else {
                    panic!("illegal non-u8 type in from_bits call");
                }
            })
            .collect()
    } else {
        panic!("illegal non-array type in blake2s call");
    }
}

pub fn from_bytes<'a, F: PrimeField, G: GroupType<F>>(
    arg: ConstrainedValue<'a, F, G>,
    output: leo_asg::Type<'a>,
    span: &Span,
) -> Result<ConstrainedValue<'a, F, G>> {
    // TODO BETTER ERROR
    let expected_len: usize = match &arg {
        ConstrainedValue::Array(items) => items.len(),
        _ => return Err(CompilerError::unknown_built_in_method("", "", span).into()),
    };
    let bytes = unwrap_argument(arg, expected_len);

    ConstrainedValue::from_bytes(output, &bytes, span)
}
