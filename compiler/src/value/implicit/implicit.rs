//! Enforces constraints on an implicit number in a compiled Leo program.

use crate::{errors::ValueError, value::ConstrainedValue, GroupType};
use leo_types::{Span, Type};

use snarkos_models::curves::{Field, PrimeField};

pub fn enforce_number_implicit<F: Field + PrimeField, G: GroupType<F>>(
    expected_types: &Vec<Type>,
    value: String,
    span: Span,
) -> Result<ConstrainedValue<F, G>, ValueError> {
    if expected_types.len() == 1 {
        return Ok(ConstrainedValue::from_type(value, &expected_types[0], span)?);
    }

    Ok(ConstrainedValue::Unresolved(value))
}
