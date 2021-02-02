// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use crate::{errors::ImportError, ConstrainedProgram, GroupType};
use leo_ast::ImportStatement;
use leo_imports::ImportParser;
use leo_symbol_table::imported_symbols::ImportedSymbols;

use snarkvm_models::curves::{Field, PrimeField};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub(crate) fn store_import(
        &mut self,
        scope: &str,
        import: &ImportStatement,
        imported_programs: &ImportParser,
    ) -> Result<(), ImportError> {
        // Fetch core packages.
        let core_package = imported_programs.get_core_package(&import.package);

        if let Some(package) = core_package {
            self.store_core_package(scope, package.clone())?;

            return Ok(());
        }

        // Fetch dependencies for the current import
        let imported_symbols = ImportedSymbols::new(import);

        for (name, symbol) in imported_symbols.symbols {
            // Find imported program
            let program = imported_programs
                .get_import(&name)
                .ok_or_else(|| ImportError::unknown_package(import.package.name.clone()))?;

            // Parse imported program
            self.store_definitions(program, imported_programs)?;

            // Store the imported symbol
            self.store_symbol(scope, &name, &symbol, program)?;
        }

        Ok(())
    }
}
