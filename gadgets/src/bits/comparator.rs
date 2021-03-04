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

use snarkvm_errors::gadgets::SynthesisError;
use snarkvm_models::curves::Field;
use snarkvm_models::curves::PrimeField;
use snarkvm_models::gadgets::r1cs::ConstraintSystem;
use snarkvm_models::gadgets::utilities::boolean::Boolean;
use snarkvm_models::gadgets::utilities::select::CondSelectGadget;
use snarkvm_models::gadgets::utilities::uint::UInt128;
use snarkvm_models::gadgets::utilities::uint::UInt16;
use snarkvm_models::gadgets::utilities::uint::UInt32;
use snarkvm_models::gadgets::utilities::uint::UInt64;
use snarkvm_models::gadgets::utilities::uint::UInt8;

pub trait EvaluateLtGadget<F: Field> {
    fn less_than<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Boolean, SynthesisError>;
}

// implementing `EvaluateLtGadget` will implement `ComparatorGadget`
pub trait ComparatorGadget<F: Field>
where
    Self: EvaluateLtGadget<F>,
{
    fn greater_than<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Boolean, SynthesisError> {
        other.less_than(cs, self)
    }

    fn less_than_or_equal<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Boolean, SynthesisError> {
        let is_gt = self.greater_than(cs, other)?;
        Ok(is_gt.not())
    }

    fn greater_than_or_equal<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Boolean, SynthesisError> {
        other.less_than_or_equal(cs, self)
    }
}

macro_rules! uint_cmp_impl {
    ($($gadget: ident),*) => ($(
        /*  Bitwise less than comparison of two unsigned integers */
        impl<F: PrimeField> EvaluateLtGadget<F> for $gadget {
            fn less_than<CS: ConstraintSystem<F>>(&self, mut cs: CS, other: &Self) -> Result<Boolean, SynthesisError> {

                let mut result = Boolean::constant(true);
                let mut all_equal = Boolean::constant(true);

                // msb -> lsb
                for (i, (a, b)) in self
                    .bits
                    .iter()
                    .rev()
                    .zip(other.bits.iter().rev())
                    .enumerate()
                {
                    // a == 0 & b == 1
                    let less = Boolean::and(cs.ns(|| format!("not a and b [{}]", i)), &a.not(), b)?;

                    // a == b = !(a ^ b)
                    let not_equal = Boolean::xor(cs.ns(|| format!("a XOR b [{}]", i)), a, b)?;
                    let equal = not_equal.not();

                    // evaluate a <= b
                    let less_or_equal = Boolean::or(cs.ns(|| format!("less or equal [{}]", i)), &less, &equal)?;

                    // select the current result if it is the first bit difference
                    result = Boolean::conditionally_select(cs.ns(|| format!("select bit [{}]", i)), &all_equal, &less_or_equal, &result)?;

                    // keep track of equal bits
                    all_equal = Boolean::and(cs.ns(|| format!("accumulate equal [{}]", i)), &all_equal, &equal)?;
                }

                result = Boolean::and(cs.ns(|| format!("false if all equal")), &result, &all_equal.not())?;

                Ok(result)
            }
        }

        /* Bitwise comparison of two unsigned integers */
        impl<F: PrimeField> ComparatorGadget<F> for $gadget {}
    )*)
}

uint_cmp_impl!(UInt8, UInt16, UInt32, UInt64, UInt128);
