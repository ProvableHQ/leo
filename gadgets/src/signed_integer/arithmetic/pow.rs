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

use crate::{errors::SignedIntegerError, Int, Int128, Int16, Int32, Int64, Int8};

use snarkvm_fields::PrimeField;
use snarkvm_gadgets::traits::utilities::{
    alloc::AllocGadget,
    arithmetic::{Mul, Pow},
    boolean::Boolean,
    select::CondSelectGadget,
};
use snarkvm_r1cs::ConstraintSystem;

macro_rules! pow_int_impl {
    ($($gadget:ty)*) => ($(
        impl<F: PrimeField> Pow<F> for $gadget {
            type ErrorType = SignedIntegerError;

            fn pow<CS: ConstraintSystem<F>>(&self, mut cs: CS, other: &Self) -> Result<Self, Self::ErrorType> {
                // let mut res = Self::one();
                //
                // let mut found_one = false;
                //
                // for i in BitIterator::new(exp) {
                //
                //     res.square_in_place();
                //
                //     if i {
                //         res *= self;
                //     }
                // }
                // res

                let is_constant = Boolean::constant(Self::result_is_constant(&self, &other));
                let one_const = Self::constant(1 as <$gadget as Int>::IntegerType);
                let one_alloc = Self::alloc(&mut cs.ns(|| "allocated_1"), || Ok(1 as <$gadget as Int>::IntegerType))?;
                let mut result = Self::conditionally_select(
                    &mut cs.ns(|| "constant_or_allocated"),
                    &is_constant,
                    &one_const,
                    &one_alloc,
                )?;

                for (i, bit) in other.bits.iter().rev().enumerate() {
                    result = result.mul(cs.ns(|| format!("square_{}", i)), &result).unwrap();

                    let mul_by_self = result
                        .mul(cs.ns(|| format!("multiply_by_self_{}", i)), &self);

                    result = Self::conditionally_select(
                        &mut cs.ns(|| format!("mul_by_self_or_result_{}", i)),
                        bit,
                        &mul_by_self?,
                        &result,
                    )?;

                }
                Ok(result)
            }
        }
    )*)
}

pow_int_impl!(Int8 Int16 Int32 Int64 Int128);
