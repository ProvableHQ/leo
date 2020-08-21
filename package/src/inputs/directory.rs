// Copyright (C) 2019-2020 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::errors::InputsDirectoryError;

use std::{fs, fs::ReadDir, path::PathBuf};

pub static INPUTS_DIRECTORY_NAME: &str = "inputs/";

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

    /// Returns a list of files in the input directory.
    pub fn files(path: &PathBuf) -> Result<Vec<PathBuf>, InputsDirectoryError> {
        let mut path = path.to_owned();
        path.push(PathBuf::from(INPUTS_DIRECTORY_NAME));
        let directory = fs::read_dir(&path).map_err(InputsDirectoryError::Reading)?;

        let mut file_paths = Vec::new();
        parse_file_paths(directory, &mut file_paths)?;

        Ok(file_paths)
    }

    /// Remove all inputs files in the input directory
    pub fn remove_files(path: &PathBuf) -> Result<(), InputsDirectoryError> {
        let files = InputsDirectory::files(path)?;
        files.iter().for_each(|file| match std::fs::remove_file(file) {
            Ok(_) => log::info!("File {:?} removed", file),
            Err(_) => log::warn!("Cannot remove {:?} file, please check permitions", file),
        });
        Ok(())
    }

    /// Remove Leo inputs directory if it is empty and permissions allowed
    pub fn remove_dir(path: &PathBuf) -> Result<(), InputsDirectoryError> {
        let mut path = path.to_owned();
        if path.is_dir() && !path.ends_with(INPUTS_DIRECTORY_NAME) {
            path.push(PathBuf::from(INPUTS_DIRECTORY_NAME));
        }
        match std::fs::remove_dir(path.clone()) {
            Ok(_) => log::info!("Directory {:?} removed", path),
            Err(_error) => log::warn!("Cannot remove {:?} directory", path),
        }
        Ok(())
    }
}

fn parse_file_paths(directory: ReadDir, file_paths: &mut Vec<PathBuf>) -> Result<(), InputsDirectoryError> {
    for file_entry in directory.into_iter() {
        let file_entry = file_entry.map_err(InputsDirectoryError::GettingFileEntry)?;
        let file_path = file_entry.path();

        // Verify that the entry is structured as a valid file or directory
        let file_type = file_entry
            .file_type()
            .map_err(|error| InputsDirectoryError::GettingFileType(file_path.as_os_str().to_owned(), error))?;
        if file_type.is_dir() {
            let directory = fs::read_dir(&file_path).map_err(InputsDirectoryError::Reading)?;

            parse_file_paths(directory, file_paths)?;
            continue;
        } else if !file_type.is_file() {
            return Err(InputsDirectoryError::InvalidFileType(
                file_path.as_os_str().to_owned(),
                file_type,
            ));
        }

        file_paths.push(file_path);
    }

    Ok(())
}
