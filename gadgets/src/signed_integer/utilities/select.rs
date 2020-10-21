// Copyright (C) 2019-2020 Aleo Systems Inc.
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

                    let result = Self::alloc(cs.ns(|| "cond_select_result"), || result_val.get())?;

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
