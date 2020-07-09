//! Evaluates a macro in a compiled Leo program.

use crate::{errors::MacroError, program::ConstrainedProgram, GroupType};
use leo_types::FormattedString;

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn format<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        formatted: FormattedString,
    ) -> Result<String, MacroError> {
        // Check that containers and parameters match
        if formatted.containers.len() != formatted.parameters.len() {
            return Err(MacroError::length(
                formatted.containers.len(),
                formatted.parameters.len(),
                formatted.span.clone(),
            ));
        }

        // Trim starting double quote `"`
        let mut string = formatted.string.as_str();
        string = string.trim_start_matches("\"");

        // Trim everything after the ending double quote `"`
        let parts: Vec<&str> = string.split("\"").collect();
        string = parts[0];

        // Insert the parameter for each container `{}`
        let mut result = string.to_string();

        for parameter in formatted.parameters.into_iter() {
            let parameter_value = self.enforce_expression(
                cs,
                file_scope.clone(),
                function_scope.clone(),
                &vec![],
                parameter.expression,
            )?;

            result = result.replacen("{}", &parameter_value.to_string(), 1);
        }

        Ok(result)
    }
}
