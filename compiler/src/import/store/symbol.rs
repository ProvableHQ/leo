// Copyright (C) 2019-2020 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::{errors::ImportError, new_scope, ConstrainedProgram, ConstrainedValue, GroupType};
use leo_ast::{ImportSymbol, Program};

use snarkos_models::curves::{Field, PrimeField};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub(crate) fn store_symbol(
        &mut self,
        scope: &str,
        program_name: &str,
        symbol: &ImportSymbol,
        program: &Program,
    ) -> Result<(), ImportError> {
        // Store the symbol that was imported by another file
        if symbol.is_star() {
            // evaluate and store all circuit definitions
            program.circuits.iter().for_each(|(identifier, circuit)| {
                let name = new_scope(scope, &identifier.name);
                let value = ConstrainedValue::Import(
                    program_name.to_owned(),
                    Box::new(ConstrainedValue::CircuitDefinition(circuit.clone())),
                );

                self.store(name, value);
            });

            // evaluate and store all function definitions
            program.functions.iter().for_each(|(identifier, function)| {
                let name = new_scope(scope, &identifier.name);
                let value = ConstrainedValue::Import(
                    program_name.to_owned(),
                    Box::new(ConstrainedValue::Function(None, Box::new(function.clone()))),
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
                    program_name.to_owned(),
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
                            program_name.to_owned(),
                            Box::new(ConstrainedValue::Function(None, Box::new(function.clone()))),
                        ),
                        None => return Err(ImportError::unknown_symbol(symbol.to_owned(), program_name.to_owned())),
                    }
                }
            };

            // take the alias if it is present
            let id = symbol.alias.clone().unwrap_or_else(|| symbol.symbol.clone());
            let name = new_scope(scope, &id.name);

            // store imported circuit under imported name
            self.store(name, value);
        }

        Ok(())
    }
}
