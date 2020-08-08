use crate::{
    arithmetic::{Mul, Pow},
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
        utilities::{boolean::Boolean, select::CondSelectGadget},
    },
};

macro_rules! pow_int_impl {
    ($($gadget:ty),*) => ($(
        impl<F: PrimeField> Pow<F> for $gadget {
            type ErrorType = SignedIntegerError;

            fn pow<CS: ConstraintSystem<F>>(&self, mut cs: CS, other: &Self) -> Result<Self, Self::ErrorType> {
                // First set constant variables that we will reuse
                let one = Self::constant(1 as <$gadget as Int>::IntegerType);

                let mut result = one.clone();

                for (i, bit) in other.bits.iter().rev().enumerate() {
                    let found_one = Boolean::constant(result.eq(&one));
                    let cond1 = Boolean::and(cs.ns(|| format!("found_one_{}", i)), &bit.not(), &found_one)?;
                    let square = result.mul(cs.ns(|| format!("square_{}", i)), &result)?;

                    result = Self::conditionally_select(
                        &mut cs.ns(|| format!("result_or_square_{}", i)),
                        &cond1,
                        &result,
                        &square,
                    )?;

                    let self_or_one = Self::conditionally_select(
                        &mut cs.ns(|| format!("self_or_one_{}", i)),
                        &bit,
                        &self,
                        &one,
                    )?;

                    result = result
                        .mul(cs.ns(|| format!("multiply_by_self_or_one_{}", i)), &self_or_one)?;
                }

                Ok(result)
            }
        }
    )*)
}

pow_int_impl!(Int8, Int16, Int32, Int64, Int128);
