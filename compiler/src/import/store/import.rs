use crate::{errors::ImportError, imported_symbols::ImportedSymbols, ConstrainedProgram, GroupType, ImportParser};
use leo_typed::Import;

use snarkos_models::curves::{Field, PrimeField};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub(crate) fn store_import(
        &mut self,
        scope: String,
        import: &Import,
        imported_programs: &ImportParser,
    ) -> Result<(), ImportError> {
        // fetch dependencies for the current import
        let imported_symbols = ImportedSymbols::from(import);

        for (package, symbol) in imported_symbols.symbols {
            // find imported program
            let program = imported_programs
                .get(&package)
                .ok_or(ImportError::unknown_package(import.package.name.clone()))?;

            // parse imported program
            self.store_definitions(program.clone(), imported_programs)?;

            // store the imported symbol
            self.store_symbol(scope.clone(), package, &symbol, program)?;
        }

        Ok(())
    }
}
