use crate::signed_integer::*;

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::PrimeField,
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{boolean::Boolean, eq::ConditionalEqGadget},
    },
};

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
