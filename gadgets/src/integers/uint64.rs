use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Field, FpParameters, PrimeField},
    gadgets::{
        r1cs::{Assignment, ConstraintSystem, LinearCombination},
        utilities::{
            alloc::AllocGadget,
            boolean::{AllocatedBit, Boolean},
            eq::{ConditionalEqGadget, EqGadget},
            select::CondSelectGadget,
            uint8::UInt8,
            ToBytesGadget,
        },
    },
};
use snarkos_utilities::bytes::ToBytes;
use std::borrow::Borrow;

/// Represents an interpretation of 64 `Boolean` objects as an
/// unsigned integer.
#[derive(Clone, Debug)]
pub struct UInt64 {
    // Least significant bit_gadget first
    pub bits: Vec<Boolean>,
    pub negated: bool,
    pub value: Option<u64>,
}

impl UInt64 {
    /// Construct a constant `UInt64` from a `u64`
    pub fn constant(value: u64) -> Self {
        let mut bits = Vec::with_capacity(64);

        let mut tmp = value;
        for _ in 0..64 {
            if tmp & 1 == 1 {
                bits.push(Boolean::constant(true))
            } else {
                bits.push(Boolean::constant(false))
            }

            tmp >>= 1;
        }

        Self {
            bits,
            negated: false,
            value: Some(value),
        }
    }

    /// Turns this `UInt64` into its little-endian byte order representation.
    pub fn to_bits_le(&self) -> Vec<Boolean> {
        self.bits.clone()
    }

    /// Converts a little-endian byte order representation of bits into a
    /// `UInt64`.
    pub fn from_bits_le(bits: &[Boolean]) -> Self {
        assert_eq!(bits.len(), 64);

        let bits = bits.to_vec();

        let mut value = Some(0u64);
        for b in bits.iter().rev() {
            value.as_mut().map(|v| *v <<= 1);

            match b {
                &Boolean::Constant(b) => {
                    if b {
                        value.as_mut().map(|v| *v |= 1);
                    }
                }
                &Boolean::Is(ref b) => match b.get_value() {
                    Some(true) => {
                        value.as_mut().map(|v| *v |= 1);
                    }
                    Some(false) => {}
                    None => value = None,
                },
                &Boolean::Not(ref b) => match b.get_value() {
                    Some(false) => {
                        value.as_mut().map(|v| *v |= 1);
                    }
                    Some(true) => {}
                    None => value = None,
                },
            }
        }

        Self {
            value,
            negated: false,
            bits,
        }
    }

    pub fn rotr(&self, by: usize) -> Self {
        let by = by % 64;

        let new_bits = self
            .bits
            .iter()
            .skip(by)
            .chain(self.bits.iter())
            .take(64)
            .cloned()
            .collect();

        Self {
            bits: new_bits,
            negated: false,
            value: self.value.map(|v| v.rotate_right(by as u32) as u64),
        }
    }

    /// XOR this `UInt64` with another `UInt64`
    pub fn xor<F, CS>(&self, mut cs: CS, other: &Self) -> Result<Self, SynthesisError>
    where
        F: Field,
        CS: ConstraintSystem<F>,
    {
        let new_value = match (self.value, other.value) {
            (Some(a), Some(b)) => Some(a ^ b),
            _ => None,
        };

        let bits = self
            .bits
            .iter()
            .zip(other.bits.iter())
            .enumerate()
            .map(|(i, (a, b))| Boolean::xor(cs.ns(|| format!("xor of bit_gadget {}", i)), a, b))
            .collect::<Result<_, _>>()?;

        Ok(Self {
            bits,
            negated: false,
            value: new_value,
        })
    }

    /// Returns the inverse UInt64
    pub fn negate(&self) -> Self {
        Self {
            bits: self.bits.clone(),
            negated: true,
            value: self.value.clone(),
        }
    }

    /// Returns true if all bits in this UInt64 are constant
    fn is_constant(&self) -> bool {
        let mut constant = true;

        // If any bits of self are allocated bits, return false
        for bit in &self.bits {
            match *bit {
                Boolean::Is(ref _bit) => constant = false,
                Boolean::Not(ref _bit) => constant = false,
                Boolean::Constant(_bit) => {}
            }
        }

        constant
    }

    /// Returns true if both UInt64s have constant bits
    fn result_is_constant(first: &Self, second: &Self) -> bool {
        // If any bits of first are allocated bits, return false
        if !first.is_constant() {
            return false;
        }

        // If any bits of second are allocated bits, return false
        second.is_constant()
    }

