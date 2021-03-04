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

//! Enforces a relational `<=` operator in a resolved Leo program.

use crate::errors::ExpressionError;
use crate::value::ConstrainedValue;
use crate::GroupType;
use leo_asg::Span;
use leo_gadgets::bits::ComparatorGadget;

use snarkvm_fields::PrimeField;
use snarkvm_r1cs::ConstraintSystem;

pub fn evaluate_le<'a, F: PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    left: ConstrainedValue<'a, F, G>,
    right: ConstrainedValue<'a, F, G>,
    span: &Span,
) -> Result<ConstrainedValue<'a, F, G>, ExpressionError> {
    let unique_namespace = cs.ns(|| format!("evaluate {} <= {} {}:{}", left, right, span.line, span.start));
    let constraint_result = match (left, right) {
        (ConstrainedValue::Integer(num_1), ConstrainedValue::Integer(num_2)) => {
            num_1.less_than_or_equal(unique_namespace, &num_2)
        }
        (val_1, val_2) => {
            return Err(ExpressionError::incompatible_types(
                format!("{} <= {}", val_1, val_2),
                span.to_owned(),
            ));
        }
    };

    let boolean = constraint_result.map_err(|_| ExpressionError::cannot_evaluate("<=".to_string(), span.to_owned()))?;

    Ok(ConstrainedValue::Boolean(boolean))
}
