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

use crate::{
    errors::CharError,
    value::{field::input::allocate_field, ConstrainedValue},
    FieldType,
    GroupType,
};

use leo_ast::{InputValue, Span};
use snarkvm_fields::PrimeField;
use snarkvm_gadgets::{
    fields::FpGadget,
    utilities::{
        boolean::Boolean,
        eq::{ConditionalEqGadget, EqGadget, EvaluateEqGadget},
        select::CondSelectGadget,
    },
};
use snarkvm_r1cs::{ConstraintSystem, SynthesisError};

/// A char
#[derive(Clone, Debug)]
pub struct Char<F: PrimeField> {
    pub character: char,
    pub field: FieldType<F>,
}

impl<F: PrimeField> Char<F> {
    pub fn constant(character: char, field: String, span: &Span) -> Result<Self, CharError> {
        Ok(Self {
            character,
            field: FieldType::constant(field, span)?,
        })
    }
}

impl<F: PrimeField> PartialEq for Char<F> {
    fn eq(&self, other: &Self) -> bool {
        self.field.eq(&other.field)
    }
}

impl<F: PrimeField> Eq for Char<F> {}

impl<F: PrimeField> PartialOrd for Char<F> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.field.partial_cmp(&other.field)
    }
}

impl<F: PrimeField> EvaluateEqGadget<F> for Char<F> {
    fn evaluate_equal<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Boolean, SynthesisError> {
        self.field.evaluate_equal(cs, &other.field)
    }
}

impl<F: PrimeField> EqGadget<F> for Char<F> {}

impl<F: PrimeField> ConditionalEqGadget<F> for Char<F> {
    fn conditional_enforce_equal<CS: ConstraintSystem<F>>(
        &self,
        cs: CS,
        other: &Self,
        condition: &Boolean,
    ) -> Result<(), SynthesisError> {
        self.field.conditional_enforce_equal(cs, &other.field, condition)
    }

    fn cost() -> usize {
        2 * <FpGadget<F> as CondSelectGadget<F>>::cost()
    }
}

pub(crate) fn char_from_input<'a, F: PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    name: &str,
    input_value: Option<InputValue>,
    span: &Span,
) -> Result<ConstrainedValue<'a, F, G>, CharError> {
    // Check that the parameter value is the correct type
    let option = match input_value {
        Some(input) => {
            if let InputValue::Char(character) = input {
                (character, Some((character as u32).to_string()))
            } else {
                return Err(CharError::invalid_char(input.to_string(), span));
            }
        }
        None => (' ', None),
    };

    let field = allocate_field(cs, name, option.1, span)?;

    Ok(ConstrainedValue::Char(Char {
        character: option.0,
        field,
    }))
}

impl<F: PrimeField + std::fmt::Display> std::fmt::Display for Char<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.character)
    }
}
