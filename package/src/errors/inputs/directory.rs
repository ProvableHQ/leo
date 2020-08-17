use crate::{InputFileError, StateFileError};

use std::{ffi::OsString, fs::FileType, io};

#[derive(Debug, Error)]
pub enum InputsDirectoryError {
    #[error("creating: {}", _0)]
    Creating(io::Error),

    #[error("file entry getting: {}", _0)]
    GettingFileEntry(io::Error),

    #[error("file {:?} extension getting", _0)]
    GettingFileExtension(OsString),

    #[error("file {:?} name getting", _0)]
    GettingFileName(OsString),

    #[error("file {:?} type getting: {}", _0, _1)]
    GettingFileType(OsString, io::Error),

    #[error("{}", _0)]
    InputFileError(#[from] InputFileError),

    #[error("invalid file {:?} extension: {:?}", _0, _1)]
    InvalidFileExtension(String, OsString),

    #[error("invalid file {:?} type: {:?}", _0, _1)]
    InvalidFileType(OsString, FileType),

    #[error("reading: {}", _0)]
    Reading(io::Error),

    #[error("{}", _0)]
    StateFileError(#[from] StateFileError),
}
