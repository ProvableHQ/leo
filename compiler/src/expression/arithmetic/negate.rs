//! Enforces a unary negate `-` operator in a resolved Leo program.

use crate::{errors::ExpressionError, value::ConstrainedValue, GroupType};
use leo_typed::Span;

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

pub fn enforce_negate<F: Field + PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    value: ConstrainedValue<F, G>,
    span: Span,
) -> Result<ConstrainedValue<F, G>, ExpressionError> {
    match value {
        ConstrainedValue::Integer(integer) => Ok(ConstrainedValue::Integer(integer.negate(cs, span)?)),
        ConstrainedValue::Field(field) => Ok(ConstrainedValue::Field(field.negate(cs, span)?)),
        ConstrainedValue::Group(group) => Ok(ConstrainedValue::Group(group.negate(cs, span)?)),
        value => Err(ExpressionError::incompatible_types(format!("-{}", value), span)),
    }
}
