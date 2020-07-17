use crate::{
    bits::{ComparatorGadget, EvaluateLtGadget},
    Int128,
    Int16,
    Int32,
    Int64,
    Int8,
};

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::PrimeField,
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{boolean::Boolean, select::CondSelectGadget},
    },
};
use std::cmp::Ordering;

macro_rules! cmp_gadget_impl {
    ($($gadget: ident)*) => ($(
        /* Bitwise less than comparison of two signed integers */
        impl<F: PrimeField> EvaluateLtGadget<F> for $gadget {
            fn less_than<CS: ConstraintSystem<F>>(
                &self,
                mut cs: CS,
                other: &Self
            ) -> Result<Boolean, SynthesisError> {

                let mut result = Boolean::constant(true);
                let mut all_equal = Boolean::constant(true);

                // msb -> lsb
                for (i, (a, b)) in self
                    .bits
                    .iter()
                    .rev()
                    .zip(other.bits.iter().rev())
                    .enumerate()
                {

                    // check msb signed bit
                    let less = if i == 0 {
                        // a == 1 & b == 0
                        Boolean::and(cs.ns(|| format!("a and not b [{}]", i)), a, &b.not())?
                    } else {
                        // a == 0 & b == 1
                        Boolean::and(cs.ns(|| format!("not a and b [{}]", i)), &a.not(), b)?
                    };

                    // a == b = !(a ^ b)
                    let not_equal = Boolean::xor(cs.ns(|| format!("a XOR b [{}]", i)), a, b)?;
                    let equal = not_equal.not();

                    // evaluate a <= b
                    let less_or_equal = Boolean::or(cs.ns(|| format!("less or equal [{}]", i)), &less, &equal)?;

                    // select the current result if it is the first bit difference
                    result = Boolean::conditionally_select(cs.ns(|| format!("select bit [{}]", i)), &all_equal, &less_or_equal, &result)?;

                    // keep track of equal bits
                    all_equal = Boolean::and(cs.ns(|| format!("accumulate equal [{}]", i)), &all_equal, &equal)?;
                }

                result = Boolean::and(cs.ns(|| format!("false if all equal")), &result, &all_equal.not())?;

                Ok(result)
            }
        }

        /* Bitwise comparison of two unsigned integers */
        impl<F: PrimeField> ComparatorGadget<F> for $gadget {}

        impl PartialOrd for $gadget {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Option::from(self.value.cmp(&other.value))
            }
        }
    )*)
}

cmp_gadget_impl!(Int8 Int16 Int32 Int64 Int128);
