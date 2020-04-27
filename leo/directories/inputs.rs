use crate::errors::InputsDirectoryError;

use std::fs;
use std::path::PathBuf;

pub(crate) static INPUTS_DIRECTORY_NAME: &str = "inputs/";

static INPUTS_FILE_EXTENSION: &str = "leo.in";

pub struct InputsDirectory;

impl InputsDirectory {
    /// Creates a directory at the provided path with the default directory name.
    pub fn create(path: &PathBuf) -> Result<(), InputsDirectoryError> {
        let mut path = path.to_owned();
        if path.is_dir() && !path.ends_with(INPUTS_DIRECTORY_NAME) {
            path.push(PathBuf::from(INPUTS_DIRECTORY_NAME));
        }

        fs::create_dir_all(&path).map_err(InputsDirectoryError::Creating)
    }

    /// Returns a list of files in the source directory.
    pub fn files(path: &PathBuf) -> Result<Vec<PathBuf>, InputsDirectoryError> {
        let mut path = path.to_owned();
        path.push(PathBuf::from(INPUTS_DIRECTORY_NAME));
        let directory = fs::read_dir(&path).map_err(InputsDirectoryError::Reading)?;

        let mut file_paths = Vec::new();
        for file_entry in directory.into_iter() {
            let file_entry = file_entry.map_err(InputsDirectoryError::GettingFileEntry)?;
            let file_path = file_entry.path();

            // Verify that the entry is structured as a valid file
            let file_type = file_entry.file_type().map_err(|error| {
                InputsDirectoryError::GettingFileType(file_path.as_os_str().to_owned(), error)
            })?;
            if !file_type.is_file() {
                return Err(InputsDirectoryError::InvalidFileType(
                    file_path.as_os_str().to_owned(),
                    file_type,
                ));
            }

            // Verify that the file has the default file extension
            let file_extension = file_path.extension().ok_or_else(|| {
                InputsDirectoryError::GettingFileExtension(file_path.as_os_str().to_owned())
            })?;
            if file_extension != INPUTS_FILE_EXTENSION {
                return Err(InputsDirectoryError::InvalidFileExtension(
                    file_path.as_os_str().to_owned(),
                    file_extension.to_owned(),
                ));
            }

            file_paths.push(file_path);
        }

        Ok(file_paths)
    }
}
