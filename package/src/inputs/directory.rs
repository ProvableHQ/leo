use crate::{errors::InputDirectoryError, inputs::INPUT_FILE_EXTENSION};

use std::{fs, path::PathBuf};

pub static INPUT_DIRECTORY_NAME: &str = "inputs/";

pub struct InputDirectory;

impl InputDirectory {
    /// Creates a directory at the provided path with the default directory name.
    pub fn create(path: &PathBuf) -> Result<(), InputDirectoryError> {
        let mut path = path.to_owned();
        if path.is_dir() && !path.ends_with(INPUT_DIRECTORY_NAME) {
            path.push(PathBuf::from(INPUT_DIRECTORY_NAME));
        }

        fs::create_dir_all(&path).map_err(InputDirectoryError::Creating)
    }

    /// Returns a list of files in the source directory.
    pub fn files(path: &PathBuf) -> Result<Vec<PathBuf>, InputDirectoryError> {
        let mut path = path.to_owned();
        path.push(PathBuf::from(INPUT_DIRECTORY_NAME));
        let directory = fs::read_dir(&path).map_err(InputDirectoryError::Reading)?;

        let mut file_paths = Vec::new();
        for file_entry in directory.into_iter() {
            let file_entry = file_entry.map_err(InputDirectoryError::GettingFileEntry)?;
            let file_path = file_entry.path();

            // Verify that the entry is structured as a valid file
            let file_type = file_entry
                .file_type()
                .map_err(|error| InputDirectoryError::GettingFileType(file_path.as_os_str().to_owned(), error))?;
            if !file_type.is_file() {
                return Err(InputDirectoryError::InvalidFileType(
                    file_path.as_os_str().to_owned(),
                    file_type,
                ));
            }

            // Verify that the file has the default file extension
            let file_extension = file_path
                .extension()
                .ok_or_else(|| InputDirectoryError::GettingFileExtension(file_path.as_os_str().to_owned()))?;
            if file_extension != INPUT_FILE_EXTENSION {
                return Err(InputDirectoryError::InvalidFileExtension(
                    file_path.as_os_str().to_owned(),
                    file_extension.to_owned(),
                ));
            }

            file_paths.push(file_path);
        }

        Ok(file_paths)
    }
}
