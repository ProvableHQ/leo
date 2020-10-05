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

use crate::{
    inputs::{InputFile, InputsDirectory, StateFile, INPUT_FILE_EXTENSION, STATE_FILE_EXTENSION},
    InputsDirectoryError,
};

use std::{collections::HashMap, convert::TryFrom, path::PathBuf};

#[derive(Default)]
pub struct InputPairs {
    /// Maps file names to input file pairs
    pub pairs: HashMap<String, InputPair>,
}

pub struct InputPair {
    pub input_file: String,
    pub state_file: String,
}

impl InputPairs {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TryFrom<&PathBuf> for InputPairs {
    type Error = InputsDirectoryError;

    fn try_from(directory: &PathBuf) -> Result<Self, Self::Error> {
        let files = InputsDirectory::files(directory)?;

        let mut pairs = HashMap::<String, InputPair>::new();

        for file in files {
            let file_extension = file
                .extension()
                .ok_or_else(|| InputsDirectoryError::GettingFileExtension(file.as_os_str().to_owned()))?;

            let file_name = file
                .file_stem()
                .ok_or_else(|| InputsDirectoryError::GettingFileName(file.as_os_str().to_owned()))?
                .to_str()
                .ok_or_else(|| InputsDirectoryError::GettingFileName(file.as_os_str().to_owned()))?;

            if file_extension == INPUT_FILE_EXTENSION.trim_start_matches('.') {
                let input_file = InputFile::new(file_name).read_from(&file)?.0;

                if pairs.contains_key(file_name) {
                    let pair = pairs.get_mut(file_name).unwrap();
                    pair.input_file = input_file;
                } else {
                    let pair = InputPair {
                        input_file,
                        state_file: "".to_owned(),
                    };
                    pairs.insert(file_name.to_owned(), pair);
                }
            } else if file_extension == STATE_FILE_EXTENSION.trim_start_matches('.') {
                let state_file = StateFile::new(file_name).read_from(&file)?.0;

                if pairs.contains_key(file_name) {
                    let pair = pairs.get_mut(file_name).unwrap();
                    pair.state_file = state_file;
                } else {
                    let pair = InputPair {
                        input_file: "".to_owned(),
                        state_file,
                    };
                    pairs.insert(file_name.to_owned(), pair);
                }
            } else {
                return Err(InputsDirectoryError::InvalidFileExtension(
                    file_name.to_owned(),
                    file_extension.to_owned(),
                ));
            }
        }

        Ok(InputPairs { pairs })
    }
}
