use std::{io, path::PathBuf};
use walkdir::Error as WalkDirError;
use zip::result::ZipError;

#[derive(Debug, Error)]
pub enum ZipFileError {
    #[error("{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[error("creating: {}", _0)]
    Creating(io::Error),

    #[error("Cannot read from the provided file path - {:?}", _0)]
    FileReadError(PathBuf),

    #[error("writing: {}", _0)]
    Writing(io::Error),

    #[error("{}", _0)]
    WalkDirError(#[from] WalkDirError),

    #[error("{}", _0)]
    ZipError(#[from] ZipError),
}

impl From<std::io::Error> for ZipFileError {
    fn from(error: std::io::Error) -> Self {
        ZipFileError::Crate("std::io", format!("{}", error))
    }
}
