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

use snarkos_models::gadgets::utilities::boolean::Boolean;

use std::iter;

/// Sign extends an array of bits to the desired length.
/// Expects least significant bit first
pub trait SignExtend
where
    Self: std::marker::Sized,
{
    #[must_use]
    fn sign_extend(bits: &[Boolean], length: usize) -> Vec<Boolean>;
}

impl SignExtend for Boolean {
    fn sign_extend(bits: &[Boolean], length: usize) -> Vec<Boolean> {
        let msb = bits.last().expect("empty bit list");
        let bits_needed = length - bits.len();

        let mut result = Vec::with_capacity(length);
        result.extend_from_slice(bits);
        result.extend(iter::repeat(*msb).take(bits_needed));

        result
    }
}
