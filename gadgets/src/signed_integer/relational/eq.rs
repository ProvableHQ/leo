use crate::{Int, Int128, Int16, Int32, Int64, Int8};

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::PrimeField,
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{boolean::Boolean, eq::EvaluateEqGadget},
    },
};

macro_rules! eq_gadget_impl {
    ($($gadget: ident)*) => ($(
        impl<F: PrimeField> EvaluateEqGadget<F> for $gadget {
            fn evaluate_equal<CS: ConstraintSystem<F>>(
                &self,
                mut cs: CS,
                other: &Self
            ) -> Result<Boolean, SynthesisError> {
                let mut result = Boolean::constant(true);
                for (i, (a, b)) in self.bits.iter().zip(&other.bits).enumerate() {
                    let equal = a.evaluate_equal(
                        &mut cs.ns(|| format!("{} evaluate equality for {}-th bit", <$gadget as Int>::SIZE, i)),
                        b,
                    )?;

                    result = Boolean::and(
                        &mut cs.ns(|| format!("{} and result for {}-th bit", <$gadget as Int>::SIZE, i)),
                        &equal,
                        &result,
                    )?;
                }

                Ok(result)
            }
        }

        impl PartialEq for $gadget {
            fn eq(&self, other: &Self) -> bool {
                !self.value.is_none() && !other.value.is_none() && self.value == other.value
            }
        }

        impl Eq for $gadget {}
    )*)
}

eq_gadget_impl!(Int8 Int16 Int32 Int64 Int128);
