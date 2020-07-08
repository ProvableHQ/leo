//! Methods to enforce constraints on booleans in a resolved Leo program.

use crate::{errors::BooleanError, value::ConstrainedValue, GroupType};
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

pub(crate) fn allocate_bool<F: Field + PrimeField, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    name: String,
    option: Option<bool>,
    span: Span,
) -> Result<Boolean, BooleanError> {
    let boolean_name = format!("{}: bool", name);
    let boolean_name_unique = format!("`{}` {}:{}", boolean_name, span.line, span.start);

    Boolean::alloc(cs.ns(|| boolean_name_unique), || {
        option.ok_or(SynthesisError::AssignmentMissing)
    })
    .map_err(|_| BooleanError::missing_boolean(boolean_name, span))
}

pub(crate) fn bool_from_input<F: Field + PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    name: String,
    input_value: Option<InputValue>,
    span: Span,
) -> Result<ConstrainedValue<F, G>, BooleanError> {
    // Check that the input value is the correct type
    let option = match input_value {
        Some(input) => {
            if let InputValue::Boolean(bool) = input {
                Some(bool)
            } else {
                return Err(BooleanError::invalid_boolean(name, span));
            }
        }
        None => None,
    };

    let number = allocate_bool(cs, name, option, span)?;

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
    let name = format!("{} || {}", left, right);

    if let (ConstrainedValue::Boolean(left_bool), ConstrainedValue::Boolean(right_bool)) = (left, right) {
        let name_unique = format!("{} {}:{}", name, span.line, span.start);
        let result = Boolean::or(cs.ns(|| name_unique), &left_bool, &right_bool)
            .map_err(|e| BooleanError::cannot_enforce(format!("||"), e, span))?;

        return Ok(ConstrainedValue::Boolean(result));
    }

    Err(BooleanError::cannot_evaluate(name, span))
}

pub(crate) fn enforce_and<F: Field + PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    left: ConstrainedValue<F, G>,
    right: ConstrainedValue<F, G>,
    span: Span,
) -> Result<ConstrainedValue<F, G>, BooleanError> {
    let name = format!("{} && {}", left, right);

    if let (ConstrainedValue::Boolean(left_bool), ConstrainedValue::Boolean(right_bool)) = (left, right) {
        let name_unique = format!("{} {}:{}", name, span.line, span.start);
        let result = Boolean::and(cs.ns(|| name_unique), &left_bool, &right_bool)
            .map_err(|e| BooleanError::cannot_enforce(format!("&&"), e, span))?;

        return Ok(ConstrainedValue::Boolean(result));
    }

    Err(BooleanError::cannot_evaluate(name, span))
}
