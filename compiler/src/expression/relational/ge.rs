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

//! Enforces a relational `>=` operator in a resolved Leo program.

use crate::{errors::ExpressionError, value::ConstrainedValue, GroupType};
use leo_gadgets::bits::ComparatorGadget;
use leo_typed::Span;

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

pub fn evaluate_ge<F: Field + PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    left: ConstrainedValue<F, G>,
    right: ConstrainedValue<F, G>,
    span: Span,
) -> Result<ConstrainedValue<F, G>, ExpressionError> {
    let mut unique_namespace = cs.ns(|| format!("evaluate {} >= {} {}:{}", left, right, span.line, span.start));
    let constraint_result = match (left, right) {
        (ConstrainedValue::Integer(num_1), ConstrainedValue::Integer(num_2)) => {
            num_1.greater_than_or_equal(unique_namespace, &num_2)
        }
        (ConstrainedValue::Unresolved(string), val_2) => {
            let val_1 = ConstrainedValue::from_other(string, &val_2, span.clone())?;
            return evaluate_ge(&mut unique_namespace, val_1, val_2, span);
        }
        (val_1, ConstrainedValue::Unresolved(string)) => {
            let val_2 = ConstrainedValue::from_other(string, &val_1, span.clone())?;
            return evaluate_ge(&mut unique_namespace, val_1, val_2, span);
        }
        (val_1, val_2) => {
            return Err(ExpressionError::incompatible_types(
                format!("{} >= {}", val_1, val_2),
                span,
            ));
        }
    };

    let boolean = constraint_result.map_err(|_| ExpressionError::cannot_evaluate(format!(">="), span))?;

    Ok(ConstrainedValue::Boolean(boolean))
}
