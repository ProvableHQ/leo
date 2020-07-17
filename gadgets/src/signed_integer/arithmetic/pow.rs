use crate::{arithmetic::*, errors::SignedIntegerError, Int, Int128, Int16, Int32, Int64, Int8};

use snarkos_models::{
    curves::PrimeField,
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{alloc::AllocGadget, boolean::Boolean, select::CondSelectGadget},
    },
};

/// Exponentiation for a signed integer gadget
pub trait Pow<Rhs = Self>
where
    Self: std::marker::Sized,
{
    #[must_use]
    fn pow<F: PrimeField, CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Self, SignedIntegerError>;
}

macro_rules! pow_int_impl {
    ($($gadget:ty)*) => ($(
        impl Pow for $gadget {
            fn pow<F: PrimeField, CS: ConstraintSystem<F>>(&self, mut cs: CS, other: &Self) -> Result<Self, SignedIntegerError> {
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
                let one_const = Self::constant(1 as <$gadget as Int>::IntegerType);
                let one_alloc = Self::alloc(&mut cs.ns(|| "allocated_1"), || Ok(1 as <$gadget as Int>::IntegerType))?;
                let mut result = Self::conditionally_select(
                    &mut cs.ns(|| "constant_or_allocated"),
                    &is_constant,
                    &one_const,
                    &one_alloc,
                )?;

                for (i, bit) in other.bits.iter().rev().enumerate() {
                    let found_one = Boolean::constant(result.eq(&one_const));
                    let cond1 = Boolean::and(cs.ns(|| format!("found_one_{}", i)), &bit.not(), &found_one)?;
                    let square = result.mul(cs.ns(|| format!("square_{}", i)), &result).unwrap();

                    result = Self::conditionally_select(
                        &mut cs.ns(|| format!("result_or_square_{}", i)),
                        &cond1,
                        &result,
                        &square,
                    )?;

                    let mul_by_self = result
                        .mul(cs.ns(|| format!("multiply_by_self_{}", i)), &self);

                    result = Self::conditionally_select(
                        &mut cs.ns(|| format!("mul_by_self_or_result_{}", i)),
                        &bit,
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
