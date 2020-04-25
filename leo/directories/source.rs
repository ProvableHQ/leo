use crate::errors::SourceDirectoryError;

use std::fs;
use std::path::PathBuf;

pub(crate) static DIRECTORY_NAME_DEFAULT: &str = "src/";

static SOURCE_FILE_EXTENSION_DEFAULT: &str = "leo";

pub struct SourceDirectory;

impl SourceDirectory {
    /// Creates a directory at the provided path with the default directory name.
    pub fn create(path: &PathBuf) -> Result<(), SourceDirectoryError> {
        let mut path = path.to_owned();
        if path.is_dir() && !path.ends_with(DIRECTORY_NAME_DEFAULT) {
            path.push(PathBuf::from(DIRECTORY_NAME_DEFAULT));
        }

        fs::create_dir_all(&path).map_err(SourceDirectoryError::Creating)
    }

    /// Returns a list of files in the source directory.
    pub fn files(path: &PathBuf) -> Result<Vec<PathBuf>, SourceDirectoryError> {
        let mut path = path.to_owned();
        path.push(PathBuf::from(DIRECTORY_NAME_DEFAULT));
        let directory = fs::read_dir(&path).map_err(SourceDirectoryError::Reading)?;

        let mut file_paths = Vec::new();
        for file_entry in directory.into_iter() {
            let file_entry = file_entry.map_err(SourceDirectoryError::GettingFileEntry)?;
            let file_path = file_entry.path();

            // Verify that the entry is structured as a valid file
            let file_type = file_entry
                .file_type()
                .map_err(|error| SourceDirectoryError::GettingFileType(file_path.as_os_str().to_owned(), error))?;
            if !file_type.is_file() {
                return Err(SourceDirectoryError::InvalidFileType(
                    file_path.as_os_str().to_owned(),
                    file_type,
                ));
            }

            // Verify that the file has the default file extension
            let file_extension = file_path
                .extension()
                .ok_or_else(|| SourceDirectoryError::GettingFileExtension(file_path.as_os_str().to_owned()))?;
            if file_extension != SOURCE_FILE_EXTENSION_DEFAULT {
                return Err(SourceDirectoryError::InvalidFileExtension(
                    file_path.as_os_str().to_owned(),
                    file_extension.to_owned(),
                ));
            }

            file_paths.push(file_path);
        }

        Ok(file_paths)
    }
}
