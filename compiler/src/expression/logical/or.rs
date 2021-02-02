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

//! Enforces a logical `||` operator in a resolved Leo program.

use crate::{errors::BooleanError, value::ConstrainedValue, GroupType};
use leo_ast::Span;

use snarkvm_models::{
    curves::{Field, PrimeField},
    gadgets::{r1cs::ConstraintSystem, utilities::boolean::Boolean},
};

pub fn enforce_or<F: Field + PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    left: ConstrainedValue<F, G>,
    right: ConstrainedValue<F, G>,
    span: &Span,
) -> Result<ConstrainedValue<F, G>, BooleanError> {
    let name = format!("{} || {}", left, right);

    if let (ConstrainedValue::Boolean(left_bool), ConstrainedValue::Boolean(right_bool)) = (left, right) {
        let result = Boolean::or(
            cs.ns(|| format!("{} {}:{}", name, span.line, span.start)),
            &left_bool,
            &right_bool,
        )
        .map_err(|e| BooleanError::cannot_enforce("||".to_string(), e, span.to_owned()))?;

        return Ok(ConstrainedValue::Boolean(result));
    }

    Err(BooleanError::cannot_evaluate(name, span.to_owned()))
}
