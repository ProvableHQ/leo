//! Resolves assignees in a compiled Leo program.

use crate::{errors::StatementError, new_scope, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_typed::{Assignee, Span};

use snarkos_models::curves::{Field, PrimeField};

pub fn resolve_assignee(scope: String, assignee: Assignee) -> String {
    match assignee {
        Assignee::Identifier(name) => new_scope(scope, name.to_string()),
        Assignee::Array(array, _index) => resolve_assignee(scope, *array),
        Assignee::CircuitField(circuit_name, _member) => resolve_assignee(scope, *circuit_name),
    }
}

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn get_mutable_assignee(
        &mut self,
        name: String,
        span: Span,
    ) -> Result<&mut ConstrainedValue<F, G>, StatementError> {
        // Check that assignee exists and is mutable
        Ok(match self.get_mut(&name) {
            Some(value) => match value {
                ConstrainedValue::Mutable(mutable_value) => mutable_value,
                _ => return Err(StatementError::immutable_assign(name, span)),
            },
            None => return Err(StatementError::undefined_variable(name, span)),
        })
    }
}
