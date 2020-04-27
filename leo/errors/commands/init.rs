use crate::errors::ManifestError;

use std::ffi::OsString;
use std::io;

#[derive(Debug, Fail)]
pub enum InitError {
    #[fail(display = "root directory {:?} creating: {}", _0, _1)]
    CreatingRootDirectory(OsString, io::Error),

    #[fail(display = "directory {:?} does not exist", _0)]
    DirectoryDoesNotExist(OsString),

    #[fail(display = "{}", _0)]
    ManifestError(ManifestError),

    #[fail(display = "package at path {:?} already exists", _0)]
    PackageAlreadyExists(OsString),

    #[fail(display = "package name is missing - {:?}", _0)]
    ProjectNameInvalid(OsString),
}

impl From<ManifestError> for InitError {
    fn from(error: ManifestError) -> Self {
        InitError::ManifestError(error)
    }
}
