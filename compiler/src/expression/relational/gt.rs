//! Methods to enforce constraints on `>` comparison expressions in a compiled Leo program.

use crate::{comparator::ComparatorGadget, errors::ExpressionError, value::ConstrainedValue, GroupType};
use leo_types::Span;

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

pub fn evaluate_gt_expression<F: Field + PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    left: ConstrainedValue<F, G>,
    right: ConstrainedValue<F, G>,
    span: Span,
) -> Result<ConstrainedValue<F, G>, ExpressionError> {
    let mut unique_namespace = cs.ns(|| format!("evaluate {} > {} {}:{}", left, right, span.line, span.start));
    let constraint_result = match (left, right) {
        (ConstrainedValue::Integer(num_1), ConstrainedValue::Integer(num_2)) => {
            num_1.greater_than(unique_namespace, &num_2)
        }
        (ConstrainedValue::Field(field_1), ConstrainedValue::Field(field_2)) => {
            field_1.greater_than(unique_namespace, &field_2)
        }
        (ConstrainedValue::Unresolved(string), val_2) => {
            let val_1 = ConstrainedValue::from_other(string, &val_2, span.clone())?;
            return evaluate_gt_expression(&mut unique_namespace, val_1, val_2, span);
        }
        (val_1, ConstrainedValue::Unresolved(string)) => {
            let val_2 = ConstrainedValue::from_other(string, &val_1, span.clone())?;
            return evaluate_gt_expression(&mut unique_namespace, val_1, val_2, span);
        }
        (val_1, val_2) => {
            return Err(ExpressionError::incompatible_types(
                format!("{} > {}", val_1, val_2),
                span,
            ));
        }
    };

    let boolean =
        constraint_result.map_err(|e| ExpressionError::cannot_enforce(format!("evaluate greater than"), e, span))?;

    Ok(ConstrainedValue::Boolean(boolean))
}
