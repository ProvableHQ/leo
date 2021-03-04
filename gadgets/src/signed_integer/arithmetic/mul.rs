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

use crate::arithmetic::Mul;
use crate::bits::RippleCarryAdder;
use crate::bits::SignExtend;
use crate::errors::SignedIntegerError;
use crate::Int;
use crate::Int128;
use crate::Int16;
use crate::Int32;
use crate::Int64;
use crate::Int8;
use snarkvm_models::curves::FpParameters;
use snarkvm_models::curves::PrimeField;
use snarkvm_models::gadgets::r1cs::Assignment;
use snarkvm_models::gadgets::r1cs::ConstraintSystem;
use snarkvm_models::gadgets::r1cs::LinearCombination;
use snarkvm_models::gadgets::utilities::alloc::AllocGadget;
use snarkvm_models::gadgets::utilities::boolean::AllocatedBit;
use snarkvm_models::gadgets::utilities::boolean::Boolean;
use snarkvm_models::gadgets::utilities::select::CondSelectGadget;

use std::iter;

macro_rules! mul_int_impl {
    ($($gadget: ident)*) => ($(
        /// Bitwise multiplication of two signed integer objects.
        impl<F: PrimeField> Mul<F> for $gadget {

            type ErrorType = SignedIntegerError;

            fn mul<CS: ConstraintSystem<F>>(&self, mut cs: CS, other: &Self) -> Result<Self, Self::ErrorType> {
                // pseudocode:
                //
                // res = 0;
                // for (i, bit) in other.bits.enumerate() {
                //   shifted_self = self << i;
                //
                //   if bit {
                //     res += shifted_self;
                //   }
                // }
                // return res


                // Conditionally select constant result
                let is_constant = Boolean::constant(Self::result_is_constant(&self, &other));
                let allocated_false = Boolean::from(AllocatedBit::alloc(&mut cs.ns(|| "false"), || Ok(false)).unwrap());
                let false_bit = Boolean::conditionally_select(
                    &mut cs.ns(|| "constant_or_allocated_false"),
                    &is_constant,
                    &Boolean::constant(false),
                    &allocated_false,
                )?;

                // Sign extend to double precision
                let size = <$gadget as Int>::SIZE * 2;

                let a = Boolean::sign_extend(&self.bits, size);
                let b = Boolean::sign_extend(&other.bits, size);

                let mut bits = vec![false_bit; size];

                // Compute double and add algorithm
                let mut to_add = Vec::new();
                let mut a_shifted = Vec::new();
                for (i, b_bit) in b.iter().enumerate() {
                    // double
                    a_shifted.extend(iter::repeat(false_bit).take(i));
                    a_shifted.extend(a.iter());
                    a_shifted.truncate(size);

                    // conditionally add
                    to_add.reserve(a_shifted.len());
                    for (j, a_bit) in a_shifted.iter().enumerate() {
                        let selected_bit = Boolean::conditionally_select(
                            &mut cs.ns(|| format!("select product bit {} {}", i, j)),
                            b_bit,
                            a_bit,
                            &false_bit,
                        )?;

                        to_add.push(selected_bit);
                    }

                    bits = bits.add_bits(
                        &mut cs.ns(|| format!("add bit {}", i)),
                        &to_add
                    )?;
                    let _carry = bits.pop();
                    to_add.clear();
                    a_shifted.clear();
                }
                drop(to_add);
                drop(a_shifted);

                // Compute the maximum value of the sum
                let max_bits = <$gadget as Int>::SIZE;

                // Truncate the bits to the size of the integer
                bits.truncate(max_bits);

                // Make some arbitrary bounds for ourselves to avoid overflows
                // in the scalar field
                assert!(F::Parameters::MODULUS_BITS >= max_bits as u32);

                // Accumulate the value
                let result_value = match (self.value, other.value) {
                    (Some(a), Some(b)) => {
                         // check for multiplication overflow here
                         let val = match a.checked_mul(b) {
                            Some(val) => val,
                            None => return Err(SignedIntegerError::Overflow)
                         };

                        Some(val)
                    },
                    _ => {
                        // If any of the operands have unknown value, we won't
                        // know the value of the result
                        None
                    }
                };

                // This is a linear combination that we will enforce to be zero
                let mut lc = LinearCombination::zero();

                let mut all_constants = true;


                // Iterate over each bit_gadget of result and add each bit to
                // the linear combination
                let mut coeff = F::one();
                for bit in bits {
                    match bit {
                        Boolean::Is(ref bit) => {
                            all_constants = false;

                            // Add the coeff * bit_gadget
                            lc += (coeff, bit.get_variable());
                        }
                        Boolean::Not(ref bit) => {
                            all_constants = false;

                            // Add coeff * (1 - bit_gadget) = coeff * ONE - coeff * bit_gadget
                            lc = lc + (coeff, CS::one()) - (coeff, bit.get_variable());
                        }
                        Boolean::Constant(bit) => {
                            if bit {
                                lc += (coeff, CS::one());
                            }
                        }
                    }

                    coeff.double_in_place();
                }

                // The value of the actual result is modulo 2 ^ $size
                let modular_value = result_value.map(|v| v as <$gadget as Int>::IntegerType);

                if all_constants && modular_value.is_some() {
                    // We can just return a constant, rather than
                    // unpacking the result into allocated bits.

                    return Ok(Self::constant(modular_value.unwrap()));
                }

                // Storage area for the resulting bits
                let mut result_bits = Vec::with_capacity(max_bits);

                // Allocate each bit_gadget of the result
                let mut coeff = F::one();
                for i in 0..max_bits {
                    // get bit value
                    let mask = 1 << i as <$gadget as Int>::IntegerType;

                    // Allocate the bit_gadget
                    let b = AllocatedBit::alloc(cs.ns(|| format!("result bit_gadget {}", i)), || {
                        result_value.map(|v| (v & mask) == mask).get()
                    })?;

                    // Subtract this bit_gadget from the linear combination to ensure that the sums
                    // balance out
                    lc = lc - (coeff, b.get_variable());

                    result_bits.push(b.into());

                    coeff.double_in_place();
                }

                // Enforce that the linear combination equals zero
                cs.enforce(|| "modular multiplication", |lc| lc, |lc| lc, |_| lc);

                // Discard carry bits we don't care about
                result_bits.truncate(<$gadget as Int>::SIZE);

                Ok(Self {
                    bits: result_bits,
                    value: modular_value,
                })
            }
        }
    )*)
}

mul_int_impl!(Int8 Int16 Int32 Int64 Int128);
