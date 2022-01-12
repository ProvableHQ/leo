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

use crate::{ConstrainedValue, GroupType, IntegerTrait};
use leo_ast::InputValue;
use leo_errors::{CompilerError, Result, Span};

use snarkvm_dpc::{account::Address as AleoAddress, network::testnet2::Testnet2};
use snarkvm_fields::PrimeField;
use snarkvm_gadgets::{
    boolean::Boolean,
    integers::uint::UInt8,
    traits::{
        alloc::AllocGadget,
        eq::{ConditionalEqGadget, EqGadget, EvaluateEqGadget},
        select::CondSelectGadget,
    },
};
use snarkvm_r1cs::{Assignment, ConstraintSystem, SynthesisError};
use snarkvm_utilities::ToBytes;
use std::{borrow::Borrow, str::FromStr};

/// A public address
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Address {
    pub address: Option<AleoAddress<Testnet2>>,
    pub bytes: Vec<UInt8>,
}

impl Address {
    pub(crate) fn constant(address: String, span: &Span) -> Result<Self> {
        let address = AleoAddress::<Testnet2>::from_str(&address)
            .map_err(|e| CompilerError::address_value_account_error(e, span))?;

        let address_bytes = address.to_bytes_le().unwrap();

        let bytes = UInt8::constant_vec(&address_bytes[..]);

        Ok(Address {
            address: Some(address),
            bytes,
        })
    }

    pub(crate) fn is_constant(&self) -> bool {
        self.bytes.iter().all(|byte| byte.is_constant())
    }

    pub(crate) fn from_input<'a, F: PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
        cs: &mut CS,
        name: &str,
        input_value: Option<InputValue>,
        span: &Span,
    ) -> Result<ConstrainedValue<'a, F, G>> {
        // Check that the input value is the correct type
        let address_value = match input_value {
            Some(input) => {
                if let InputValue::Address(string) = input {
                    Some(string)
                } else {
                    return Err(CompilerError::address_value_invalid_address(name, span).into());
                }
            }
            None => None,
        };

        let address = Address::alloc(
            cs.ns(|| format!("`{}: address` {}:{}", name, span.line_start, span.col_start)),
            || address_value.ok_or(SynthesisError::AssignmentMissing),
        )
        .map_err(|_| CompilerError::address_value_missing_address(span))?;

        Ok(ConstrainedValue::Address(address))
    }

    pub(crate) fn alloc_helper<
        F: PrimeField,
        CS: ConstraintSystem<F>,
        Fn: FnOnce() -> Result<T, SynthesisError>,
        T: Borrow<String>,
    >(
        cs: CS,
        value_gen: Fn,
    ) -> Result<AleoAddress<Testnet2>, SynthesisError> {
        if cs.is_in_setup_mode() {
            Ok(AleoAddress::<Testnet2>::default())
        } else {
            let address_string = value_gen()?.borrow().clone();
            AleoAddress::from_str(&address_string).map_err(|_| SynthesisError::AssignmentMissing)
        }
    }
}

impl<F: PrimeField> AllocGadget<String, F> for Address {
    fn alloc_constant<Fn: FnOnce() -> Result<T, SynthesisError>, T: Borrow<String>, CS: ConstraintSystem<F>>(
        _cs: CS,
        value_gen: Fn,
    ) -> Result<Self, SynthesisError> {
        let address: AleoAddress<Testnet2> = {
            let address_string = value_gen()?.borrow().clone();
            AleoAddress::from_str(&address_string).map_err(|_| SynthesisError::AssignmentMissing)?
        };
        let address_bytes = address.to_bytes_le().map_err(|_| SynthesisError::AssignmentMissing)?;

        let bytes = UInt8::constant_vec(&address_bytes[..]);

        Ok(Address {
            address: Some(address),
            bytes,
        })
    }

    fn alloc<Fn: FnOnce() -> Result<T, SynthesisError>, T: Borrow<String>, CS: ConstraintSystem<F>>(
        mut cs: CS,
        value_gen: Fn,
    ) -> Result<Self, SynthesisError> {
        let address: AleoAddress<Testnet2> = Self::alloc_helper(cs.ns(|| "allocate the address"), value_gen)?;
        let address_bytes = address.to_bytes_le().map_err(|_| SynthesisError::AssignmentMissing)?;

        let bytes = UInt8::alloc_vec(cs.ns(|| "allocate the address bytes"), &address_bytes[..])?;

        Ok(Address {
            address: Some(address),
            bytes,
        })
    }

