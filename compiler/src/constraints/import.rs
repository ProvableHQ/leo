use crate::{
    ast,
    constraints::{ConstrainedProgram, ConstrainedValue},
    errors::constraints::ImportError,
    new_scope,
    types::Program,
    Import,
};

use from_pest::FromPest;
use snarkos_models::curves::TEModelParameters;
use snarkos_models::gadgets::curves::FieldGadget;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};
use std::env::current_dir;
use std::fs;

impl<
        P: std::clone::Clone + TEModelParameters,
        F: Field + PrimeField,
        FG: FieldGadget<P::BaseField, F>,
        CS: ConstraintSystem<F>,
    > ConstrainedProgram<P, F, FG, CS>
{
    pub fn enforce_import(
        &mut self,
        cs: &mut CS,
        scope: String,
        import: Import<P::BaseField, F>,
    ) -> Result<(), ImportError> {
        let path = current_dir().map_err(|error| ImportError::DirectoryError(error))?;

        // Sanitize the package path to the imports directory
        let mut package_path = path.clone();
        if package_path.is_file() {
            package_path.pop();
        }

        // Construct the path to the import file in the import directory
        let mut main_file_path = package_path.clone();
        main_file_path.push(import.path_string_full());

        println!("Compiling import - {:?}", main_file_path);

        // Resolve program file path
        let unparsed_file = fs::read_to_string(main_file_path.clone())
            .map_err(|_| ImportError::FileReadError(main_file_path))?;
        let mut file = ast::parse(&unparsed_file).map_err(|_| ImportError::FileParsingError)?;

        // generate ast from file
        let syntax_tree =
            ast::File::from_pest(&mut file).map_err(|_| ImportError::SyntaxTreeError)?;

        // generate aleo program from file
        let mut program = Program::from(syntax_tree, import.path_string.clone());

        // Use same namespace as calling function for imported symbols
        program = program.name(scope);

        // * -> import all imports, circuits, functions in the current scope
        if import.is_star() {
            // recursively evaluate program statements
            self.resolve_definitions(cs, program).unwrap();

            Ok(())
        } else {
            let program_name = program.name.clone();

            // match each import symbol to a symbol in the imported file
            for symbol in import.symbols.into_iter() {
                // see if the imported symbol is a circuit
                let matched_circuit = program
                    .circuits
                    .clone()
                    .into_iter()
                    .find(|(circuit_name, _circuit_def)| symbol.symbol == *circuit_name);

                let value = match matched_circuit {
                    Some((_circuit_name, circuit_def)) => {
                        ConstrainedValue::CircuitDefinition(circuit_def)
                    }
                    None => {
                        // see if the imported symbol is a function
                        let matched_function = program.functions.clone().into_iter().find(
                            |(function_name, _function)| symbol.symbol.name == *function_name.name,
                        );

                        match matched_function {
                            Some((_function_name, function)) => {
                                ConstrainedValue::Function(None, function)
                            }
                            None => unimplemented!(
                                "cannot find imported symbol {} in imported file {}",
                                symbol,
                                program_name.clone()
                            ),
                        }
                    }
                };

                // take the alias if it is present
                let resolved_name = symbol.alias.unwrap_or(symbol.symbol);
                let resolved_circuit_name =
                    new_scope(program_name.to_string(), resolved_name.to_string());

                // store imported circuit under resolved name
                self.store(resolved_circuit_name, value);
            }

            // evaluate all import statements in imported file
            program
                .imports
                .into_iter()
                .map(|nested_import| {
                    self.enforce_import(cs, program_name.name.clone(), nested_import)
                })
                .collect::<Result<Vec<_>, ImportError>>()?;

            Ok(())
        }
    }
}
