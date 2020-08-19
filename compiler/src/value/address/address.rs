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

use crate::{errors::AddressError, ConstrainedValue, GroupType};
use leo_typed::{InputValue, Span};

use snarkos_dpc::base_dpc::instantiated::Components;
use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{
            boolean::Boolean,
            eq::{ConditionalEqGadget, EvaluateEqGadget},
            select::CondSelectGadget,
            uint::{UInt, UInt8},
        },
    },
};
use snarkos_objects::account::AccountAddress;
use snarkos_utilities::ToBytes;
use std::str::FromStr;

/// A public address
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Address {
    pub address: Option<AccountAddress<Components>>,
    pub bytes: Vec<UInt8>,
}

impl Address {
    pub(crate) fn constant(address: String, span: Span) -> Result<Self, AddressError> {
        let address = AccountAddress::from_str(&address).map_err(|error| AddressError::account_error(error, span))?;

        let mut address_bytes = vec![];
        address.write(&mut address_bytes);

        let bytes = UInt8::constant_vec(&address_bytes[..]);

        Ok(Address {
            address: Some(address),
            bytes,
        })
    }

    pub(crate) fn is_constant(&self) -> bool {
        let mut result = true;

        for byte in self.bytes {
            result = result && byte.is_constant()
        }

        result
    }

    pub(crate) fn from_input<F: Field + PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
        _cs: &mut CS,
        name: String,
        input_value: Option<InputValue>,
        span: Span,
    ) -> Result<ConstrainedValue<F, G>, AddressError> {
        // Check that the input value is the correct type
        let address_value = match input_value {
            Some(input) => {
                if let InputValue::Address(string) = input {
                    let address = Address::constant(string, span)?;

                    address
                } else {
                    return Err(AddressError::invalid_address(name, span));
                }
            }
            None => unimplemented!(),
        };

        Ok(ConstrainedValue::Address(address_value))
    }
}

impl<F: Field + PrimeField> EvaluateEqGadget<F> for Address {
    fn evaluate_equal<CS: ConstraintSystem<F>>(&self, _cs: CS, _other: &Self) -> Result<Boolean, SynthesisError> {
        unimplemented!()
    }
}

fn cond_equal_helper(first: &Address, second: &Address, cond: bool) -> Result<(), SynthesisError> {
    unimplemented!()
    // if cond && first.0.is_some() && second.0.is_some() {
    //     if first.eq(second) {
    //         Ok(())
    //     } else {
    //         Err(SynthesisError::Unsatisfiable)
    //     }
    // } else {
    //     Ok(())
    // }
}

impl<F: Field + PrimeField> ConditionalEqGadget<F> for Address {
    fn conditional_enforce_equal<CS: ConstraintSystem<F>>(
        &self,
        _cs: CS,
        other: &Self,
        condition: &Boolean,
    ) -> Result<(), SynthesisError> {
        if let Boolean::Constant(cond) = *condition {
            cond_equal_helper(self, other, cond)
        } else {
            unimplemented!()
        }
    }

    fn cost() -> usize {
        unimplemented!()
    }
}

fn cond_select_helper(first: &Address, second: &Address, cond: bool) -> Address {
    if cond { first.clone() } else { second.clone() }
}

impl<F: Field + PrimeField> CondSelectGadget<F> for Address {
    fn conditionally_select<CS: ConstraintSystem<F>>(
        _cs: CS,
        cond: &Boolean,
        first: &Self,
        second: &Self,
    ) -> Result<Self, SynthesisError> {
        if let Boolean::Constant(cond) = *cond {
            Ok(cond_select_helper(first, second, cond))
        } else {
            unimplemented!()
        }
    }

    fn cost() -> usize {
        unimplemented!()
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        unimplemented!()
        // write!(
        //     f,
        //     "{}",
        //     self.0.as_ref().map(|v| v.to_string()).unwrap_or(format!("[allocated]"))
        // )
    }
}
