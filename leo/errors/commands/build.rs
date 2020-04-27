use crate::errors::ManifestError;

use std::ffi::OsString;

#[derive(Debug, Fail)]
pub enum BuildError {
    #[fail(display = "main file {:?} does not exist", _0)]
    MainFileDoesNotExist(OsString),

    #[fail(display = "{}", _0)]
    ManifestError(ManifestError),
}

impl From<ManifestError> for BuildError {
    fn from(error: ManifestError) -> Self {
        BuildError::ManifestError(error)
    }
}
