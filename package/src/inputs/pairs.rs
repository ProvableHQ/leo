// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use crate::inputs::{InputFile, InputsDirectory, StateFile, INPUT_FILE_EXTENSION, STATE_FILE_EXTENSION};
use leo_errors::{LeoError, PackageError};

use std::{collections::HashMap, convert::TryFrom, path::Path};

use backtrace::Backtrace;

#[derive(Default)]
pub struct InputPairs {
    /// Maps file names to input file pairs
    pub pairs: HashMap<String, InputPair>,
}

#[derive(Debug)]
pub struct InputPair {
    pub input_file: String,
    pub state_file: String,
}

impl InputPairs {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TryFrom<&Path> for InputPairs {
    type Error = LeoError;

    fn try_from(directory: &Path) -> Result<Self, Self::Error> {
        let files = InputsDirectory::files(directory)?;

        let mut pairs = HashMap::<String, InputPair>::new();

        for file in files {
            // if file name starts with . (dot) None is returned - we're
            // skipping these files intentionally but not exiting
            let file_extension = match file.extension() {
                Some(extension) => extension,
                None => continue,
            };

            // Have to handle error mapping this way because of rust error: https://github.com/rust-lang/rust/issues/42424.
            let file_name = match file.file_stem() {
                Some(stem) => match stem.to_str() {
                    Some(file_name) => file_name,
                    None => {
                        return Err(
                            PackageError::failed_to_get_input_file_name(file.as_os_str(), Backtrace::new()).into(),
                        );
                    }
                },
                None => {
                    return Err(PackageError::failed_to_get_input_file_name(file.as_os_str(), Backtrace::new()).into());
                }
            };

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
                // kept for verbosity, can be removed
                continue;
            }
        }

        Ok(InputPairs { pairs })
    }
}
