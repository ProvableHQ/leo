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

use crate::signed_integer::*;

use snarkvm_fields::PrimeField;
use snarkvm_gadgets::traits::boolean::Boolean;
use snarkvm_gadgets::traits::eq::ConditionalEqGadget;
use snarkvm_r1cs::ConstraintSystem;
use snarkvm_r1cs::SynthesisError;

macro_rules! cond_eq_int_impl {
    ($($gadget: ident),*) => ($(
        impl<F: PrimeField> ConditionalEqGadget<F> for $gadget {
            fn conditional_enforce_equal<CS: ConstraintSystem<F>>(
                &self,
                mut cs: CS,
                other: &Self,
                condition: &Boolean,
            ) -> Result<(), SynthesisError> {
                for (i, (a, b)) in self.bits.iter().zip(&other.bits).enumerate() {
                    a.conditional_enforce_equal(
                        &mut cs.ns(|| format!("{} equality check for the {}-th bit", <$gadget as Int>::SIZE, i)),
                        b,
                        condition,
                    )?;
                }

                Ok(())
            }

            fn cost() -> usize {
                <$gadget as Int>::SIZE * <Boolean as ConditionalEqGadget<F>>::cost()
            }
        }
    )*)
}

cond_eq_int_impl!(Int8, Int16, Int32, Int64, Int128);
