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

//! Methods to enforce constraints on input boolean values in a resolved Leo program.

use crate::errors::BooleanError;
use crate::value::ConstrainedValue;
use crate::GroupType;
use leo_ast::InputValue;
use leo_ast::Span;

use snarkvm_errors::gadgets::SynthesisError;
use snarkvm_models::curves::PrimeField;
use snarkvm_models::gadgets::r1cs::ConstraintSystem;
use snarkvm_models::gadgets::utilities::alloc::AllocGadget;
use snarkvm_models::gadgets::utilities::boolean::Boolean;

pub(crate) fn allocate_bool<F: PrimeField, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    name: &str,
    option: Option<bool>,
    span: &Span,
) -> Result<Boolean, BooleanError> {
    Boolean::alloc(
        cs.ns(|| format!("`{}: bool` {}:{}", name, span.line, span.start)),
        || option.ok_or(SynthesisError::AssignmentMissing),
    )
    .map_err(|_| BooleanError::missing_boolean(format!("{}: bool", name), span.to_owned()))
}

pub(crate) fn bool_from_input<'a, F: PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    name: &str,
    input_value: Option<InputValue>,
    span: &Span,
) -> Result<ConstrainedValue<'a, F, G>, BooleanError> {
    // Check that the input value is the correct type
    let option = match input_value {
        Some(input) => {
            if let InputValue::Boolean(bool) = input {
                Some(bool)
            } else {
                return Err(BooleanError::invalid_boolean(name.to_owned(), span.to_owned()));
            }
        }
        None => None,
    };

    let number = allocate_bool(cs, name, option, span)?;

    Ok(ConstrainedValue::Boolean(number))
}
