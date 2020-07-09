use crate::{errors::AddressError, ConstrainedValue, GroupType};
use leo_types::{InputValue, Span};

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
        },
    },
};
use snarkos_objects::account::AccountPublicKey;
use std::str::FromStr;

/// A public address
/// Addresses are currently constant values in the constraint system only
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Address(pub Option<AccountPublicKey<Components>>);

impl Address {
    pub(crate) fn new(address: String, span: Span) -> Result<Self, AddressError> {
        let address = AccountPublicKey::from_str(&address).map_err(|error| AddressError::account_error(error, span))?;

        Ok(Address(Some(address)))
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
                    let address = Address::new(string, span)?;

                    address
                } else {
                    return Err(AddressError::invalid_address(name, span));
                }
            }
            None => Address(None),
        };

        Ok(ConstrainedValue::Address(address_value))
    }
}

impl<F: Field + PrimeField> EvaluateEqGadget<F> for Address {
    fn evaluate_equal<CS: ConstraintSystem<F>>(&self, _cs: CS, other: &Self) -> Result<Boolean, SynthesisError> {
        Ok(Boolean::constant(self.eq(other)))
    }
}

fn cond_equal_helper(first: &Address, second: &Address, cond: bool) -> Result<(), SynthesisError> {
    if cond && first.0.is_some() && second.0.is_some() {
        if first.eq(second) {
            Ok(())
        } else {
            Err(SynthesisError::Unsatisfiable)
        }
    } else {
        Ok(())
    }
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
            condition
                .get_value()
                .map(|cond| cond_equal_helper(self, other, cond))
                .unwrap_or(Ok(()))
        }
    }

    fn cost() -> usize {
        0
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
            Ok(cond
                .get_value()
                .map(|cond| cond_select_helper(first, second, cond))
                .unwrap_or(first.clone()))
        }
    }

    fn cost() -> usize {
        0
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0.as_ref().map(|v| v.to_string()).unwrap_or(format!("[allocated]"))
        )
    }
}
