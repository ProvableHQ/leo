use std::{ffi::OsString, fs::FileType, io};

#[derive(Debug, Fail)]
pub enum OutputsDirectoryError {

    #[fail(display = "creating: {}", _0)]
    Creating(io::Error),

    #[fail(display = "file entry getting: {}", _0)]
    GettingFileEntry(io::Error),

    #[fail(display = "file {:?} extension getting", _0)]
    GettingFileExtension(OsString),

    #[fail(display = "file {:?} type getting: {}", _0, _1)]
    GettingFileType(OsString, io::Error),

    #[fail(display = "invalid file {:?} extension: {:?}", _0, _1)]
    InvalidFileExtension(OsString, OsString),

    #[fail(display = "invalid file {:?} type: {:?}", _0, _1)]
    InvalidFileType(OsString, FileType),

    #[fail(display = "reading: {}", _0)]
    Reading(io::Error),

    #[fail(display = "removing: {}", _0)]
    Removing(io::Error),

}
