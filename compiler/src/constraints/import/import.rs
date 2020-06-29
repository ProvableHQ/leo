use crate::{constraints::ConstrainedProgram, errors::constraints::ImportError, GroupType};
use leo_types::Import;

use snarkos_models::curves::{Field, PrimeField};
use std::env::current_dir;

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn enforce_import(&mut self, scope: String, import: Import) -> Result<(), ImportError> {
        let path = current_dir().map_err(|error| ImportError::directory_error(error, import.span.clone()))?;

        self.enforce_package(scope, path, import.package)
    }
}
