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

use crate::{InputFileError, StateFileError};

use std::{ffi::OsString, fs::FileType, io};

#[derive(Debug, Error)]
pub enum InputsDirectoryError {
    #[error("creating: {}", _0)]
    Creating(io::Error),

    #[error("file entry getting: {}", _0)]
    GettingFileEntry(io::Error),

    #[error("file {:?} name getting", _0)]
    GettingFileName(OsString),

    #[error("file {:?} type getting: {}", _0, _1)]
    GettingFileType(OsString, io::Error),

    #[error("{}", _0)]
    InputFileError(#[from] InputFileError),

    #[error("invalid file {:?} type: {:?}", _0, _1)]
    InvalidFileType(OsString, FileType),

    #[error("reading: {}", _0)]
    Reading(io::Error),

    #[error("{}", _0)]
    StateFileError(#[from] StateFileError),
}
