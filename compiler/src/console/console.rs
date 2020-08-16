//! Evaluates a macro in a compiled Leo program.

use crate::{errors::ConsoleError, program::ConstrainedProgram, GroupType};
use leo_typed::{ConsoleFunction, ConsoleFunctionCall};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn evaluate_console_function_call<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        console: ConsoleFunctionCall,
    ) -> Result<(), ConsoleError> {
        match console.function {
            ConsoleFunction::Assert(expression) => {
                self.evaluate_console_assert(cs, file_scope, function_scope, expression, console.span)?;
            }
            ConsoleFunction::Debug(string) => {
                let string = self.format(cs, file_scope, function_scope, string)?;

                log::debug!("{}", string);
            }
            ConsoleFunction::Error(string) => {
                let string = self.format(cs, file_scope, function_scope, string)?;

                log::error!("{}", string);
            }
            ConsoleFunction::Log(string) => {
                let string = self.format(cs, file_scope, function_scope, string)?;

                println!("{}", string);
            }
        }

        Ok(())
    }
}
