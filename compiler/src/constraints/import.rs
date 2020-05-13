use crate::{
    ast,
    constraints::{new_variable_from_variables, ConstrainedProgram, ConstrainedValue},
    errors::constraints::ImportError,
    types::Program,
    Import,
};

use from_pest::FromPest;
use snarkos_models::{
    curves::{Field, Group, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};
use std::fs;
use std::path::Path;

impl<F: Field + PrimeField, G: Group, CS: ConstraintSystem<F>> ConstrainedProgram<F, G, CS> {
    pub fn enforce_import(
        &mut self,
        cs: &mut CS,
        scope: String,
        import: Import<F, G>,
    ) -> Result<(), ImportError> {
        // Resolve program file path
        let unparsed_file = fs::read_to_string(Path::new(&import.path_string_full()))
            .map_err(|_| ImportError::FileReadError(import.path_string_full()))?;
        let mut file = ast::parse(&unparsed_file).map_err(|_| ImportError::FileParsingError)?;

        // generate ast from file
        let syntax_tree =
            ast::File::from_pest(&mut file).map_err(|_| ImportError::SyntaxTreeError)?;

        // generate aleo program from file
        let mut program = Program::from(syntax_tree, import.path_string.clone());

        // Use same namespace as calling function for imported symbols
        program = program.name(scope);

        // * -> import all imports, structs, functions in the current scope
        if import.is_star() {
            // recursively evaluate program statements
            self.resolve_definitions(cs, program).unwrap();

            Ok(())
        } else {
            let program_name = program.name.clone();

            // match each import symbol to a symbol in the imported file
            import.symbols.into_iter().for_each(|symbol| {
                // see if the imported symbol is a struct
                let matched_struct = program
                    .structs
                    .clone()
                    .into_iter()
                    .find(|(struct_name, _struct_def)| symbol.symbol == *struct_name);

                match matched_struct {
                    Some((_struct_name, struct_def)) => {
                        // take the alias if it is present
                        let resolved_name = symbol.alias.unwrap_or(symbol.symbol);
                        let resolved_struct_name =
                            new_variable_from_variables(&program_name.clone(), &resolved_name);

                        // store imported struct under resolved name
                        self.store_variable(
                            resolved_struct_name,
                            ConstrainedValue::StructDefinition(struct_def),
                        );
                    }
                    None => {
                        // see if the imported symbol is a function
                        let matched_function = program.functions.clone().into_iter().find(
                            |(function_name, _function)| symbol.symbol.name == *function_name.0,
                        );

                        match matched_function {
                            Some((_function_name, function)) => {
                                // take the alias if it is present
                                let resolved_name = symbol.alias.unwrap_or(symbol.symbol);
                                let resolved_function_name = new_variable_from_variables(
                                    &program_name.clone(),
                                    &resolved_name,
                                );

                                // store imported function under resolved name
                                self.store_variable(
                                    resolved_function_name,
                                    ConstrainedValue::Function(function),
                                )
                            }
                            None => unimplemented!(
                                "cannot find imported symbol {} in imported file {}",
                                symbol,
                                program_name.clone()
                            ),
                        }
                    }
                }
            });

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
