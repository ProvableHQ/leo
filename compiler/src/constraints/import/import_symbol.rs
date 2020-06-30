use crate::{
    constraints::{ConstrainedProgram, ConstrainedValue},
    errors::constraints::ImportError,
    new_scope,
    GroupType,
};
use leo_ast::LeoParser;
use leo_types::{ImportSymbol, Program, Span};

use snarkos_models::curves::{Field, PrimeField};
use std::{ffi::OsString, fs::DirEntry};

static LIBRARY_FILE: &str = "src/lib.leo";
static FILE_EXTENSION: &str = "leo";

fn parse_import_file(entry: &DirEntry, span: &Span) -> Result<Program, ImportError> {
    // make sure the given entry is file
    let file_type = entry
        .file_type()
        .map_err(|error| ImportError::directory_error(error, span.clone()))?;
    let file_name = entry
        .file_name()
        .into_string()
        .map_err(|_| ImportError::convert_os_string(span.clone()))?;

    let mut file_path = entry.path();
    if file_type.is_dir() {
        file_path.push(LIBRARY_FILE);

        if !file_path.exists() {
            return Err(ImportError::expected_lib_file(
                format!("{:?}", file_path.as_path()),
                span.clone(),
            ));
        }
    }

    // Build the abstract syntax tree
    let input_file = &LeoParser::load_file(&file_path)?;
    let syntax_tree = LeoParser::parse_file(&file_path, input_file)?;

    // Generate aleo program from file
    Ok(Program::from(syntax_tree, file_name.clone()))
}

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn enforce_import_star(&mut self, scope: String, entry: &DirEntry, span: Span) -> Result<(), ImportError> {
        let mut path = entry.path();
        let is_dir = path.is_dir();
        let is_leo_file = path
            .extension()
            .map_or(false, |ext| ext.eq(&OsString::from(FILE_EXTENSION)));

        path.push(LIBRARY_FILE);

        let is_package = is_dir && path.exists();

        // import * can only be invoked on a package with a library file or a leo file
        if is_package || is_leo_file {
            let mut program = parse_import_file(entry, &span)?;

            // use the same namespace as calling function for imported symbols
            program = program.name(scope);

            // * -> import all imports, circuits, functions in the current scope
            self.resolve_definitions(program)
        } else {
            // importing * from a directory or non-leo file in `package/src/` is illegal
            Err(ImportError::star(entry.path(), span))
        }
    }

    pub fn enforce_import_symbol(
        &mut self,
        scope: String,
        entry: &DirEntry,
        symbol: ImportSymbol,
    ) -> Result<(), ImportError> {
        // Generate aleo program from file
        let mut program = parse_import_file(entry, &symbol.span)?;

        // Use same namespace as calling function for imported symbols
        program = program.name(scope);

        let program_name = program.name.clone();

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
                    .find(|(function_name, _function)| symbol.symbol == *function_name);

                match matched_function {
                    Some((_function_name, function)) => ConstrainedValue::Function(None, function),
                    None => return Err(ImportError::unknown_symbol(symbol, program_name, &entry.path())),
                }
            }
        };

        // take the alias if it is present
        let name = symbol.alias.unwrap_or(symbol.symbol);
        let resolved_name = new_scope(program_name.clone(), name.to_string());

        // store imported circuit under resolved name
        self.store(resolved_name, value);

        // evaluate all import statements in imported file
        // todo: add logic to detect import loops
        program
            .imports
            .into_iter()
            .map(|nested_import| self.enforce_import(program_name.clone(), nested_import))
            .collect::<Result<Vec<_>, ImportError>>()?;

        Ok(())
    }
}
