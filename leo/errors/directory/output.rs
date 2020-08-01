use std::{ffi::OsString, fs::FileType, io};

#[derive(Debug, Error)]
pub enum OutputDirectoryError {
    #[error("creating: {}", _0)]
    Creating(io::Error),

    #[error("file entry getting: {}", _0)]
    GettingFileEntry(io::Error),

    #[error("file {:?} extension getting", _0)]
    GettingFileExtension(OsString),

    #[error("file {:?} type getting: {}", _0, _1)]
    GettingFileType(OsString, io::Error),

    #[error("invalid file {:?} extension: {:?}", _0, _1)]
    InvalidFileExtension(OsString, OsString),

    #[error("invalid file {:?} type: {:?}", _0, _1)]
    InvalidFileType(OsString, FileType),

    #[error("reading: {}", _0)]
    Reading(io::Error),

    #[error("removing: {}", _0)]
    Removing(io::Error),
}