    /// Perform modular addition of several `UInt64` objects.
    pub fn addmany<F, CS>(mut cs: CS, operands: &[Self]) -> Result<Self, SynthesisError>
    where
        F: PrimeField,
        CS: ConstraintSystem<F>,
    {
        // Make some arbitrary bounds for ourselves to avoid overflows
        // in the scalar field
        assert!(F::Params::MODULUS_BITS >= 128);
        assert!(operands.len() >= 2); // Weird trivial cases that should never happen

        // Compute the maximum value of the sum so we allocate enough bits for
        // the result
        let mut max_value = (operands.len() as u128) * u128::from(u64::max_value());

        // Keep track of the resulting value
        let mut result_value = Some(0u128);

        // This is a linear combination that we will enforce to be "zero"
        let mut lc = LinearCombination::zero();

        let mut all_constants = true;

        // Iterate over the operands
        for op in operands {
            // Accumulate the value
            match op.value {
                Some(val) => {
                    // Subtract or add operand
                    if op.negated {
                        // Perform subtraction
                        result_value.as_mut().map(|v| *v -= u128::from(val));
                    } else {
                        // Perform addition
                        result_value.as_mut().map(|v| *v += u128::from(val));
                    }
                }
                None => {
                    // If any of our operands have unknown value, we won't
                    // know the value of the result
                    result_value = None;
                }
            }

            // Iterate over each bit_gadget of the operand and add the operand to
            // the linear combination
            let mut coeff = F::one();
            for bit in &op.bits {
                match *bit {
                    Boolean::Is(ref bit) => {
                        all_constants = false;

                        if op.negated {
                            // Subtract coeff * bit gadget
                            lc = lc - (coeff, bit.get_variable());
                        } else {
                            // Add coeff * bit_gadget
                            lc = lc + (coeff, bit.get_variable());
                        }
                    }
                    Boolean::Not(ref bit) => {
                        all_constants = false;

                        if op.negated {
                            // subtract coeff * (1 - bit_gadget) = coeff * ONE - coeff * bit_gadget
                            lc = lc - (coeff, CS::one()) + (coeff, bit.get_variable());
                        } else {
                            // Add coeff * (1 - bit_gadget) = coeff * ONE - coeff * bit_gadget
                            lc = lc + (coeff, CS::one()) - (coeff, bit.get_variable());
                        }
                    }
                    Boolean::Constant(bit) => {
                        if bit {
                            if op.negated {
                                lc = lc - (coeff, CS::one());
                            } else {
                                lc = lc + (coeff, CS::one());
                            }
                        }
                    }
                }

                coeff.double_in_place();
            }
        }

        // The value of the actual result is modulo 2^64
        let modular_value = result_value.map(|v| v as u64);

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
        while max_value != 0 {
            // Allocate the bit_gadget
            let b = AllocatedBit::alloc(cs.ns(|| format!("result bit_gadget {}", i)), || {
                result_value.map(|v| (v >> i) & 1 == 1).get()
            })?;

            // Subtract this bit_gadget from the linear combination to ensure the sums
            // balance out
            lc = lc - (coeff, b.get_variable());

            result_bits.push(b.into());

            max_value >>= 1;
            i += 1;
            coeff.double_in_place();
        }

        // Enforce that the linear combination equals zero
        cs.enforce(|| "modular addition", |lc| lc, |lc| lc, |_| lc);

        // Discard carry bits that we don't care about
        result_bits.truncate(64);

        Ok(Self {
            bits: result_bits,
            negated: false,
            value: modular_value,
        })
    }

    /// Perform modular subtraction of two `UInt64` objects.
    pub fn sub<F: PrimeField, CS: ConstraintSystem<F>>(
        &self,
        mut cs: CS,
        other: &Self,
    ) -> Result<Self, SynthesisError> {
        // pseudocode:
        //
        // a - b
        // a + (-b)

        Self::addmany(&mut cs.ns(|| "add_not"), &[self.clone(), other.negate()])
    }

    /// Perform unsafe subtraction of two `UInt64` objects which returns 0 if overflowed
    fn sub_unsafe<F: PrimeField, CS: ConstraintSystem<F>>(
        &self,
        mut cs: CS,
        other: &Self,
    ) -> Result<Self, SynthesisError> {
        match (self.value, other.value) {
            (Some(val1), Some(val2)) => {
                // Check for overflow
                if val1 < val2 {
                    // Instead of erroring, return 0

                    if Self::result_is_constant(&self, &other) {
                        // Return constant 0u64
                        Ok(Self::constant(0u64))
                    } else {
                        // Return allocated 0u64
                        let result_value = Some(0u64);
                        let modular_value = result_value.map(|v| v as u64);

                        // Storage area for the resulting bits
                        let mut result_bits = vec![];

                        // This is a linear combination that we will enforce to be "zero"
                        let mut lc = LinearCombination::zero();

                        // Allocate each bit_gadget of the result
                        let mut coeff = F::one();
                        for i in 0..64 {
                            // Allocate the bit_gadget
                            let b = AllocatedBit::alloc(
                                cs.ns(|| format!("result bit_gadget {}", i)),
                                || result_value.map(|v| (v >> i) & 1 == 1).get(),
                            )?;

                            // Subtract this bit_gadget from the linear combination to ensure the sums
                            // balance out
                            lc = lc - (coeff, b.get_variable());

                            result_bits.push(b.into());

                            coeff.double_in_place();
                        }

                        // Enforce that the linear combination equals zero
                        cs.enforce(|| "unsafe subtraction", |lc| lc, |lc| lc, |_| lc);

                        // Discard carry bits that we don't care about
                        result_bits.truncate(64);

                        Ok(Self {
                            bits: result_bits,
                            negated: false,
                            value: modular_value,
                        })
                    }
                } else {
                    // Perform subtraction
                    self.sub(&mut cs.ns(|| ""), &other)
                }
            }
            (_, _) => {
                // If either of our operands have unknown value, we won't
                // know the value of the result
                return Err(SynthesisError::AssignmentMissing);
            }
        }
    }

