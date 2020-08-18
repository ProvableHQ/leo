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

//! Enforces a relational `==` operator in a resolved Leo program.

use crate::{errors::ExpressionError, value::ConstrainedValue, GroupType};
use leo_typed::Span;

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{r1cs::ConstraintSystem, utilities::eq::EvaluateEqGadget},
};

pub fn evaluate_eq<F: Field + PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    left: ConstrainedValue<F, G>,
    right: ConstrainedValue<F, G>,
    span: Span,
) -> Result<ConstrainedValue<F, G>, ExpressionError> {
    let mut unique_namespace = cs.ns(|| format!("evaluate {} == {} {}:{}", left, right, span.line, span.start));
    let constraint_result = match (left, right) {
        (ConstrainedValue::Address(address_1), ConstrainedValue::Address(address_2)) => {
            address_1.evaluate_equal(unique_namespace, &address_2)
        }
        (ConstrainedValue::Boolean(bool_1), ConstrainedValue::Boolean(bool_2)) => {
            bool_1.evaluate_equal(unique_namespace, &bool_2)
        }
        (ConstrainedValue::Integer(num_1), ConstrainedValue::Integer(num_2)) => {
            num_1.evaluate_equal(unique_namespace, &num_2)
        }
        (ConstrainedValue::Field(field_1), ConstrainedValue::Field(field_2)) => {
            field_1.evaluate_equal(unique_namespace, &field_2)
        }
        (ConstrainedValue::Group(point_1), ConstrainedValue::Group(point_2)) => {
            point_1.evaluate_equal(unique_namespace, &point_2)
        }
        (ConstrainedValue::Unresolved(string), val_2) => {
            let val_1 = ConstrainedValue::from_other(string, &val_2, span.clone())?;
            return evaluate_eq(&mut unique_namespace, val_1, val_2, span);
        }
        (val_1, ConstrainedValue::Unresolved(string)) => {
            let val_2 = ConstrainedValue::from_other(string, &val_1, span.clone())?;
            return evaluate_eq(&mut unique_namespace, val_1, val_2, span);
        }
        (val_1, val_2) => {
            return Err(ExpressionError::incompatible_types(
                format!("{} == {}", val_1, val_2,),
                span,
            ));
        }
    };

    let boolean = constraint_result.map_err(|e| ExpressionError::cannot_enforce(format!("evaluate equal"), e, span))?;

    Ok(ConstrainedValue::Boolean(boolean))
}
