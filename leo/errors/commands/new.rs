use crate::errors::ManifestError;

use std::ffi::OsString;
use std::io;

#[derive(Debug, Fail)]
pub enum NewError {

    #[fail(display = "root directory {:?} creating: {}", _0, _1)]
    CreatingRootDirectory(OsString, io::Error),

    #[fail(display = "directory {:?} already exists", _0)]
    DirectoryAlreadyExists(OsString),

    #[fail(display = "{}", _0)]
    ManifestError(ManifestError),

    #[fail(display = "package name is missing - {:?}", _0)]
    ProjectNameInvalid(OsString),

}

impl From<ManifestError> for NewError {
    fn from(error: ManifestError) -> Self {
        NewError::ManifestError(error)
    }
}
