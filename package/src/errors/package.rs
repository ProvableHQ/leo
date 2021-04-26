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
use std::ffi::OsString;

#[derive(Debug, Error)]
pub enum PackageError {
    #[error("{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[error("Failed to initialize package {:?} ({:?})", _0, _1)]
    FailedToInitialize(String, OsString),

    #[error("Invalid project name: {:?}", _0)]
    InvalidPackageName(String),
}

impl From<std::io::Error> for PackageError {
    fn from(error: std::io::Error) -> Self {
        PackageError::Crate("std::io", error.to_string())
    }
}

impl From<crate::errors::GitignoreError> for PackageError {
    fn from(error: crate::errors::GitignoreError) -> Self {
        PackageError::Crate("leo-package", error.to_string())
    }
}

impl From<crate::errors::InputFileError> for PackageError {
    fn from(error: crate::errors::InputFileError) -> Self {
        PackageError::Crate("leo-package", error.to_string())
    }
}

impl From<crate::errors::InputsDirectoryError> for PackageError {
    fn from(error: crate::errors::InputsDirectoryError) -> Self {
        PackageError::Crate("leo-package", error.to_string())
    }
}

impl From<crate::errors::ImportsDirectoryError> for PackageError {
    fn from(error: crate::errors::ImportsDirectoryError) -> Self {
        PackageError::Crate("leo-package", error.to_string())
    }
}

impl From<crate::errors::OutputsDirectoryError> for PackageError {
    fn from(error: crate::errors::OutputsDirectoryError) -> Self {
        PackageError::Crate("leo-package", error.to_string())
    }
}

impl From<crate::errors::READMEError> for PackageError {
    fn from(error: crate::errors::READMEError) -> Self {
        PackageError::Crate("leo-package", error.to_string())
    }
}

impl From<crate::errors::SourceDirectoryError> for PackageError {
    fn from(error: crate::errors::SourceDirectoryError) -> Self {
        PackageError::Crate("leo-package", error.to_string())
    }
}

impl From<crate::errors::StateFileError> for PackageError {
    fn from(error: crate::errors::StateFileError) -> Self {
        PackageError::Crate("leo-package", error.to_string())
    }
}

impl From<crate::errors::LibraryFileError> for PackageError {
    fn from(error: crate::errors::LibraryFileError) -> Self {
        PackageError::Crate("leo-package", error.to_string())
    }
}

impl From<crate::errors::ManifestError> for PackageError {
    fn from(error: crate::errors::ManifestError) -> Self {
        PackageError::Crate("leo-package", error.to_string())
    }
}

impl From<crate::errors::MainFileError> for PackageError {
    fn from(error: crate::errors::MainFileError) -> Self {
        PackageError::Crate("leo-package", error.to_string())
    }
}
