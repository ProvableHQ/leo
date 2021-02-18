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

//! Enforces a relational `==` operator in a resolved Leo program.

use crate::{enforce_and, errors::ExpressionError, value::ConstrainedValue, GroupType};
use leo_asg::Span;

use snarkvm_models::{
    curves::PrimeField,
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{boolean::Boolean, eq::EvaluateEqGadget},
    },
};

pub fn evaluate_eq<'a, F: PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    left: ConstrainedValue<'a, F, G>,
    right: ConstrainedValue<'a, F, G>,
    span: &Span,
) -> Result<ConstrainedValue<'a, F, G>, ExpressionError> {
    let namespace_string = format!("evaluate {} == {} {}:{}", left, right, span.line, span.start);
    let constraint_result = match (left, right) {
        (ConstrainedValue::Address(address_1), ConstrainedValue::Address(address_2)) => {
            let unique_namespace = cs.ns(|| namespace_string);
            address_1.evaluate_equal(unique_namespace, &address_2)
        }
        (ConstrainedValue::Boolean(bool_1), ConstrainedValue::Boolean(bool_2)) => {
            let unique_namespace = cs.ns(|| namespace_string);
            bool_1.evaluate_equal(unique_namespace, &bool_2)
        }
        (ConstrainedValue::Integer(num_1), ConstrainedValue::Integer(num_2)) => {
            let unique_namespace = cs.ns(|| namespace_string);
            num_1.evaluate_equal(unique_namespace, &num_2)
        }
        (ConstrainedValue::Field(field_1), ConstrainedValue::Field(field_2)) => {
            let unique_namespace = cs.ns(|| namespace_string);
            field_1.evaluate_equal(unique_namespace, &field_2)
        }
        (ConstrainedValue::Group(point_1), ConstrainedValue::Group(point_2)) => {
            let unique_namespace = cs.ns(|| namespace_string);
            point_1.evaluate_equal(unique_namespace, &point_2)
        }
        (ConstrainedValue::Array(arr_1), ConstrainedValue::Array(arr_2)) => {
            let mut current = ConstrainedValue::Boolean(Boolean::constant(true));
            for (i, (left, right)) in arr_1.into_iter().zip(arr_2.into_iter()).enumerate() {
                let next = evaluate_eq(&mut cs.ns(|| format!("array[{}]", i)), left, right, span)?;

                current = enforce_and(&mut cs.ns(|| format!("array result {}", i)), current, next, span)?;
            }
            return Ok(current);
        }
        (ConstrainedValue::Tuple(tuple_1), ConstrainedValue::Tuple(tuple_2)) => {
            let mut current = ConstrainedValue::Boolean(Boolean::constant(true));

            for (i, (left, right)) in tuple_1.into_iter().zip(tuple_2.into_iter()).enumerate() {
                let next = evaluate_eq(&mut cs.ns(|| format!("tuple_index {}", i)), left, right, span)?;

                current = enforce_and(&mut cs.ns(|| format!("array result {}", i)), current, next, span)?;
            }
            return Ok(current);
        }
        (val_1, val_2) => {
            return Err(ExpressionError::incompatible_types(
                format!("{} == {}", val_1, val_2,),
                span.to_owned(),
            ));
        }
    };

    let boolean = constraint_result.map_err(|_| ExpressionError::cannot_evaluate("==".to_string(), span.to_owned()))?;

    Ok(ConstrainedValue::Boolean(boolean))
}
