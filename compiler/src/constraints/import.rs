use crate::{
    constraints::{ConstrainedProgram, ConstrainedValue},
    errors::constraints::ImportError,
    new_scope,
    GroupType,
};
use leo_ast::LeoParser;
use leo_types::{Import, Program};

use snarkos_models::curves::{Field, PrimeField};
use std::env::current_dir;

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn enforce_import(&mut self, scope: String, import: Import) -> Result<(), ImportError> {
        let path = current_dir().map_err(|error| ImportError::directory_error(error, import.span.clone()))?;

        // Sanitize the package path to the imports directory
        let mut package_path = path.clone();
        if package_path.is_file() {
            package_path.pop();
        }

        // Construct the path to the import file in the import directory
        let mut main_file_path = package_path.clone();
        main_file_path.push(import.path_string_full());

        println!("Compiling import - {:?}", main_file_path);

        // Build the abstract syntax tree
        let file_path = &main_file_path;
        let input_file = &LeoParser::load_file(file_path)?;
        let syntax_tree = LeoParser::parse_file(file_path, input_file)?;

        // Generate aleo program from file
        let mut program = Program::from(syntax_tree, import.path_string.clone());

        // Use same namespace as calling function for imported symbols
        program = program.name(scope);

        // * -> import all imports, circuits, functions in the current scope
        if import.is_star() {
            // recursively evaluate program statements
            self.resolve_definitions(program)
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
                    Some((_circuit_name, circuit_def)) => ConstrainedValue::CircuitDefinition(circuit_def),
                    None => {
                        // see if the imported symbol is a function
                        let matched_function = program
                            .functions
                            .clone()
                            .into_iter()
                            .find(|(function_name, _function)| symbol.symbol.name == *function_name.name);

                        match matched_function {
                            Some((_function_name, function)) => ConstrainedValue::Function(None, function),
                            None => return Err(ImportError::unknown_symbol(symbol, program_name, file_path)),
                        }
                    }
                };

                // take the alias if it is present
                let resolved_name = symbol.alias.unwrap_or(symbol.symbol);
                let resolved_circuit_name = new_scope(program_name.clone(), resolved_name.to_string());

                // store imported circuit under resolved name
                self.store(resolved_circuit_name, value);
            }

            // evaluate all import statements in imported file
            program
                .imports
                .into_iter()
                .map(|nested_import| self.enforce_import(program_name.clone(), nested_import))
                .collect::<Result<Vec<_>, ImportError>>()?;

            Ok(())
        }
    }
}
