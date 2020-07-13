//! Evaluates a macro in a compiled Leo program.

use crate::{errors::MacroError, program::ConstrainedProgram, GroupType};
use leo_types::{FormattedMacro, MacroName};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn evaluate_macro<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        macro_: FormattedMacro,
    ) -> Result<(), MacroError> {
        let string = macro_
            .string
            .map(|string| self.format(cs, file_scope, function_scope, string))
            .unwrap_or(Ok("".to_string()))?;

        match macro_.name {
            MacroName::Debug(_) => log::debug!("{}", string),
            MacroName::Error(_) => log::error!("{}", string),
            MacroName::Print(_) => println!("{}", string),
        }

        Ok(())
    }
}