    fn alloc_input<Fn: FnOnce() -> Result<T, SynthesisError>, T: Borrow<String>, CS: ConstraintSystem<F>>(
        mut cs: CS,
        value_gen: Fn,
    ) -> Result<Self, SynthesisError> {
        let address = Self::alloc_helper(cs.ns(|| "allocate the address"), value_gen)?;
        let address_bytes = address.to_bytes_le().map_err(|_| SynthesisError::AssignmentMissing)?;

        let bytes = UInt8::alloc_input_vec_le(cs.ns(|| "allocate the address bytes"), &address_bytes[..])?;

        Ok(Address {
            address: Some(address),
            bytes,
        })
    }
}

impl<F: PrimeField> EvaluateEqGadget<F> for Address {
    fn evaluate_equal<CS: ConstraintSystem<F>>(&self, mut cs: CS, other: &Self) -> Result<Boolean, SynthesisError> {
        if self.is_constant() && other.is_constant() {
            Ok(Boolean::Constant(self.eq(other)))
        } else {
            let mut result = Boolean::constant(true);

            for (i, (a, b)) in self.bytes.iter().zip(&other.bytes).enumerate() {
                let equal =
                    a.evaluate_equal(&mut cs.ns(|| format!("address evaluate equality for {}-th byte", i)), b)?;

                result = Boolean::and(
                    &mut cs.ns(|| format!("address and result for {}-th byte", i)),
                    &equal,
                    &result,
                )?;
            }

            Ok(result)
        }
    }
}

fn cond_equal_helper(first: &Address, second: &Address, cond: bool) -> Result<(), SynthesisError> {
    if cond && first.address.is_some() && second.address.is_some() {
        if first.eq(second) {
            Ok(())
        } else {
            Err(SynthesisError::Unsatisfiable)
        }
    } else {
        Ok(())
    }
}

impl<F: PrimeField> ConditionalEqGadget<F> for Address {
    fn conditional_enforce_equal<CS: ConstraintSystem<F>>(
        &self,
        mut cs: CS,
        other: &Self,
        condition: &Boolean,
    ) -> Result<(), SynthesisError> {
        if let Boolean::Constant(cond) = *condition {
            cond_equal_helper(self, other, cond)
        } else {
            for (i, (a, b)) in self.bytes.iter().zip(&other.bytes).enumerate() {
                a.conditional_enforce_equal(
                    &mut cs.ns(|| format!("address equality check for {}-th byte", i)),
                    b,
                    condition,
                )?;
            }
            Ok(())
        }
    }

    fn cost() -> usize {
        <UInt8 as ConditionalEqGadget<F>>::cost() * 32 // address 32 bytes
    }
}

fn cond_select_helper(first: &Address, second: &Address, cond: bool) -> Address {
    if cond {
        first.clone()
    } else {
        second.clone()
    }
}

impl<F: PrimeField> CondSelectGadget<F> for Address {
    fn conditionally_select<CS: ConstraintSystem<F>>(
        mut cs: CS,
        cond: &Boolean,
        first: &Self,
        second: &Self,
    ) -> Result<Self, SynthesisError> {
        if let Boolean::Constant(cond) = *cond {
            Ok(cond_select_helper(first, second, cond))
        } else {
            let result_val = cond.get_value().and_then(|c| {
                if c {
                    first.address.clone()
                } else {
                    second.address.clone()
                }
            });

            let result = Self::alloc(cs.ns(|| "cond_select_result"), || {
                result_val.get().map(|v| v.to_string())
            })?;

            let expected_bytes = first
                .bytes
                .iter()
                .zip(&second.bytes)
                .enumerate()
                .map(|(i, (a, b))| {
                    UInt8::conditionally_select(&mut cs.ns(|| format!("address_cond_select_{}", i)), cond, a, b)
                        .unwrap()
                })
                .collect::<Vec<UInt8>>();

            for (i, (actual, expected)) in result.bytes.iter().zip(expected_bytes.iter()).enumerate() {
                actual.enforce_equal(&mut cs.ns(|| format!("selected_result_byte_{}", i)), expected)?;
            }

            Ok(result)
        }
    }

    fn cost() -> usize {
        <UInt8 as CondSelectGadget<F>>::cost() * 32
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.address {
            Some(ref address) => write!(f, "{}", address),
            None => write!(f, "[input address]"),
        }
    }
}
