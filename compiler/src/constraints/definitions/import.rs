use crate::{errors::ImportError, imported_symbols::ImportedSymbols, ConstrainedProgram, GroupType, ImportedPrograms};
use leo_types::Import;

use snarkos_models::curves::{Field, PrimeField};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub(crate) fn store_import(
        &mut self,
        scope: String,
        import: &Import,
        imported_programs: &ImportedPrograms,
    ) -> Result<(), ImportError> {
        // get imported program name from import
        // get imported symbols from from import
        let imported_symbols = ImportedSymbols::from(import);

        for (package, symbol) in imported_symbols.symbols {
            // get imported program from hashmap
            let program = imported_programs
                .get(&package)
                .ok_or(ImportError::unknown_package(import.package.name.clone()))?;

            // resolve imported program's import statements
            program
                .imports
                .iter()
                .map(|import| self.store_import(package.clone(), import, imported_programs))
                .collect::<Result<Vec<()>, ImportError>>()?;

            self.store_symbol(scope.clone(), package, &symbol, program)?;
        }

        Ok(())
    }
}
