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

use crate::{value::ConstrainedValue, GroupType};
use leo_ast::InputValue;
use leo_errors::{CompilerError, LeoError, Span};

use snarkvm_fields::PrimeField;
use snarkvm_gadgets::{boolean::Boolean, traits::alloc::AllocGadget};
use snarkvm_r1cs::{ConstraintSystem, SynthesisError};

pub(crate) fn allocate_bool<F: PrimeField, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    name: &str,
    option: Option<bool>,
    span: &Span,
) -> Result<Boolean, LeoError> {
    Ok(Boolean::alloc(
        cs.ns(|| format!("`{}: bool` {}:{}", name, span.line_start, span.col_start)),
        || option.ok_or(SynthesisError::AssignmentMissing),
    )
    .map_err(|_| CompilerError::boolean_value_missing_boolean(format!("{}: bool", name), span))?)
}

pub(crate) fn bool_from_input<'a, F: PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    name: &str,
    input_value: Option<InputValue>,
    span: &Span,
) -> Result<ConstrainedValue<'a, F, G>, LeoError> {
    // Check that the input value is the correct type
    let option = match input_value {
        Some(input) => {
            if let InputValue::Boolean(bool) = input {
                Some(bool)
            } else {
                return Err(CompilerError::boolean_value_invalid_boolean(name, span))?;
            }
        }
        None => None,
    };

    let number = allocate_bool(cs, name, option, span)?;

    Ok(ConstrainedValue::Boolean(number))
}
