use crate::{
    constraints::{new_scope, ConstrainedProgram, ConstrainedValue},
    errors::ImportError,
    GroupType,
    ImportedPrograms,
};
use leo_types::Program;

use snarkos_models::curves::{Field, PrimeField};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub(crate) fn store_definitions(
        &mut self,
        program: Program,
        imported_programs: &ImportedPrograms,
    ) -> Result<(), ImportError> {
        let program_name = program.name.clone();

        // evaluate all import statements and store imported definitions
        program
            .imports
            .iter()
            .map(|import| self.store_import(program_name.clone(), import, imported_programs))
            .collect::<Result<Vec<_>, ImportError>>()?;

        self.store_all(program_name.clone(), &program);

        Ok(())
    }

    pub(crate) fn store_all(&mut self, scope: String, program: &Program) {
        // evaluate and store all circuit definitions
        program.circuits.iter().for_each(|(identifier, circuit)| {
            let resolved_circuit_name = new_scope(scope.clone(), identifier.to_string());
            self.store(
                resolved_circuit_name,
                ConstrainedValue::CircuitDefinition(circuit.clone()),
            );
        });

        // evaluate and store all function definitions
        program.functions.iter().for_each(|(function_name, function)| {
            let resolved_function_name = new_scope(scope.clone(), function_name.to_string());
            self.store(
                resolved_function_name,
                ConstrainedValue::Function(None, function.clone()),
            );
        });
    }
}