    /// Bitwise multiplication of two `UInt64` objects.
    /// Reference: https://en.wikipedia.org/wiki/Binary_multiplier
    pub fn mul<F: PrimeField, CS: ConstraintSystem<F>>(
        &self,
        mut cs: CS,
        other: &Self,
    ) -> Result<Self, SynthesisError> {
        // pseudocode:
        //
        // res = 0;
        // shifted_self = self;
        // for bit in other.bits {
        //   if bit {
        //     res += shifted_self;
        //   }
        //   shifted_self = shifted_self << 1;
        // }
        // return res

        let is_constant = Boolean::constant(Self::result_is_constant(&self, &other));
        let constant_result = Self::constant(0u64);
        let allocated_result = Self::alloc(&mut cs.ns(|| "allocated_1u64"), || Ok(0u64))?;
        let zero_result = Self::conditionally_select(
            &mut cs.ns(|| "constant_or_allocated"),
            &is_constant,
            &constant_result,
            &allocated_result,
        )?;

        let mut left_shift = self.clone();

        let partial_products = other
            .bits
            .iter()
            .enumerate()
            .map(|(i, bit)| {
                let current_left_shift = left_shift.clone();
                left_shift = Self::addmany(
                    &mut cs.ns(|| format!("shift_left_{}", i)),
                    &[left_shift.clone(), left_shift.clone()],
                )
                .unwrap();

                Self::conditionally_select(
                    &mut cs.ns(|| format!("calculate_product_{}", i)),
                    &bit,
                    &current_left_shift,
                    &zero_result,
                )
                .unwrap()
            })
            .collect::<Vec<Self>>();

        Self::addmany(
            &mut cs.ns(|| format!("partial_products")),
            &partial_products,
        )
    }

