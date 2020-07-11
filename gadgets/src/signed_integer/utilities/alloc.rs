use crate::{Int, Int16, Int32, Int64, Int8};

use core::borrow::Borrow;
use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::Field,
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{
            alloc::AllocGadget,
            boolean::{AllocatedBit, Boolean},
        },
    },
};

macro_rules! alloc_int_impl {
    ($($gadget: ident)*) => ($(
        impl<F: Field> AllocGadget<<$gadget as Int>::IntegerType, F> for $gadget {
            fn alloc<
                Fn: FnOnce() -> Result<T, SynthesisError>,
                T: Borrow<<$gadget as Int>::IntegerType>,
                CS: ConstraintSystem<F>
            >(
                mut cs: CS,
                value_gen: Fn,
            ) -> Result<Self, SynthesisError> {
                let value = value_gen().map(|val| *val.borrow());
                let values = match value {
                    Ok(mut val) => {
                        let mut v = Vec::with_capacity(<$gadget as Int>::SIZE);

                        for _ in 0..<$gadget as Int>::SIZE {
                            v.push(Some(val & 1 == 1));
                            val >>= 1;
                        }

                        v
                    }
                    _ => vec![None; <$gadget as Int>::SIZE],
                };

                let bits = values
                    .into_iter()
                    .enumerate()
                    .map(|(i, v)| {
                        Ok(Boolean::from(AllocatedBit::alloc(
                            &mut cs.ns(|| format!("allocated bit_gadget {}", i)),
                            || v.ok_or(SynthesisError::AssignmentMissing),
                        )?))
                    })
                    .collect::<Result<Vec<_>, SynthesisError>>()?;

                Ok(Self {
                    bits,
                    value: value.ok(),
                })
            }

            fn alloc_input<
                Fn: FnOnce() -> Result<T, SynthesisError>,
                T: Borrow<<$gadget as Int>::IntegerType>,
                CS: ConstraintSystem<F>
            >(
                mut cs: CS,
                value_gen: Fn,
            ) -> Result<Self, SynthesisError> {
                let value = value_gen().map(|val| *val.borrow());
                let values = match value {
                    Ok(mut val) => {
                        let mut v = Vec::with_capacity(<$gadget as Int>::SIZE);

                        for _ in 0..<$gadget as Int>::SIZE {
                            v.push(Some(val & 1 == 1));
                            val >>= 1;
                        }

                        v
                    }
                    _ => vec![None; <$gadget as Int>::SIZE],
                };

                let bits = values
                    .into_iter()
                    .enumerate()
                    .map(|(i, v)| {
                        Ok(Boolean::from(AllocatedBit::alloc(
                            &mut cs.ns(|| format!("allocated bit_gadget {}", i)),
                            || v.ok_or(SynthesisError::AssignmentMissing),
                        )?))
                    })
                    .collect::<Result<Vec<_>, SynthesisError>>()?;

                Ok(Self {
                    bits,
                    value: value.ok(),
                })
            }
        }
    )*)
}

alloc_int_impl!(Int8 Int16 Int32 Int64);
