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

//! Stores all defined names in a compiled Leo program.

use crate::{
    errors::ImportError,
    program::{new_scope, ConstrainedProgram},
    value::ConstrainedValue,
    GroupType,
};
use leo_ast::Program;
use leo_imports::ImportParser;

use snarkos_models::curves::{Field, PrimeField};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn store_definitions(
        &mut self,
        program: &Program,
        imported_programs: &ImportParser,
    ) -> Result<(), ImportError> {
        let program_name = program.name.trim_end_matches(".leo");

        // evaluate all import statements and store imported definitions
        program
            .imports
            .iter()
            .map(|import| self.store_import(&program_name, import, imported_programs))
            .collect::<Result<Vec<_>, ImportError>>()?;

        // evaluate and store all circuit definitions
        program.circuits.iter().for_each(|(identifier, circuit)| {
            let resolved_circuit_name = new_scope(program_name, &identifier.name);
            self.store(
                resolved_circuit_name,
                ConstrainedValue::CircuitDefinition(circuit.clone()),
            );
        });

        // evaluate and store all function definitions
        program.functions.iter().for_each(|(function_name, function)| {
            let resolved_function_name = new_scope(program_name, &function_name.name);
            self.store(
                resolved_function_name,
                ConstrainedValue::Function(None, Box::new(function.clone())),
            );
        });

        Ok(())
    }
}