    /// Perform long division of two `UInt64` objects.
    /// Reference: https://en.wikipedia.org/wiki/Division_algorithm
    pub fn div<F: PrimeField, CS: ConstraintSystem<F>>(
        &self,
        mut cs: CS,
        other: &Self,
    ) -> Result<Self, SynthesisError> {
        // pseudocode:
        //
        // if D = 0 then error(DivisionByZeroException) end
        // Q := 0                  -- Initialize quotient and remainder to zero
        // R := 0
        // for i := n − 1 .. 0 do  -- Where n is number of bits in N
        //   R := R << 1           -- Left-shift R by 1 bit
        //   R(0) := N(i)          -- Set the least-significant bit of R equal to bit i of the numerator
        //   if R ≥ D then
        //     R := R − D
        //     Q(i) := 1
        //   end
        // end

        if other.eq(&Self::constant(0u64)) {
            return Err(SynthesisError::DivisionByZero);
        }

        let is_constant = Boolean::constant(Self::result_is_constant(&self, &other));

        let allocated_true =
            Boolean::from(AllocatedBit::alloc(&mut cs.ns(|| "true"), || Ok(true)).unwrap());
        let true_bit = Boolean::conditionally_select(
            &mut cs.ns(|| "constant_or_allocated_true"),
            &is_constant,
            &Boolean::constant(true),
            &allocated_true,
        )?;

        let allocated_one = Self::alloc(&mut cs.ns(|| "one"), || Ok(1u64))?;
        let one = Self::conditionally_select(
            &mut cs.ns(|| "constant_or_allocated_1u64"),
            &is_constant,
            &Self::constant(1u64),
            &allocated_one,
        )?;

        let allocated_zero = Self::alloc(&mut cs.ns(|| "zero"), || Ok(0u64))?;
        let zero = Self::conditionally_select(
            &mut cs.ns(|| "constant_or_allocated_0u64"),
            &is_constant,
            &Self::constant(0u64),
            &allocated_zero,
        )?;

        let self_is_zero = Boolean::Constant(self.eq(&Self::constant(0u64)));
        let mut quotient = zero.clone();
        let mut remainder = zero.clone();

        for (i, bit) in self.bits.iter().rev().enumerate() {
            // Left shift remainder by 1
            remainder = Self::addmany(
                &mut cs.ns(|| format!("shift_left_{}", i)),
                &[remainder.clone(), remainder.clone()],
            )?;

            // Set the least-significant bit of remainder to bit i of the numerator
            let bit_is_true = Boolean::constant(bit.eq(&Boolean::constant(true)));
            let new_remainder = Self::addmany(
                &mut cs.ns(|| format!("set_remainder_bit_{}", i)),
                &[remainder.clone(), one.clone()],
            )?;

            remainder = Self::conditionally_select(
                &mut cs.ns(|| format!("increment_or_remainder_{}", i)),
                &bit_is_true,
                &new_remainder,
                &remainder,
            )?;

            // Greater than or equal to:
            //   R >= D
            //   (R == D) || (R > D)
            //   (R == D) || ((R !=D) && ((R - D) != 0))
            //
            //  (R > D)                     checks subtraction overflow before evaluation
            //  (R != D) && ((R - D) != 0)  instead evaluate subtraction and check for overflow after

            let no_remainder = Boolean::constant(remainder.eq(&other));
            let subtraction =
                remainder.sub_unsafe(&mut cs.ns(|| format!("subtract_divisor_{}", i)), &other)?;
            let sub_is_zero = Boolean::constant(subtraction.eq(&Self::constant(0)));
            let cond1 = Boolean::and(
                &mut cs.ns(|| format!("cond_1_{}", i)),
                &no_remainder.not(),
                &sub_is_zero.not(),
            )?;
            let cond2 = Boolean::or(
                &mut cs.ns(|| format!("cond_2_{}", i)),
                &no_remainder,
                &cond1,
            )?;

            remainder = Self::conditionally_select(
                &mut cs.ns(|| format!("subtract_or_same_{}", i)),
                &cond2,
                &subtraction,
                &remainder,
            )?;

            let index = 63 - i as usize;
            let bit_value = 1u64 << (index as u64);
            let mut new_quotient = quotient.clone();
            new_quotient.bits[index] = true_bit.clone();
            new_quotient.value = Some(new_quotient.value.unwrap() + bit_value);

            quotient = Self::conditionally_select(
                &mut cs.ns(|| format!("set_bit_or_same_{}", i)),
                &cond2,
                &new_quotient,
                &quotient,
            )?;
        }
        Self::conditionally_select(
            &mut cs.ns(|| "self_or_quotient"),
            &self_is_zero,
            self,
            &quotient,
        )
    }

    /// Bitwise multiplication of two `UInt64` objects.
    /// Reference: /snarkOS/models/src/curves/field.rs
    pub fn pow<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &self,
        mut cs: CS,
        other: &Self,
    ) -> Result<Self, SynthesisError> {
        // let mut res = Self::one();
        //
        // let mut found_one = false;
        //
        // for i in BitIterator::new(exp) {
        //     if !found_one {
        //         if i {
        //             found_one = true;
        //         } else {
        //             continue;
        //         }
        //     }
        //
        //     res.square_in_place();
        //
        //     if i {
        //         res *= self;
        //     }
        // }
        // res

        let is_constant = Boolean::constant(Self::result_is_constant(&self, &other));
        let constant_result = Self::constant(1u64);
        let allocated_result = Self::alloc(&mut cs.ns(|| "allocated_1u64"), || Ok(1u64))?;
        let mut result = Self::conditionally_select(
            &mut cs.ns(|| "constant_or_allocated"),
            &is_constant,
            &constant_result,
            &allocated_result,
        )?;

        for (i, bit) in other.bits.iter().rev().enumerate() {
            let found_one = Boolean::Constant(result.eq(&Self::constant(1u64)));
            let cond1 = Boolean::and(cs.ns(|| format!("found_one_{}", i)), &bit.not(), &found_one)?;
            let square = result
                .mul(cs.ns(|| format!("square_{}", i)), &result)
                .unwrap();

            result = Self::conditionally_select(
                &mut cs.ns(|| format!("result_or_sqaure_{}", i)),
                &cond1,
                &result,
                &square,
            )?;

            let mul_by_self = result
                .mul(cs.ns(|| format!("multiply_by_self_{}", i)), &self)
                .unwrap();

            result = Self::conditionally_select(
                &mut cs.ns(|| format!("mul_by_self_or_result_{}", i)),
                &bit,
                &mul_by_self,
                &result,
            )?;
        }

        Ok(result)
    }
}

impl<F: Field> ToBytesGadget<F> for UInt64 {
    #[inline]
    fn to_bytes<CS: ConstraintSystem<F>>(&self, _cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
        let value_chunks = match self.value.map(|val| {
            let mut bytes = [0u8; 8];
            val.write(bytes.as_mut()).unwrap();
            bytes
        }) {
            Some(chunks) => [
                Some(chunks[0]),
                Some(chunks[1]),
                Some(chunks[2]),
                Some(chunks[3]),
            ],
            None => [None, None, None, None],
        };
        let mut bytes = Vec::new();
        for (i, chunk8) in self.to_bits_le().chunks(8).into_iter().enumerate() {
            let byte = UInt8 {
                bits: chunk8.to_vec(),
                negated: false,
                value: value_chunks[i],
            };
            bytes.push(byte);
        }

        Ok(bytes)
    }

