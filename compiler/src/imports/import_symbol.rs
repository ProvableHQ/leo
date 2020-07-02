use crate::{errors::constraints::ImportError, ProgramImports};
use leo_ast::LeoParser;
use leo_types::{ImportSymbol, Program, Span};

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
        .to_os_string()
        .into_string()
        .map_err(|_| ImportError::convert_os_string(span.clone()))?;

    let mut file_path = entry.path().to_path_buf();
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

impl ProgramImports {
    pub fn parse_import_star(&mut self, entry: &DirEntry, span: &Span) -> Result<(), ImportError> {
        let path = entry.path();
        let is_dir = path.is_dir();
        let is_leo_file = path
            .extension()
            .map_or(false, |ext| ext.eq(&OsString::from(FILE_EXTENSION)));

        let mut package_path = path.to_path_buf();
        package_path.push(LIBRARY_FILE);

        let is_package = is_dir && package_path.exists();

        // import * can only be invoked on a package with a library file or a leo file
        if is_package || is_leo_file {
            // Generate aleo program from file
            let name = format!("{:?}", entry.path());
            let program = parse_import_file(entry, &span)?;

            // Store program in imports hashmap
            self.store(name, program);

            Ok(())
        } else {
            // importing * from a directory or non-leo file in `package/src/` is illegal
            Err(ImportError::star(entry.path().to_path_buf(), span.clone()))
        }
    }

    pub fn parse_import_symbol(&mut self, entry: &DirEntry, symbol: &ImportSymbol) -> Result<(), ImportError> {
        // Generate aleo program from file
        let name = format!("{:?}", entry.path());
        let program = parse_import_file(entry, &symbol.span)?;

        // Store program in imports hashmap
        self.store(name, program);

        Ok(())
    }
}
