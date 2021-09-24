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

use snarkvm_fields::PrimeField;
use snarkvm_gadgets::bits::Boolean;
use snarkvm_gadgets::integers::uint::UInt8;

// TODO figure out how to make this function generic?

pub fn unwrap_boolean_array_argument<F: PrimeField, G: GroupType<F>>(arg: ConstrainedValue<F, G>) -> Vec<Boolean> {
    if let ConstrainedValue::Array(args) = arg {
        args.into_iter()
            .map(|item| {
                if let ConstrainedValue::Boolean(boolean) = item {
                    boolean
                } else {
                    panic!("illegal non-boolean type in from_bits call");
                }
            })
            .collect()
    } else {
        panic!("illegal non-array type in blake2s call");
    }
}

pub fn unwrap_u8_array_argument<F: PrimeField, G: GroupType<F>>(arg: ConstrainedValue<F, G>) -> Vec<UInt8> {
    if let ConstrainedValue::Array(args) = arg {
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
