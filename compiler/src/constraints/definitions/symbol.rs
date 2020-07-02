use crate::{errors::ImportError, new_scope, ConstrainedProgram, ConstrainedValue, GroupType};
use leo_types::{ImportSymbol, Program};

use snarkos_models::curves::{Field, PrimeField};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub(crate) fn store_symbol(
        &mut self,
        scope: String,
        symbol: &ImportSymbol,
        program: &Program,
    ) -> Result<(), ImportError> {
        if symbol.is_star() {
            self.store_all(scope, program);
        } else {
            let matched_circuit = program
                .circuits
                .iter()
                .find(|(circuit_name, _circuit_def)| symbol.symbol == **circuit_name);

            let value = match matched_circuit {
                Some((_circuit_name, circuit_def)) => ConstrainedValue::CircuitDefinition(circuit_def.clone()),
                None => {
                    // see if the imported symbol is a function
                    let matched_function = program
                        .functions
                        .iter()
                        .find(|(function_name, _function)| symbol.symbol == **function_name);

                    match matched_function {
                        Some((_function_name, function)) => ConstrainedValue::Function(None, function.clone()),
                        None => return Err(ImportError::unknown_symbol(symbol.to_owned(), scope)),
                    }
                }
            };

            // take the alias if it is present
            let name = symbol.alias.clone().unwrap_or(symbol.symbol.clone());
            let resolved_name = new_scope(scope, name.to_string());

            // store imported circuit under resolved name
            self.store(resolved_name, value);
        }

        Ok(())
    }
}
