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

use std::convert::TryFrom;
use std::str::FromStr;

use num_bigint::BigUint;
use serde::Deserialize;
use serde::Serialize;
use snarkvm_fields::Field;
use snarkvm_fields::FieldError;
use snarkvm_models::curves::Fp256;
use snarkvm_models::curves::Fp256Parameters;

#[derive(Serialize, Deserialize)]
pub struct SerializedField(pub String);

impl<F: Field> From<&F> for SerializedField {
    fn from(field: &F) -> Self {
        // write field to buffer

        let mut buf = Vec::new();

        field.write(&mut buf).unwrap();

        // convert to base 10 integer

        let f_bigint = BigUint::from_bytes_le(&buf);

        let f_string = f_bigint.to_str_radix(10);

        Self(f_string)
    }
}

impl<P: Fp256Parameters> TryFrom<&SerializedField> for Fp256<P> {
    type Error = FieldError;

    fn try_from(serialized: &SerializedField) -> Result<Self, Self::Error> {
        Fp256::<P>::from_str(&serialized.0)
    }
}
