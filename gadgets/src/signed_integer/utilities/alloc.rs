// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::Int;
use crate::Int128;
use crate::Int16;
use crate::Int32;
use crate::Int64;
use crate::Int8;

use core::borrow::Borrow;
use core::iter;
use snarkvm_fields::Field;
use snarkvm_gadgets::traits::utilities::alloc::AllocGadget;
use snarkvm_gadgets::traits::utilities::boolean::AllocatedBit;
use snarkvm_gadgets::traits::utilities::boolean::Boolean;
use snarkvm_r1cs::ConstraintSystem;
use snarkvm_r1cs::SynthesisError;

fn create_value<T: Borrow<bool>, I: IntoIterator<Item = Option<T>>, F: Field, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    iter: I,
) -> Result<Vec<Boolean>, SynthesisError> {
    iter.into_iter()
        .enumerate()
        .map(|(i, v)| {
            Ok(Boolean::from(AllocatedBit::alloc(
                &mut cs.ns(|| format!("allocated bit_gadget {}", i)),
                || v.ok_or(SynthesisError::AssignmentMissing),
            )?))
        })
        .collect()
}

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

                let bits = match value {
                    Ok(mut val) => {
                        let mut v = Vec::with_capacity(<$gadget as Int>::SIZE);
                        for _ in 0..<$gadget as Int>::SIZE {
                            v.push(Some(val & 1 == 1));
                            val >>= 1;
                        }
                        create_value(&mut cs, v)
                    }
                    Err(_) => {
                        let i = iter::repeat(None::<bool>).take(<$gadget as Int>::SIZE);
                        create_value(&mut cs, i)
                    },
                }?;

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

                let bits = match value {
                    Ok(mut val) => {
                        let mut v = Vec::with_capacity(<$gadget as Int>::SIZE);
                        for _ in 0..<$gadget as Int>::SIZE {
                            v.push(Some(val & 1 == 1));
                            val >>= 1;
                        }
                        create_value(&mut cs, v)
                    }
                    Err(_) => {
                        let i = iter::repeat(None::<bool>).take(<$gadget as Int>::SIZE);
                        create_value(&mut cs, i)
                    },
                }?;

                Ok(Self {
                    bits,
                    value: value.ok(),
                })
            }
        }
    )*)
}

alloc_int_impl!(Int8 Int16 Int32 Int64 Int128);