    fn to_bytes_strict<CS: ConstraintSystem<F>>(
        &self,
        cs: CS,
    ) -> Result<Vec<UInt8>, SynthesisError> {
        self.to_bytes(cs)
    }
}

impl PartialEq for UInt64 {
    fn eq(&self, other: &Self) -> bool {
        !self.value.is_none() && !other.value.is_none() && self.value == other.value
    }
}

impl Eq for UInt64 {}

impl<F: Field> EqGadget<F> for UInt64 {}

impl<F: Field> ConditionalEqGadget<F> for UInt64 {
    fn conditional_enforce_equal<CS: ConstraintSystem<F>>(
        &self,
        mut cs: CS,
        other: &Self,
        condition: &Boolean,
    ) -> Result<(), SynthesisError> {
        for (i, (a, b)) in self.bits.iter().zip(&other.bits).enumerate() {
            a.conditional_enforce_equal(
                &mut cs.ns(|| format!("uint64_equal_{}", i)),
                b,
                condition,
            )?;
        }
        Ok(())
    }

    fn cost() -> usize {
        64 * <Boolean as ConditionalEqGadget<F>>::cost()
    }
}

impl<F: PrimeField> CondSelectGadget<F> for UInt64 {
    fn conditionally_select<CS: ConstraintSystem<F>>(
        mut cs: CS,
        cond: &Boolean,
        first: &Self,
        second: &Self,
    ) -> Result<Self, SynthesisError> {
        if let Boolean::Constant(cond) = *cond {
            if cond {
                Ok(first.clone())
            } else {
                Ok(second.clone())
            }
        } else {
            let mut is_negated = false;

            let result_val = cond.get_value().and_then(|c| {
                if c {
                    is_negated = first.negated;
                    first.value
                } else {
                    is_negated = second.negated;
                    second.value
                }
            });

            let mut result = Self::alloc(cs.ns(|| "cond_select_result"), || {
                result_val.get().map(|v| v)
            })?;

            result.negated = is_negated;

            let expected_bits = first
                .bits
                .iter()
                .zip(&second.bits)
                .enumerate()
                .map(|(i, (a, b))| {
                    Boolean::conditionally_select(
                        &mut cs.ns(|| format!("uint64_cond_select_{}", i)),
                        cond,
                        a,
                        b,
                    )
                    .unwrap()
                })
                .collect::<Vec<Boolean>>();

            for (i, (actual, expected)) in result
                .to_bits_le()
                .iter()
                .zip(expected_bits.iter())
                .enumerate()
            {
                actual.enforce_equal(
                    &mut cs.ns(|| format!("selected_result_bit_{}", i)),
                    expected,
                )?;
            }

            Ok(result)
        }
    }

    fn cost() -> usize {
        64 * (<Boolean as ConditionalEqGadget<F>>::cost()
            + <Boolean as CondSelectGadget<F>>::cost())
    }
}

impl<F: Field> AllocGadget<u64, F> for UInt64 {
    fn alloc<Fn, T, CS: ConstraintSystem<F>>(
        mut cs: CS,
        value_gen: Fn,
    ) -> Result<Self, SynthesisError>
    where
        Fn: FnOnce() -> Result<T, SynthesisError>,
        T: Borrow<u64>,
    {
        let value = value_gen().map(|val| *val.borrow());
        let values = match value {
            Ok(mut val) => {
                let mut v = Vec::with_capacity(64);

                for _ in 0..64 {
                    v.push(Some(val & 1 == 1));
                    val >>= 1;
                }

                v
            }
            _ => vec![None; 64],
        };

        let bits = values
            .into_iter()
            .enumerate()
            .map(|(i, v)| {
                Ok(Boolean::from(AllocatedBit::alloc(
                    &mut cs.ns(|| format!("allocated bit_gadget {}", i)),
                    || v.ok_or(SynthesisError::AssignmentMissing),
                )?))
            })
            .collect::<Result<Vec<_>, SynthesisError>>()?;

        Ok(Self {
            bits,
            negated: false,
            value: value.ok(),
        })
    }

