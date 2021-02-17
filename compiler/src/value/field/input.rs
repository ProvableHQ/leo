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

//! Methods to enforce constraints on input field values in a compiled Leo program.

use crate::{errors::FieldError, value::ConstrainedValue, FieldType, GroupType};
use leo_ast::{InputValue, Span};

use snarkvm_errors::gadgets::SynthesisError;
use snarkvm_models::{
    curves::{Field, PrimeField},
    gadgets::{r1cs::ConstraintSystem, utilities::alloc::AllocGadget},
};

pub(crate) fn allocate_field<F: Field + PrimeField, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    name: &str,
    option: Option<String>,
    span: &Span,
) -> Result<FieldType<F>, FieldError> {
    FieldType::alloc(
        cs.ns(|| format!("`{}: field` {}:{}", name, span.line, span.start)),
        || option.ok_or(SynthesisError::AssignmentMissing),
    )
    .map_err(|_| FieldError::missing_field(format!("{}: field", name), span.to_owned()))
}

pub(crate) fn field_from_input<'a, F: Field + PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    name: &str,
    input_value: Option<InputValue>,
    span: &Span,
) -> Result<ConstrainedValue<'a, F, G>, FieldError> {
    // Check that the parameter value is the correct type
    let option = match input_value {
        Some(input) => {
            if let InputValue::Field(string) = input {
                Some(string)
            } else {
                return Err(FieldError::invalid_field(input.to_string(), span.to_owned()));
            }
        }
        None => None,
    };

    let field = allocate_field(cs, name, option, span)?;

    Ok(ConstrainedValue::Field(field))
}
