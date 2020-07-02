use crate::{errors::ImportError, new_scope, ConstrainedProgram, ConstrainedValue, GroupType};
use leo_types::{ImportSymbol, Program};

use snarkos_models::curves::{Field, PrimeField};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub(crate) fn store_symbol(
        &mut self,
        scope: String,
        program_name: String,
        symbol: &ImportSymbol,
        program: &Program,
    ) -> Result<(), ImportError> {
        if symbol.is_star() {
            // evaluate and store all circuit definitions
            program.circuits.iter().for_each(|(identifier, circuit)| {
                let name = new_scope(scope.clone(), identifier.to_string());
                let value = ConstrainedValue::Import(
                    program_name.clone(),
                    Box::new(ConstrainedValue::CircuitDefinition(circuit.clone())),
                );

                self.store(name, value);
            });

            // evaluate and store all function definitions
            program.functions.iter().for_each(|(identifier, function)| {
                let name = new_scope(scope.clone(), identifier.to_string());
                let value = ConstrainedValue::Import(
                    program_name.clone(),
                    Box::new(ConstrainedValue::Function(None, function.clone())),
                );

                self.store(name, value);
            });
        } else {
            // see if the imported symbol is a circuit
            let matched_circuit = program
                .circuits
                .iter()
                .find(|(circuit_name, _circuit_def)| symbol.symbol == **circuit_name);

            let value = match matched_circuit {
                Some((_circuit_name, circuit)) => ConstrainedValue::Import(
                    program_name.clone(),
                    Box::new(ConstrainedValue::CircuitDefinition(circuit.clone())),
                ),
                None => {
                    // see if the imported symbol is a function
                    let matched_function = program
                        .functions
                        .iter()
                        .find(|(function_name, _function)| symbol.symbol == **function_name);

                    match matched_function {
                        Some((_function_name, function)) => ConstrainedValue::Import(
                            program_name.clone(),
                            Box::new(ConstrainedValue::Function(None, function.clone())),
                        ),
                        None => return Err(ImportError::unknown_symbol(symbol.to_owned(), scope)),
                    }
                }
            };

            // take the alias if it is present
            let id = symbol.alias.clone().unwrap_or(symbol.symbol.clone());
            let name = new_scope(scope, id.to_string());

            // store imported circuit under imported name
            self.store(name, value);
        }

        Ok(())
    }
}
