use crate::{errors::ImportError, ConstrainedProgram, GroupType, ImportedPrograms, ImportedSymbols};
use leo_types::Import;

use snarkos_models::curves::{Field, PrimeField};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub(crate) fn store_import(
        &mut self,
        scope: String,
        import: &Import,
        imported_programs: &ImportedPrograms,
    ) -> Result<(), ImportError> {
        println!("program name {}", scope);
        println!("import {}", import);

        // get imported program name from import
        // get imported symbols from from import
        let imported_symbols = ImportedSymbols::from(import);
        let program_name = imported_symbols.name.clone();
        println!("symbols {:?}", imported_symbols);

        // get imported program from hashmap
        let program = imported_programs
            .get(&program_name)
            .ok_or(ImportError::unknown_package(import.package.name.clone()))?;

        // resolve imported program's import statements
        program
            .imports
            .iter()
            .map(|import| self.store_import(program_name.clone(), import, imported_programs))
            .collect::<Result<Vec<()>, ImportError>>()?;

        // store imported symbols in constrained program
        imported_symbols
            .symbols
            .iter()
            .map(|symbol| self.store_symbol(scope.clone(), symbol, program))
            .collect::<Result<Vec<()>, ImportError>>()?;

        Ok(())
    }
}
