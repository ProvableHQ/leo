//! Stores all defined names in a compiled Leo program.

use crate::{
    program::{new_scope, ConstrainedProgram},
    value::ConstrainedValue,
    GroupType,
};
use leo_typed::Variable;

use snarkos_models::curves::{Field, PrimeField};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn store_definition(
        &mut self,
        function_scope: String,
        variable: Variable,
        mut value: ConstrainedValue<F, G>,
    ) -> () {
        // Store with given mutability
        if variable.mutable {
            value = ConstrainedValue::Mutable(Box::new(value));
        }

        let variable_program_identifier = new_scope(function_scope, variable.identifier.name);

        self.store(variable_program_identifier, value);
    }
}
