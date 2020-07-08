//! Methods to enforce arithmetic addition in a compiled Leo program.

use crate::{errors::ExpressionError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_types::Span;

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    /// Enforce numerical operations
    pub fn enforce_add_expression<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        left: ConstrainedValue<F, G>,
        right: ConstrainedValue<F, G>,
        span: Span,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        match (left, right) {
            (ConstrainedValue::Integer(num_1), ConstrainedValue::Integer(num_2)) => {
                Ok(ConstrainedValue::Integer(num_1.add(cs, num_2, span)?))
            }
            (ConstrainedValue::Field(field_1), ConstrainedValue::Field(field_2)) => {
                Ok(ConstrainedValue::Field(field_1.add(cs, &field_2, span)?))
            }
            (ConstrainedValue::Group(point_1), ConstrainedValue::Group(point_2)) => {
                Ok(ConstrainedValue::Group(point_1.add(cs, &point_2, span)?))
            }
            (ConstrainedValue::Unresolved(string), val_2) => {
                let val_1 = ConstrainedValue::from_other(string, &val_2, span.clone())?;
                self.enforce_add_expression(cs, val_1, val_2, span)
            }
            (val_1, ConstrainedValue::Unresolved(string)) => {
                let val_2 = ConstrainedValue::from_other(string, &val_1, span.clone())?;
                self.enforce_add_expression(cs, val_1, val_2, span)
            }
            (val_1, val_2) => Err(ExpressionError::incompatible_types(
                format!("{} + {}", val_1, val_2),
                span,
            )),
        }
    }
}
