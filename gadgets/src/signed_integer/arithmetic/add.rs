use crate::{binary::RippleCarryAdder, errors::IntegerError, Int, Int128, Int16, Int32, Int64, Int8};

use snarkos_models::{
    curves::{fp_parameters::FpParameters, PrimeField},
    gadgets::{
        r1cs::{Assignment, ConstraintSystem, LinearCombination},
        utilities::{
            alloc::AllocGadget,
            boolean::{AllocatedBit, Boolean},
        },
    },
};

/// Addition for a signed integer gadget
pub trait Add<Rhs = Self>
where
    Self: std::marker::Sized,
{
    #[must_use]
    fn add<F: PrimeField, CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Self, IntegerError>;
}

macro_rules! add_int_impl {
    ($($gadget: ident)*) => ($(
        impl Add for $gadget {
            fn add<F: PrimeField, CS: ConstraintSystem<F>>(&self, mut cs: CS, other: &Self) -> Result<Self, IntegerError> {
                // Compute the maximum value of the sum
                let mut max_bits = <$gadget as Int>::SIZE;

                // Make some arbitrary bounds for ourselves to avoid overflows
                // in the scalar field
                assert!(F::Params::MODULUS_BITS >= max_bits as u32);


                // Accumulate the value
                let result_value = match (self.value, other.value) {
                    (Some(a), Some(b)) => {
                         // check for addition overflow here
                         let val = match a.checked_add(b) {
                            Some(val) => val,
                            None => return Err(IntegerError::Overflow)
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

                let mut bits = self.add_bits(cs.ns(|| format!("bits")), other)?;

                // we discard the carry since we check for overflow above
                let _carry = bits.pop();

                // Iterate over each bit_gadget of self and add each bit to
                // the linear combination
                let mut coeff = F::one();
                for bit in bits {
                    match bit {
                        Boolean::Is(ref bit) => {
                            all_constants = false;

                            // Add the coeff * bit_gadget
                            lc = lc + (coeff, bit.get_variable());
                        }
                        Boolean::Not(ref bit) => {
                            all_constants = false;

                            // Add coeff * (1 - bit_gadget) = coeff * ONE - coeff * bit_gadget
                            lc = lc + (coeff, CS::one()) - (coeff, bit.get_variable());
                        }
                        Boolean::Constant(bit) => {
                            if bit {
                                lc = lc + (coeff, CS::one());
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
                let mut result_bits = vec![];

                // Allocate each bit_gadget of the result
                let mut coeff = F::one();
                let mut i = 0;
                while max_bits != 0 {
                    // Allocate the bit_gadget
                    let b = AllocatedBit::alloc(cs.ns(|| format!("result bit_gadget {}", i)), || {
                        result_value.map(|v| (v >> i) & 1 == 1).get()
                    })?;

                    // Subtract this bit_gadget from the linear combination to ensure that the sums
                    // balance out
                    lc = lc - (coeff, b.get_variable());

                    result_bits.push(b.into());

                    max_bits -= 1;
                    i += 1;
                    coeff.double_in_place();
                }

                // Enforce that the linear combination equals zero
                cs.enforce(|| "modular addition", |lc| lc, |lc| lc, |_| lc);

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

add_int_impl!(Int8 Int16 Int32 Int64 Int128);
