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

use crate::arithmetic::Add;
use crate::arithmetic::Neg;
use crate::arithmetic::Sub;
use crate::errors::SignedIntegerError;
use crate::Int128;
use crate::Int16;
use crate::Int32;
use crate::Int64;
use crate::Int8;
use snarkvm_models::curves::PrimeField;
use snarkvm_models::gadgets::r1cs::ConstraintSystem;

macro_rules! sub_int_impl {
    ($($gadget: ident)*) => ($(
        impl<F: PrimeField> Sub<F> for $gadget {
            type ErrorType = SignedIntegerError;

            fn sub<CS: ConstraintSystem<F>>(&self, mut cs: CS, other: &Self) -> Result<Self, Self::ErrorType> {
                // Negate other
                let other_neg = other.neg(cs.ns(|| format!("negate")))?;

                // self + negated other
                self.add(cs.ns(|| format!("add_complement")), &other_neg)
            }
        }
    )*)
}

sub_int_impl!(Int8 Int16 Int32 Int64 Int128);
