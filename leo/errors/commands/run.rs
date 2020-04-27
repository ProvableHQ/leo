use crate::errors::ManifestError;

use std::ffi::OsString;

#[derive(Debug, Fail)]
pub enum RunError {

    #[fail(display = "main file {:?} does not exist", _0)]
    MainFileDoesNotExist(OsString),

    #[fail(display = "{}", _0)]
    ManifestError(ManifestError),

}

impl From<ManifestError> for RunError {
    fn from(error: ManifestError) -> Self {
        RunError::ManifestError(error)
    }
}
