use crate::{constraints::ConstrainedProgram, errors::constraints::ImportError, GroupType};
use leo_types::{Package, PackageAccess};

use snarkos_models::curves::{Field, PrimeField};
use std::{fs, fs::DirEntry, path::PathBuf};

static SOURCE_FILE_EXTENSION: &str = ".leo";
static SOURCE_DIRECTORY_NAME: &str = "src/";
// pub(crate) static IMPORTS_DIRECTORY_NAME: &str = "imports/";

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn enforce_package_access(
        &mut self,
        scope: String,
        entry: &DirEntry,
        access: PackageAccess,
    ) -> Result<(), ImportError> {
        // bring one or more import symbols into scope for the current constrained program
        // we will recursively traverse sub packages here until we find the desired symbol
        match access {
            PackageAccess::Star(span) => self.enforce_import_star(scope, entry, span),
            PackageAccess::Symbol(symbol) => self.enforce_import_symbol(scope, entry, symbol),
            PackageAccess::SubPackage(package) => self.enforce_package(scope, entry.path(), *package),
            PackageAccess::Multiple(accesses) => {
                for access in accesses {
                    self.enforce_package_access(scope.clone(), entry, access)?;
                }

                Ok(())
            }
        }
    }

    pub fn enforce_package(&mut self, scope: String, path: PathBuf, package: Package) -> Result<(), ImportError> {
        let package_name = package.name;

        // search for package name in local src directory
        let mut source_directory = path.clone();
        source_directory.push(SOURCE_DIRECTORY_NAME);

        let entries = fs::read_dir(source_directory)
            .map_err(|error| ImportError::directory_error(error, package_name.span.clone()))?
            .into_iter()
            .collect::<Result<Vec<_>, std::io::Error>>()
            .map_err(|error| ImportError::directory_error(error, package_name.span.clone()))?;

        let matched_source_entry = entries.into_iter().find(|entry| {
            entry
                .file_name()
                .into_string()
                .unwrap()
                .trim_end_matches(SOURCE_FILE_EXTENSION)
                .eq(&package_name.name)
        });

        // search for package name in imports directory
        // let mut source_directory = path.clone();
        // source_directory.push(IMPORTS_DIRECTORY_NAME);
        //
        // let entries = fs::read_dir(source_directory)
        //     .map_err(|error| ImportError::directory_error(error, package_name.span.clone()))?
        //     .into_iter()
        //     .collect::<Result<Vec<_>, std::io::Error>>()
        //     .map_err(|error| ImportError::directory_error(error, package_name.span.clone()))?;
        //
        // let matched_import_entry = entries.into_iter().find(|entry| {
        //     entry.file_name().eq(&package_name.name)
        // });

        // todo: return error if package name is present in both directories

        // Enforce package access
        if let Some(entry) = matched_source_entry {
            self.enforce_package_access(scope, &entry, package.access)
        } else {
            Err(ImportError::unknown_package(package_name))
        }
    }
}
