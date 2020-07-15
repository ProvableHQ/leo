use crate::{
    binary::{ComparatorGadget, EvaluateLtGadget},
    Int128,
    Int16,
    Int32,
    Int64,
    Int8,
};

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::PrimeField,
    gadgets::{r1cs::ConstraintSystem, utilities::boolean::Boolean},
};

macro_rules! cmp_gadget_impl {
    ($($gadget: ident)*) => ($(
        impl<F: PrimeField> EvaluateLtGadget<F> for $gadget {
            fn less_than<CS: ConstraintSystem<F>>(
                &self,
                mut cs: CS,
                other: &Self
            ) -> Result<Boolean, SynthesisError> {

                for (i, (a, b)) in self.bits
                    .iter()
                    .rev()
                    .zip(other.bits.iter().rev())
                    .enumerate()
                {
                    let is_greater = if i == 0 {
                        // Check sign bit
                        // is_greater = !a_msb & b_msb
                        // only true when a > b
                        Boolean::and(cs.ns(|| format!("not a and b [{}]", i)), &a.not(), b)?
                    } else {
                        // is_greater = a & !b
                        // only true when a > b
                        Boolean::and(cs.ns(|| format!("a and not b [{}]", i)), a, &b.not())?
                    };

                    let is_less = if i == 0 {
                        // Check sign bit
                        // is_less = a_msb & ! b_msb
                        // only true when a < b
                        Boolean::and(cs.ns(|| format!("a and not b [{}]", i)), a, &b.not())?
                    } else {
                        // is_less = !a & b
                        // only true when a < b
                        Boolean::and(cs.ns(|| format!("not a and b [{}]", i)), &a.not(), b)?
                    };

                    if is_greater.get_value().unwrap() {
                        return Ok(is_greater.not());
                    } else if is_less.get_value().unwrap() {
                        return Ok(is_less);
                    } else if i == self.bits.len() - 1 {
                        return Ok(is_less);
                    }
                }

                Err(SynthesisError::Unsatisfiable)
            }
        }

        impl<F: PrimeField> ComparatorGadget<F> for $gadget {}
    )*)
}

cmp_gadget_impl!(Int8 Int16 Int32 Int64 Int128);
