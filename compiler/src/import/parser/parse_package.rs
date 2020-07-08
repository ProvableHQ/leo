use crate::{errors::constraints::ImportError, ImportParser};
use leo_types::{Package, PackageAccess};

use std::{fs, fs::DirEntry, path::PathBuf};

static SOURCE_FILE_EXTENSION: &str = ".leo";
static SOURCE_DIRECTORY_NAME: &str = "src/";
static IMPORTS_DIRECTORY_NAME: &str = "imports/";

impl ImportParser {
    pub fn parse_package_access(&mut self, entry: &DirEntry, access: &PackageAccess) -> Result<(), ImportError> {
        // bring one or more import symbols into scope for the current constrained program
        // we will recursively traverse sub packages here until we find the desired symbol
        match access {
            PackageAccess::Star(span) => self.parse_import_star(entry, span),
            PackageAccess::Symbol(symbol) => self.parse_import_symbol(entry, symbol),
            PackageAccess::SubPackage(package) => self.parse_package(entry.path(), package),
            PackageAccess::Multiple(accesses) => {
                for access in accesses {
                    self.parse_package_access(entry, access)?;
                }

                Ok(())
            }
        }
    }

    pub fn parse_package(&mut self, mut path: PathBuf, package: &Package) -> Result<(), ImportError> {
        let error_path = path.clone();
        let package_name = package.name.clone();

        // trim path if importing from another file
        if path.is_file() {
            path.pop();
        }

        // search for package name in local directory
        let mut source_directory = path.clone();
        source_directory.push(SOURCE_DIRECTORY_NAME);

        // search for package name in `imports` directory
        let mut imports_directory = path.clone();
        imports_directory.push(IMPORTS_DIRECTORY_NAME);

        // read from local `src` directory or the current path
        if source_directory.exists() {
            path = source_directory
        }

        let entries = fs::read_dir(path)
            .map_err(|error| ImportError::directory_error(error, package_name.span.clone(), error_path.clone()))?
            .into_iter()
            .collect::<Result<Vec<_>, std::io::Error>>()
            .map_err(|error| ImportError::directory_error(error, package_name.span.clone(), error_path.clone()))?;

        let matched_source_entry = entries.into_iter().find(|entry| {
            entry
                .file_name()
                .to_os_string()
                .into_string()
                .unwrap()
                .trim_end_matches(SOURCE_FILE_EXTENSION)
                .eq(&package_name.name)
        });

        if imports_directory.exists() {
            let entries = fs::read_dir(imports_directory)
                .map_err(|error| ImportError::directory_error(error, package_name.span.clone(), error_path.clone()))?
                .into_iter()
                .collect::<Result<Vec<_>, std::io::Error>>()
                .map_err(|error| ImportError::directory_error(error, package_name.span.clone(), error_path.clone()))?;

            let matched_import_entry = entries
                .into_iter()
                .find(|entry| entry.file_name().into_string().unwrap().eq(&package_name.name));

            match (matched_source_entry, matched_import_entry) {
                (Some(_), Some(_)) => Err(ImportError::conflicting_imports(package_name)),
                (Some(source_entry), None) => self.parse_package_access(&source_entry, &package.access),
                (None, Some(import_entry)) => self.parse_package_access(&import_entry, &package.access),
                (None, None) => Err(ImportError::unknown_package(package_name)),
            }
        } else {
            // Enforce local package access with no found imports directory
            match matched_source_entry {
                Some(source_entry) => self.parse_package_access(&source_entry, &package.access),
                None => Err(ImportError::unknown_package(package_name)),
            }
        }
    }
}
