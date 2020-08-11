//! Enforces constraints on an implicit number in a compiled Leo program.

use crate::{errors::ValueError, value::ConstrainedValue, GroupType};
use leo_typed::{Span, Type};

use snarkos_models::curves::{Field, PrimeField};

pub fn enforce_number_implicit<F: Field + PrimeField, G: GroupType<F>>(
    expected_type: Option<Type>,
    value: String,
    span: Span,
) -> Result<ConstrainedValue<F, G>, ValueError> {
    match expected_type {
        Some(type_) => Ok(ConstrainedValue::from_type(value, &type_, span)?),
        None => Ok(ConstrainedValue::Unresolved(value)),
    }
}
