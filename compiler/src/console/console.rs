//! Evaluates a macro in a compiled Leo program.

use crate::{errors::ConsoleError, program::ConstrainedProgram, GroupType};
use leo_typed::{ConsoleFunction, ConsoleFunctionCall};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{r1cs::ConstraintSystem, utilities::boolean::Boolean},
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn evaluate_console_function_call<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        indicator: Option<Boolean>,
        console: ConsoleFunctionCall,
    ) -> Result<(), ConsoleError> {
        match console.function {
            ConsoleFunction::Assert(expression) => {
                self.evaluate_console_assert(cs, file_scope, function_scope, indicator, expression, console.span)?;
            }
            ConsoleFunction::Debug(string) => {
                let string = self.format(cs, file_scope, function_scope, string)?;

                if unwrap_indicator_value(indicator) {
                    log::debug!("{}", string);
                }
            }
            ConsoleFunction::Error(string) => {
                let string = self.format(cs, file_scope, function_scope, string)?;

                if unwrap_indicator_value(indicator) {
                    log::error!("{}", string);
                }
            }
            ConsoleFunction::Log(string) => {
                let string = self.format(cs, file_scope, function_scope, string)?;

                if unwrap_indicator_value(indicator) {
                    log::info!("{}", string);
                }
            }
        }

        Ok(())
    }
}

// Return the indicator boolean gadget value or true if it is None
// This is okay since we are not enforcing any constraints
fn unwrap_indicator_value(indicator: Option<Boolean>) -> bool {
    let false_boolean = Boolean::constant(false);

    if let Some(indicator_bool) = indicator {
        if indicator_bool.eq(&false_boolean) {
            return false;
        }
    }
    true
}
