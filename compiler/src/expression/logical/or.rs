//! Methods to enforce constraints on boolean operations in a resolved Leo program.

use crate::{errors::BooleanError, value::ConstrainedValue, GroupType};
use leo_types::Span;

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{r1cs::ConstraintSystem, utilities::boolean::Boolean},
};

pub(crate) fn enforce_or<F: Field + PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    left: ConstrainedValue<F, G>,
    right: ConstrainedValue<F, G>,
    span: Span,
) -> Result<ConstrainedValue<F, G>, BooleanError> {
    let name = format!("{} || {}", left, right);

    if let (ConstrainedValue::Boolean(left_bool), ConstrainedValue::Boolean(right_bool)) = (left, right) {
        let name_unique = format!("{} {}:{}", name, span.line, span.start);
        let result = Boolean::or(cs.ns(|| name_unique), &left_bool, &right_bool)
            .map_err(|e| BooleanError::cannot_enforce(format!("||"), e, span))?;

        return Ok(ConstrainedValue::Boolean(result));
    }

    Err(BooleanError::cannot_evaluate(name, span))
}
