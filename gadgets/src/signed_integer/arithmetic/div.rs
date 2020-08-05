use crate::{
    arithmetic::{Add, Div, Neg, Sub},
    bits::ComparatorGadget,
    errors::SignedIntegerError,
    Int,
    Int128,
    Int16,
    Int32,
    Int64,
    Int8,
};
use snarkos_models::{
    curves::PrimeField,
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{
            alloc::AllocGadget,
            boolean::{AllocatedBit, Boolean},
            eq::EvaluateEqGadget,
            select::CondSelectGadget,
        },
    },
};

macro_rules! div_int_impl {
    ($($gadget:ident),*) => ($(
        impl<F: PrimeField> Div<F> for $gadget {
            type ErrorType = SignedIntegerError;

            fn div<CS: ConstraintSystem<F>>(
                &self,
                mut cs: CS,
                other: &Self
            ) -> Result<Self, Self::ErrorType> {
                // N / D pseudocode:
                //
                // if D = 0 then error(DivisionByZeroException) end
                //
                // positive = msb(N) == msb(D) -- if msb's equal, return positive result
                //
                // Q := 0                  -- Initialize quotient and remainder to zero
                // R := 0
                //
                // for i := n − 1 .. 0 do  -- Where n is number of bits in N
                //   R := R << 1           -- Left-shift R by 1 bit
                //   R(0) := N(i)          -- Set the least-significant bit of R equal to bit i of the numerator
                //   if R ≥ D then
                //     R := R − D
                //     Q(i) := 1
                //   end
                // end
                //
                // if positive then           -- positive result
                //    Q
                // else
                //    !Q                      -- negative result

                if other.eq(&Self::constant(0 as <$gadget as Int>::IntegerType)) {
                    return Err(SignedIntegerError::DivisionByZero);
                }

                let is_constant = Boolean::constant(Self::result_is_constant(&self, &other));

                let allocated_true = Boolean::from(AllocatedBit::alloc(&mut cs.ns(|| "true"), || Ok(true)).unwrap());
                let true_bit = Boolean::conditionally_select(
                    &mut cs.ns(|| "constant_or_allocated_true"),
                    &is_constant,
                    &Boolean::constant(true),
                    &allocated_true,
                )?;

                let allocated_one = Self::alloc(&mut cs.ns(|| "one"), || Ok(1 as <$gadget as Int>::IntegerType))?;
                let one = Self::conditionally_select(
                    &mut cs.ns(|| "constant_or_allocated_one"),
                    &is_constant,
                    &Self::constant(1 as <$gadget as Int>::IntegerType),
                    &allocated_one,
                )?;

                let allocated_zero = Self::alloc(&mut cs.ns(|| "zero"), || Ok(0 as <$gadget as Int>::IntegerType))?;
                let zero = Self::conditionally_select(
                    &mut cs.ns(|| "constant_or_allocated_zero"),
                    &is_constant,
                    &Self::constant(0 as <$gadget as Int>::IntegerType),
                    &allocated_zero,
                )?;

                // if the numerator is 0, return 0
                let self_is_zero = Boolean::Constant(self.eq(&Self::constant(0 as <$gadget as Int>::IntegerType)));

                // if other is the minimum number, the result will be zero or one
                // -128 / -128 = 1
                // x / -128 = 0 fractional result rounds to 0
                let min = Self::constant(<$gadget as Int>::IntegerType::MIN);
                let other_is_min = other.evaluate_equal(
                    &mut cs.ns(|| "other_min_check"),
                    &min
                )?;
                let self_is_min = self.evaluate_equal(
                    &mut cs.ns(|| "self_min_check"),
                    &min
                )?;
                let both_min = Boolean::and(
                    &mut cs.ns(|| "both_min"),
                    &other_is_min,
                    &self_is_min
                )?;


                // if other is the minimum, set other to -1 so the calculation will not fail
                let negative_one = one.neg(&mut cs.ns(|| "negative_one"))?;
                let a_valid = min.add(&mut cs.ns(||"a_valid"), &one);
                let a_set = Self::conditionally_select(
                    &mut cs.ns(|| "a_set"),
                    &self_is_min,
                    &a_valid?,
                    &self
                )?;

                let b_set = Self::conditionally_select(
                    &mut cs.ns(|| "b_set"),
                    &other_is_min,
                    &negative_one,
                    &other
                )?;

                // If the most significant bits of both numbers are equal, the quotient will be positive
                let b_msb = other.bits.last().unwrap();
                let a_msb = self.bits.last().unwrap();
                let positive = a_msb.evaluate_equal(cs.ns(|| "compare_msb"), &b_msb)?;

                // Get the absolute value of each number
                let a_comp = a_set.neg(&mut cs.ns(|| "a_neg"))?;
                let a = Self::conditionally_select(
                    &mut cs.ns(|| "a_abs"),
                    &a_msb,
                    &a_comp,
                    &self
                )?;

                let b_comp = b_set.neg(&mut cs.ns(|| "b_neg"))?;
                let b = Self::conditionally_select(
                    &mut cs.ns(|| "b_abs"),
                    &b_msb,
                    &b_comp,
                    &b_set,
                )?;

                let mut q = zero.clone();
                let mut r = zero.clone();

                let mut index = <$gadget as Int>::SIZE - 1 as usize;
                let mut bit_value = (1 as <$gadget as Int>::IntegerType) << ((index - 1) as <$gadget as Int>::IntegerType);

                for (i, bit) in a.bits.iter().rev().enumerate().skip(1) {

                    // Left shift remainder by 1
                    r = r.add(
                        &mut cs.ns(|| format!("shift_left_{}", i)),
                        &r
                    )?;

                    // Set the least-significant bit of remainder to bit i of the numerator
                    let r_new = r.add(
                        &mut cs.ns(|| format!("set_remainder_bit_{}", i)),
                        &one.clone(),
                    )?;

                    r = Self::conditionally_select(
                        &mut cs.ns(|| format!("increment_or_remainder_{}", i)),
                        &bit,
                        &r_new,
                        &r
                    )?;

                    let can_sub = r.greater_than_or_equal(
                        &mut cs.ns(|| format!("compare_remainder_{}", i)),
                        &b
                    )?;

                    let sub = r.sub(
                        &mut cs.ns(|| format!("subtract_divisor_{}", i)),
                        &b
                    );

                    r = Self::conditionally_select(
                        &mut cs.ns(|| format!("subtract_or_same_{}", i)),
                        &can_sub,
                        &sub?,
                        &r
                    )?;

                    index = index - 1;

                    let mut q_new = q.clone();
                    q_new.bits[index] = true_bit.clone();
                    q_new.value = Some(q_new.value.unwrap() + bit_value);

                    bit_value = (bit_value >> 1);

                    q = Self::conditionally_select(
                        &mut cs.ns(|| format!("set_bit_or_same_{}", i)),
                        &can_sub,
                        &q_new,
                        &q,
                    )?;

                }

                let q_neg = q.neg(&mut cs.ns(|| "negate"))?;

                q = Self::conditionally_select(
                    &mut cs.ns(|| "positive or negative"),
                    &positive,
                    &q,
                    &q_neg,
                )?;

                // add if we computed using the minimum value
                // if q is the maximum value, add zero
                // else add one
                let max = Self::constant(<$gadget as Int>::IntegerType::MAX);
                let q_is_max = q.evaluate_equal(cs.ns(|| "compare_max"), &max)?;
                let to_add = Self::conditionally_select(
                    &mut cs.ns(|| "select_add"),
                    &q_is_max,
                    &zero,
                    &one,
                )?;

                let q_valid = q.add(&mut cs.ns(||"q_valid"), &to_add)?;
                q = Self::conditionally_select(
                    &mut cs.ns(|| "self_is_min_case"),
                    &self_is_min,
                    &q_valid,
                    &q,
                )?;

                // set to zero if we know result is fractional
                q = Self::conditionally_select(
                    &mut cs.ns(|| "fraction"),
                    &other_is_min,
                    &zero,
                    &q,
                )?;

                // set to one if we know result is division of the minimum number by itself
                q = Self::conditionally_select(
                    &mut cs.ns(|| "one_result"),
                    &both_min,
                    &one,
                    &q,
                )?;

                Ok(Self::conditionally_select(
                    &mut cs.ns(|| "self_or_quotient"),
                    &self_is_zero,
                    self,
                    &q
                )?)
            }
        }
    )*)
}

div_int_impl!(Int8, Int16, Int32, Int64, Int128);
