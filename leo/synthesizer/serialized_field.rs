use snarkos_errors::curves::FieldError;
use snarkos_models::curves::{Field, Fp256, Fp256Parameters};

use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, str::FromStr};

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
