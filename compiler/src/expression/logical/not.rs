//! Methods to enforce constraints on boolean operations in a resolved Leo program.

use crate::{errors::BooleanError, value::ConstrainedValue, GroupType};
use leo_types::Span;

use snarkos_models::curves::{Field, PrimeField};

pub(crate) fn evaluate_not<F: Field + PrimeField, G: GroupType<F>>(
    value: ConstrainedValue<F, G>,
    span: Span,
) -> Result<ConstrainedValue<F, G>, BooleanError> {
    match value {
        ConstrainedValue::Boolean(boolean) => Ok(ConstrainedValue::Boolean(boolean.not())),
        value => Err(BooleanError::cannot_evaluate(format!("!{}", value), span)),
    }
}
