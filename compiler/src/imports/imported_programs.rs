use crate::errors::ImportError;
use leo_types::Program;

use std::{collections::HashMap, env::current_dir};

#[derive(Clone)]
pub struct ImportedPrograms {
    imports: HashMap<String, Program>,
}

impl ImportedPrograms {
    pub fn new() -> Self {
        Self {
            imports: HashMap::new(),
        }
    }

    pub(crate) fn store(&mut self, name: String, program: Program) {
        // todo: handle conflicting versions for duplicate imports here
        println!("{}, {:?}", name, program);
        let _res = self.imports.insert(name, program);
    }

    pub fn get(&self, name: &String) -> Option<&Program> {
        self.imports.get(name)
    }

    pub fn from_program(program: &Program) -> Result<Self, ImportError> {
        let mut imports = Self::new();

        // Find all imports relative to current directory
        let path = current_dir().map_err(|error| ImportError::current_directory_error(error))?;

        // Parse each imported file
        program
            .imports
            .iter()
            .map(|import| imports.parse_package(path.clone(), &import.package))
            .collect::<Result<Vec<()>, ImportError>>()?;

        Ok(imports)
    }
}
