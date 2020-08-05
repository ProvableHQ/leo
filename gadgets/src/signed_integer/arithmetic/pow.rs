use crate::{
    arithmetic::{Div, Mul, Pow},
    bits::comparator::{ComparatorGadget, EvaluateLtGadget},
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
        utilities::{alloc::AllocGadget, boolean::Boolean, eq::EvaluateEqGadget, select::CondSelectGadget},
    },
};

macro_rules! pow_int_impl {
    ($($gadget:ty)*) => ($(
        impl<F: PrimeField> Pow<F> for $gadget {
            type ErrorType = SignedIntegerError;

            fn pow<CS: ConstraintSystem<F>>(&self, mut cs: CS, other: &Self) -> Result<Self, Self::ErrorType> {
                // First allocate some variables that we will reuse

                let bool_true = Boolean::constant(true);
                let bool_false = Boolean::constant(false);

                let is_constant = Boolean::constant(Self::result_is_constant(&self, &other));

                let max_const = Self::constant(<$gadget as Int>::IntegerType::MAX);
                let max_alloc = Self::alloc(&mut cs.ns(|| "allocated_max"), || Ok(<$gadget as Int>::IntegerType::MAX))?;
                let max = Self::conditionally_select(
                    &mut cs.ns(|| "constant_or_allocated_max"),
                    &is_constant,
                    &max_const,
                    &max_alloc,
                )?;

                let min_const = Self::constant(<$gadget as Int>::IntegerType::MIN);
                let min_alloc = Self::alloc(&mut cs.ns(|| "allocated_min"), || Ok(<$gadget as Int>::IntegerType::MIN))?;
                let min = Self::conditionally_select(
                    &mut cs.ns(|| "constant_or_allocated_min"),
                    &is_constant,
                    &min_const,
                    &min_alloc,
                )?;

                let one_const = Self::constant(1 as <$gadget as Int>::IntegerType);
                let one_alloc = Self::alloc(&mut cs.ns(|| "allocated_one"), || Ok(1 as <$gadget as Int>::IntegerType))?;
                let one = Self::conditionally_select(
                    &mut cs.ns(|| "constant_or_allocated_one"),
                    &is_constant,
                    &one_const,
                    &one_alloc,
                )?;

                let zero_const = Self::constant(0 as <$gadget as Int>::IntegerType);
                let zero_alloc = Self::alloc(&mut cs.ns(|| "allocated_zero"), || Ok(0 as <$gadget as Int>::IntegerType))?;
                let zero = Self::conditionally_select(
                    &mut cs.ns(|| "constant_or_allocated_zero"),
                    &is_constant,
                    &zero_const,
                    &zero_alloc,
                )?;

                let sign_bit = self.bits.last().unwrap();

                // if self is zero, remember to return zero or one
                let self_is_zero = self.evaluate_equal(
                    &mut cs.ns(|| "self_is_zero_check"),
                    &zero
                )?;

                // if other is 0, return one
                let other_is_zero = other.evaluate_equal(
                    &mut cs.ns(|| "other_is_zero_check"),
                    &zero
                )?;

                // if self is zero, set self_value to a dummy value so computation does not error
                let self_value = Self::conditionally_select(
                    &mut cs.ns(|| "set_self_value"),
                    &self_is_zero,
                    &one,
                    &self,
                )?;

                // Check that we can multiply the next result without overflow or underflow
                // We want to calculate result * self
                // We overflow when result > MAX / self
                // We underflow when result < MIN / self
                let max_div_self = max.div(
                    &mut cs.ns(|| "max_div_self"),
                     &self_value
                 )?;
                let min_div_self = min.div(
                    &mut cs.ns(|| "min_div_self"),
                    &self_value
                )?;

                // account for sign bit if self is negative
                // overflow when result < MAX / self
                // underflow when result > MIN / self
                let gt_check = Self::conditionally_select(
                    &mut cs.ns(|| "select_result_for_greater_than_check"),
                    &sign_bit,
                    &min_div_self,
                    &max_div_self
                )?;
                let lt_check = Self::conditionally_select(
                    &mut cs.ns(|| "select_result_for_less_than_check"),
                    &sign_bit,
                    &max_div_self,
                    &min_div_self
                )?;

                let mut result = one.clone();

                for (i, bit) in other.bits.iter().rev().enumerate() {
                    let found_one = Boolean::constant(result.eq(&one_const));
                    let cond1 = Boolean::and(cs.ns(|| format!("found_one_{}", i)), &bit.not(), &found_one)?;
                    let square = result.mul(cs.ns(|| format!("square_{}", i)), &result)?;

                    result = Self::conditionally_select(
                        &mut cs.ns(|| format!("result_or_square_{}", i)),
                        &cond1,
                        &result,
                        &square,
                    )?;

                    let gt_fail = result.greater_than(
                        &mut cs.ns(|| format!("greater_than_check_fail_{}", i)),
                        &gt_check,
                    )?;
                    let lt_fail = result.less_than(
                        &mut cs.ns(|| format!("less_than_check_fail_{}", i)),
                        &lt_check,
                    )?;

                    // if we will overflow or underflow, select a dummy result
                    // this dummy result should not be selected by the bit
                    // if it is selected, then throw an integer overflow or underflow error
                    let select_dummy = Boolean::or(
                        &mut cs.ns(|| format!("select_dummy_{}", i)),
                        &lt_fail,
                        &gt_fail
                    )?;
                    let self_selected = Self::conditionally_select(
                        &mut cs.ns(|| format!("select_self_no_overflow_{}", i)),
                        &select_dummy,
                        &one,
                        &self_value
                    )?;

                    let mul_by_self = result
                        .mul(cs.ns(|| format!("multiply_by_self_{}", i)), &self_selected)?;

                    // if an overflowing or underflowing result is selected by the current bit, then throw an integer error
                    if bit.eq(&bool_true) {
                        if gt_fail.eq(&bool_true) && sign_bit.eq(&bool_false)
                            || lt_fail.eq(&bool_true) && sign_bit.eq(&bool_true) {
                            return Err(SignedIntegerError::Overflow)
                        } else if gt_fail.eq(&bool_true) && sign_bit.eq(&bool_true)
                            || lt_fail.eq(&bool_true) && sign_bit.eq(&bool_false)  {
                            return Err(SignedIntegerError::Underflow)
                        }
                    }

                    result = Self::conditionally_select(
                        &mut cs.ns(|| format!("mul_by_self_or_result_{}", i)),
                        &bit,
                        &mul_by_self,
                        &result,
                    )?;
                }

                // return zero or one if self is zero
                result = Self::conditionally_select(
                    &mut cs.ns(|| "return_zero"),
                    &self_is_zero,
                    &zero,
                    &result,
                )?;

                // return one if other is zero
                result = Self::conditionally_select(
                    &mut cs.ns(|| "return_one"),
                    &other_is_zero,
                    &one,
                    &result,
                )?;

                Ok(result)
            }
        }
    )*)
}

pow_int_impl!(Int8 Int16 Int32 Int64 Int128);
