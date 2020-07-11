use crate::signed_integer::*;

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::PrimeField,
    gadgets::{
        r1cs::{Assignment, ConstraintSystem},
        utilities::{alloc::AllocGadget, boolean::Boolean, eq::EqGadget, select::CondSelectGadget},
    },
};

macro_rules! select_int_impl {
    ($($gadget: ident)*) => ($(
        impl<F: PrimeField> CondSelectGadget<F> for $gadget {
            fn conditionally_select<CS: ConstraintSystem<F>> (
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
                    let result_val = cond.get_value().and_then(|c| {
                        if c {
                            first.value
                        } else {
                            second.value
                        }
                    });

                    let result = Self::alloc(cs.ns(|| "cond_select_result"), || result_val.get().map(|v| v))?;

                    let expected_bits = first
                        .bits
                        .iter()
                        .zip(&second.bits)
                        .enumerate()
                        .map(|(i, (a, b))| {
                            Boolean::conditionally_select(
                                &mut cs.ns(|| format!("{}_cond_select_{}", <$gadget as Int>::SIZE, i)),
                                cond,
                                a,
                                b,
                            ).unwrap()
                        })
                        .collect::<Vec<Boolean>>();

                    for (i, (actual, expected)) in result.bits.iter().zip(expected_bits.iter()).enumerate() {
                        actual.enforce_equal(&mut cs.ns(|| format!("selected_result_bit_{}", i)), expected)?;
                    }

                    Ok(result)
                }
            }

            fn cost() -> usize {
                unimplemented!();
            }
        }
    )*)
}

select_int_impl!(Int8 Int16 Int32 Int64 Int128);