    fn alloc_input<Fn, T, CS: ConstraintSystem<F>>(
        mut cs: CS,
        value_gen: Fn,
    ) -> Result<Self, SynthesisError>
    where
        Fn: FnOnce() -> Result<T, SynthesisError>,
        T: Borrow<u64>,
    {
        let value = value_gen().map(|val| *val.borrow());
        let values = match value {
            Ok(mut val) => {
                let mut v = Vec::with_capacity(64);

                for _ in 0..64 {
                    v.push(Some(val & 1 == 1));
                    val >>= 1;
                }

                v
            }
            _ => vec![None; 64],
        };

        let bits = values
            .into_iter()
            .enumerate()
            .map(|(i, v)| {
                Ok(Boolean::from(AllocatedBit::alloc_input(
                    &mut cs.ns(|| format!("allocated bit_gadget {}", i)),
                    || v.ok_or(SynthesisError::AssignmentMissing),
                )?))
            })
            .collect::<Result<Vec<_>, SynthesisError>>()?;

        Ok(Self {
            bits,
            negated: false,
            value: value.ok(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use snarkos_models::{
        curves::Field,
        gadgets::{
            r1cs::{Fr, TestConstraintSystem},
            utilities::boolean::Boolean,
        },
    };

    use rand::{Rng, SeedableRng};
    use rand_xorshift::XorShiftRng;
    use std::convert::TryInto;

    fn check_all_constant_bits(mut expected: u64, actual: UInt64) {
        for b in actual.bits.iter() {
            match b {
                &Boolean::Is(_) => panic!(),
                &Boolean::Not(_) => panic!(),
                &Boolean::Constant(b) => {
                    assert!(b == (expected & 1 == 1));
                }
            }

            expected >>= 1;
        }
    }

    fn check_all_allocated_bits(mut expected: u64, actual: UInt64) {
        for b in actual.bits.iter() {
            match b {
                &Boolean::Is(ref b) => {
                    assert!(b.get_value().unwrap() == (expected & 1 == 1));
                }
                &Boolean::Not(ref b) => {
                    assert!(!b.get_value().unwrap() == (expected & 1 == 1));
                }
                &Boolean::Constant(_) => unreachable!(),
            }

            expected >>= 1;
        }
    }

    #[test]
    fn test_uint64_from_bits() {
        let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

        for _ in 0..1000 {
            let v = (0..64)
                .map(|_| Boolean::constant(rng.gen()))
                .collect::<Vec<_>>();

            let b = UInt64::from_bits_le(&v);

            for (i, bit_gadget) in b.bits.iter().enumerate() {
                match bit_gadget {
                    &Boolean::Constant(bit_gadget) => {
                        assert!(bit_gadget == ((b.value.unwrap() >> i) & 1 == 1));
                    }
                    _ => unreachable!(),
                }
            }

            let expected_to_be_same = b.to_bits_le();

            for x in v.iter().zip(expected_to_be_same.iter()) {
                match x {
                    (&Boolean::Constant(true), &Boolean::Constant(true)) => {}
                    (&Boolean::Constant(false), &Boolean::Constant(false)) => {}
                    _ => unreachable!(),
                }
            }
        }
    }

    #[test]
    fn test_uint64_xor() {
        let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

        for _ in 0..1000 {
            let mut cs = TestConstraintSystem::<Fr>::new();

            let a: u64 = rng.gen();
            let b: u64 = rng.gen();
            let c: u64 = rng.gen();

            let mut expected = a ^ b ^ c;

            let a_bit = UInt64::alloc(cs.ns(|| "a_bit"), || Ok(a)).unwrap();
            let b_bit = UInt64::constant(b);
            let c_bit = UInt64::alloc(cs.ns(|| "c_bit"), || Ok(c)).unwrap();

            let r = a_bit.xor(cs.ns(|| "first xor"), &b_bit).unwrap();
            let r = r.xor(cs.ns(|| "second xor"), &c_bit).unwrap();

            assert!(cs.is_satisfied());

            assert!(r.value == Some(expected));

            for b in r.bits.iter() {
                match b {
                    &Boolean::Is(ref b) => {
                        assert!(b.get_value().unwrap() == (expected & 1 == 1));
                    }
                    &Boolean::Not(ref b) => {
                        assert!(!b.get_value().unwrap() == (expected & 1 == 1));
                    }
                    &Boolean::Constant(b) => {
                        assert!(b == (expected & 1 == 1));
                    }
                }

                expected >>= 1;
            }
        }
    }

    #[test]
    fn test_uint64_rotr() {
        let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

        let mut num = rng.gen();

        let a = UInt64::constant(num);

        for i in 0..64 {
            let b = a.rotr(i);

            assert!(b.value.unwrap() == num);

            let mut tmp = num;
            for b in &b.bits {
                match b {
                    &Boolean::Constant(b) => {
                        assert_eq!(b, tmp & 1 == 1);
                    }
                    _ => unreachable!(),
                }

                tmp >>= 1;
            }

            num = num.rotate_right(1);
        }
    }

    #[test]
    fn test_uint64_addmany_constants() {
        let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

        for _ in 0..1000 {
            let mut cs = TestConstraintSystem::<Fr>::new();

            let a: u64 = rng.gen();
            let b: u64 = rng.gen();
            let c: u64 = rng.gen();

            let a_bit = UInt64::constant(a);
            let b_bit = UInt64::constant(b);
            let c_bit = UInt64::constant(c);

            let expected = a.wrapping_add(b).wrapping_add(c);

            let r = UInt64::addmany(cs.ns(|| "addition"), &[a_bit, b_bit, c_bit]).unwrap();

            assert!(r.value == Some(expected));

            check_all_constant_bits(expected, r);
        }
    }

    #[test]
    fn test_uint64_addmany() {
        let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

        for _ in 0..1000 {
            let mut cs = TestConstraintSystem::<Fr>::new();

            let a: u64 = rng.gen();
            let b: u64 = rng.gen();
            let c: u64 = rng.gen();
            let d: u64 = rng.gen();

            let expected = (a ^ b).wrapping_add(c).wrapping_add(d);

            let a_bit = UInt64::alloc(cs.ns(|| "a_bit"), || Ok(a)).unwrap();
            let b_bit = UInt64::constant(b);
            let c_bit = UInt64::constant(c);
            let d_bit = UInt64::alloc(cs.ns(|| "d_bit"), || Ok(d)).unwrap();

            let r = a_bit.xor(cs.ns(|| "xor"), &b_bit).unwrap();
            let r = UInt64::addmany(cs.ns(|| "addition"), &[r, c_bit, d_bit]).unwrap();

            assert!(cs.is_satisfied());

            assert!(r.value == Some(expected));

            check_all_allocated_bits(expected, r);

            // Flip a bit_gadget and see if the addition constraint still works
            if cs.get("addition/result bit_gadget 0/boolean").is_zero() {
                cs.set("addition/result bit_gadget 0/boolean", Field::one());
            } else {
                cs.set("addition/result bit_gadget 0/boolean", Field::zero());
            }

            assert!(!cs.is_satisfied());
        }
    }

    #[test]
    fn test_uint64_sub_constants() {
        let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

        for _ in 0..1000 {
            let mut cs = TestConstraintSystem::<Fr>::new();

            let a: u64 = rng.gen_range(u64::max_value() / 2u64, u64::max_value());
            let b: u64 = rng.gen_range(0u64, u64::max_value() / 2u64);

            let a_bit = UInt64::constant(a);
            let b_bit = UInt64::constant(b);

            let expected = a.wrapping_sub(b);

            let r = a_bit.sub(cs.ns(|| "subtraction"), &b_bit).unwrap();

            assert!(r.value == Some(expected));

            check_all_constant_bits(expected, r);
        }
    }

    #[test]
    fn test_uint64_sub() {
        let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

        for _ in 0..1000 {
            let mut cs = TestConstraintSystem::<Fr>::new();

            let a: u64 = rng.gen_range(u64::max_value() / 2u64, u64::max_value());
            let b: u64 = rng.gen_range(0u64, u64::max_value() / 2u64);

            let expected = a.wrapping_sub(b);

            let a_bit = UInt64::alloc(cs.ns(|| "a_bit"), || Ok(a)).unwrap();
            let b_bit = if b > u64::max_value() / 4 {
                UInt64::constant(b)
            } else {
                UInt64::alloc(cs.ns(|| "b_bit"), || Ok(b)).unwrap()
            };

            let r = a_bit.sub(cs.ns(|| "subtraction"), &b_bit).unwrap();

            assert!(cs.is_satisfied());

            assert!(r.value == Some(expected));

            check_all_allocated_bits(expected, r);

            // Flip a bit_gadget and see if the subtraction constraint still works
            if cs
                .get("subtraction/add_not/result bit_gadget 0/boolean")
                .is_zero()
            {
                cs.set(
                    "subtraction/add_not/result bit_gadget 0/boolean",
                    Field::one(),
                );
            } else {
                cs.set(
                    "subtraction/add_not/result bit_gadget 0/boolean",
                    Field::zero(),
                );
            }

            assert!(!cs.is_satisfied());
        }
    }

    #[test]
    fn test_uint64_mul_constants() {
        let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

        for _ in 0..1000 {
            let mut cs = TestConstraintSystem::<Fr>::new();

            let a: u64 = rng.gen();
            let b: u64 = rng.gen();

            let a_bit = UInt64::constant(a);
            let b_bit = UInt64::constant(b);

            let expected = a.wrapping_mul(b);

            let r = a_bit.mul(cs.ns(|| "multiply"), &b_bit).unwrap();

            assert!(r.value == Some(expected));

            check_all_constant_bits(expected, r);
        }
    }

    #[test]
    fn test_uint64_mul() {
        let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

        for _ in 0..100 {
            let mut cs = TestConstraintSystem::<Fr>::new();

            let a: u64 = rng.gen();
            let b: u64 = rng.gen();

            let expected = a.wrapping_mul(b);

            let a_bit = UInt64::alloc(cs.ns(|| "a_bit"), || Ok(a)).unwrap();
            let b_bit = if b > (u64::max_value() / 2) {
                UInt64::constant(b)
            } else {
                UInt64::alloc(cs.ns(|| "b_bit"), || Ok(b)).unwrap()
            };

            let r = a_bit.mul(cs.ns(|| "multiplication"), &b_bit).unwrap();

            assert!(cs.is_satisfied());

            assert!(r.value == Some(expected));

            check_all_allocated_bits(expected, r);

            // Flip a bit_gadget and see if the multiplication constraint still works
            if cs
                .get("multiplication/partial_products/result bit_gadget 0/boolean")
                .is_zero()
            {
                cs.set(
                    "multiplication/partial_products/result bit_gadget 0/boolean",
                    Field::one(),
                );
            } else {
                cs.set(
                    "multiplication/partial_products/result bit_gadget 0/boolean",
                    Field::zero(),
                );
            }

            assert!(!cs.is_satisfied());
        }
    }

    #[test]
    fn test_uint64_div_constants() {
        let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

        for _ in 0..1000 {
            let mut cs = TestConstraintSystem::<Fr>::new();

            let a: u64 = rng.gen();
            let b: u64 = rng.gen();

            let a_bit = UInt64::constant(a);
            let b_bit = UInt64::constant(b);

            let expected = a.wrapping_div(b);

            let r = a_bit.div(cs.ns(|| "division"), &b_bit).unwrap();

            assert!(r.value == Some(expected));

            check_all_constant_bits(expected, r);
        }
    }

    #[test]
    fn test_uint64_div() {
        let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

        for _ in 0..100 {
            let mut cs = TestConstraintSystem::<Fr>::new();

            let a: u64 = rng.gen();
            let b: u64 = rng.gen();

            let expected = a.wrapping_div(b);

            let a_bit = UInt64::alloc(cs.ns(|| "a_bit"), || Ok(a)).unwrap();
            let b_bit = if b > u64::max_value() / 2 {
                UInt64::constant(b)
            } else {
                UInt64::alloc(cs.ns(|| "b_bit"), || Ok(b)).unwrap()
            };

            let r = a_bit.div(cs.ns(|| "division"), &b_bit).unwrap();

            assert!(cs.is_satisfied());

            assert!(r.value == Some(expected));

            check_all_allocated_bits(expected, r);

            // Flip a bit_gadget and see if the division constraint still works
            if cs
                .get("division/subtract_divisor_0/result bit_gadget 0/boolean")
                .is_zero()
            {
                cs.set(
                    "division/subtract_divisor_0/result bit_gadget 0/boolean",
                    Field::one(),
                );
            } else {
                cs.set(
                    "division/subtract_divisor_0/result bit_gadget 0/boolean",
                    Field::zero(),
                );
            }

            assert!(!cs.is_satisfied());
        }
    }

    #[test]
    fn test_uint64_pow_constants() {
        let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

        for _ in 0..100 {
            let mut cs = TestConstraintSystem::<Fr>::new();

            let a: u64 = rng.gen_range(0, u64::from(u16::max_value()));
            let b: u64 = rng.gen_range(0, 4);

            let a_bit = UInt64::constant(a);
            let b_bit = UInt64::constant(b);

            let expected = a.wrapping_pow(b.try_into().unwrap());

            let r = a_bit.pow(cs.ns(|| "exponentiation"), &b_bit).unwrap();

            assert!(r.value == Some(expected));

            check_all_constant_bits(expected, r);
        }
    }

    #[test]
    fn test_uint64_pow() {
        let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

        for _ in 0..4 {
            let mut cs = TestConstraintSystem::<Fr>::new();

            let a: u64 = rng.gen_range(0, u64::from(u16::max_value()));
            let b: u64 = rng.gen_range(0, 4);

            let expected = a.wrapping_pow(b.try_into().unwrap());

            let a_bit = UInt64::alloc(cs.ns(|| "a_bit"), || Ok(a)).unwrap();
            let b_bit = UInt64::alloc(cs.ns(|| "b_bit"), || Ok(b)).unwrap();

            println!("num constraints before {}", cs.num_constraints());

            let r = a_bit.pow(cs.ns(|| "exponentiation"), &b_bit).unwrap();
            println!("num constraints after {}\n", cs.num_constraints());

            assert!(cs.is_satisfied());

            assert!(r.value == Some(expected));

            check_all_allocated_bits(expected, r);

            // Flip a bit_gadget and see if the exponentiation constraint still works
            if cs
                .get("exponentiation/multiply_by_self_0/partial_products/result bit_gadget 0/boolean")
                .is_zero()
            {
                cs.set(
                    "exponentiation/multiply_by_self_0/partial_products/result bit_gadget 0/boolean",
                    Field::one(),
                );
            } else {
                cs.set(
                    "exponentiation/multiply_by_self_0/partial_products/result bit_gadget 0/boolean",
                    Field::zero(),
                );
            }

            assert!(!cs.is_satisfied());
        }
    }
}
