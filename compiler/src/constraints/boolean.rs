//! Methods to enforce constraints on booleans in a resolved Leo program.

use crate::{constraints::ConstrainedValue, errors::BooleanError, GroupType};
use leo_types::{InputValue, Span};

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{alloc::AllocGadget, boolean::Boolean},
    },
};

pub(crate) fn new_bool_constant(string: String, span: Span) -> Result<Boolean, BooleanError> {
    let boolean = string
        .parse::<bool>()
        .map_err(|_| BooleanError::invalid_boolean(string, span))?;

    Ok(Boolean::constant(boolean))
}

pub(crate) fn bool_from_input<F: Field + PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    name: String,
    input_value: Option<InputValue>,
    span: Span,
) -> Result<ConstrainedValue<F, G>, BooleanError> {
    // Check that the input value is the correct type
    let bool_value = match input_value {
        Some(input) => {
            if let InputValue::Boolean(bool) = input {
                Some(bool)
            } else {
                return Err(BooleanError::invalid_boolean(name, span));
            }
        }
        None => None,
    };

    let boolean_name = format!("{}: bool", name);
    let boolean_name_unique = format!("`{}` {}:{}", boolean_name, span.line, span.start);
    let number = Boolean::alloc(cs.ns(|| boolean_name_unique), || {
        bool_value.ok_or(SynthesisError::AssignmentMissing)
    })
    .map_err(|_| BooleanError::missing_boolean(boolean_name, span))?;

    Ok(ConstrainedValue::Boolean(number))
}

pub(crate) fn evaluate_not<F: Field + PrimeField, G: GroupType<F>>(
    value: ConstrainedValue<F, G>,
    span: Span,
) -> Result<ConstrainedValue<F, G>, BooleanError> {
    match value {
        ConstrainedValue::Boolean(boolean) => Ok(ConstrainedValue::Boolean(boolean.not())),
        value => Err(BooleanError::cannot_evaluate(format!("!{}", value), span)),
    }
}

pub(crate) fn enforce_or<F: Field + PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    left: ConstrainedValue<F, G>,
    right: ConstrainedValue<F, G>,
    span: Span,
) -> Result<ConstrainedValue<F, G>, BooleanError> {
    match (left, right) {
        (ConstrainedValue::Boolean(left_bool), ConstrainedValue::Boolean(right_bool)) => Ok(ConstrainedValue::Boolean(
            Boolean::or(cs, &left_bool, &right_bool)
                .map_err(|e| BooleanError::cannot_enforce(format!("||"), e, span))?,
        )),
        (left_value, right_value) => Err(BooleanError::cannot_evaluate(
            format!("{} || {}", left_value, right_value),
            span,
        )),
    }
}

pub(crate) fn enforce_and<F: Field + PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    left: ConstrainedValue<F, G>,
    right: ConstrainedValue<F, G>,
    span: Span,
) -> Result<ConstrainedValue<F, G>, BooleanError> {
    match (left, right) {
        (ConstrainedValue::Boolean(left_bool), ConstrainedValue::Boolean(right_bool)) => Ok(ConstrainedValue::Boolean(
            Boolean::and(cs, &left_bool, &right_bool)
                .map_err(|e| BooleanError::cannot_enforce(format!("&&"), e, span))?,
        )),
        (left_value, right_value) => Err(BooleanError::cannot_evaluate(
            format!("{} && {}", left_value, right_value),
            span,
        )),
    }
}
