use crate::errors::ManifestError;

use std::{ffi::OsString, io};

#[derive(Debug, Error)]
pub enum NewError {
    #[error("root directory {:?} creating: {}", _0, _1)]
    CreatingRootDirectory(OsString, io::Error),

    #[error("directory {:?} already exists", _0)]
    DirectoryAlreadyExists(OsString),

    #[error("{}", _0)]
    ManifestError(#[from] ManifestError),

    #[error("package at path {:?} already exists", _0)]
    PackageAlreadyExists(OsString),

    #[error("package name is missing - {:?}", _0)]
    ProjectNameInvalid(OsString),
}
