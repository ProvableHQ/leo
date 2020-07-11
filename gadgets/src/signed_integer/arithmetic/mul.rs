use crate::{binary::RippleCarryAdder, errors::IntegerError, sign_extend::SignExtend, Int, Int16, Int32, Int64, Int8};
use snarkos_models::{
    curves::PrimeField,
    gadgets::{r1cs::ConstraintSystem, utilities::boolean::Boolean},
};

/// Multiplication for a signed integer gadget
/// 1. Sign extend both integers to double precision.
/// 2. Compute double and add.
/// 3. Truncate to original bit size.
pub trait Mul<Rhs = Self>
where
    Self: std::marker::Sized,
{
    #[must_use]
    fn mul<F: PrimeField, CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<(), IntegerError>;
}

macro_rules! mul_int_impl {
    ($($gadget: ident)*) => ($(
        impl Mul for $gadget {
            fn mul<F: PrimeField, CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<(), IntegerError> {
                // let is_constant = Boolean::constant(Self::result_is_constant(&self, &other));
                // let constant_result = Self::constant(0 as <$gadget as Int>::)
                //
                // let double = <$gadget as Int>::SIZE * 2;
                //
                // let a = Boolean::sign_extend(&self.bits, double);
                // let b = Boolean::sign_extend(&other.bits, double);
                //
                // let result =
                //
                // for bit in b.iter() {
                //
                // }

                Ok(())
            }
        }
    )*)
}

// mul_int_impl!(Int8 Int16 Int32 Int64);
mul_int_impl!(Int8);
