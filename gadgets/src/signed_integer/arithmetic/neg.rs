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

use crate::arithmetic::Neg;
use crate::errors::SignedIntegerError;
use crate::signed_integer::*;

use snarkvm_fields::PrimeField;
use snarkvm_r1cs::ConstraintSystem;

macro_rules! neg_int_impl {
    ($($gadget: ident)*) => ($(
        impl<F: PrimeField> Neg<F> for $gadget {
            type ErrorType = SignedIntegerError;

            fn neg<CS: ConstraintSystem<F>>(
                &self,
                cs: CS
            ) -> Result<Self, Self::ErrorType> {
                let value = match self.value {
                    Some(val) => {
                        match val.checked_neg() {
                            Some(val_neg) => Some(val_neg),
                            None => return Err(SignedIntegerError::Overflow) // -0 should fail
                        }
                    }
                    None => None,
                };

                // calculate two's complement
                let bits = self.bits.neg(cs)?;

                Ok(Self {
                    bits,
                    value,
                })
            }
        }
    )*)
}

neg_int_impl!(Int8 Int16 Int32 Int64 Int128);
