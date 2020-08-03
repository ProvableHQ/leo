//! Enforces a branch of a conditional or iteration statement in a compiled Leo program.

use crate::{errors::StatementError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_typed::{Statement, Type};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{r1cs::ConstraintSystem, utilities::boolean::Boolean},
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn evaluate_branch<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        indicator: Option<Boolean>,
        statements: Vec<Statement>,
        return_types: Vec<Type>,
    ) -> Result<Vec<(Option<Boolean>, ConstrainedValue<F, G>)>, StatementError> {
        let mut results = vec![];
        // Evaluate statements. Only allow a single return argument to be returned.
        for statement in statements.iter() {
            let mut value = self.enforce_statement(
                cs,
                file_scope.clone(),
                function_scope.clone(),
                indicator.clone(),
                statement.clone(),
                return_types.clone(),
            )?;

            results.append(&mut value);
        }

        Ok(results)
    }
}
