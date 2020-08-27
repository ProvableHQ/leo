use std::io;

#[derive(Debug, Error)]
pub enum PackageError {
    #[error("{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[error("`{}` creating: {}", _0, _1)]
    Creating(&'static str, io::Error),

    #[error("`{}` metadata: {}", _0, _1)]
    Removing(&'static str, io::Error),
}

impl From<std::io::Error> for PackageError {
    fn from(error: std::io::Error) -> Self {
        PackageError::Crate("std::io", format!("{}", error))
    }
}

impl From<crate::errors::GitignoreError> for PackageError {
    fn from(error: crate::errors::GitignoreError) -> Self {
        PackageError::Crate("std::io", format!("{}", error))
    }
}

impl From<crate::errors::InputFileError> for PackageError {
    fn from(error: crate::errors::InputFileError) -> Self {
        PackageError::Crate("std::io", format!("{}", error))
    }
}

impl From<crate::errors::InputsDirectoryError> for PackageError {
    fn from(error: crate::errors::InputsDirectoryError) -> Self {
        PackageError::Crate("std::io", format!("{}", error))
    }
}

impl From<crate::errors::ImportsDirectoryError> for PackageError {
    fn from(error: crate::errors::ImportsDirectoryError) -> Self {
        PackageError::Crate("std::io", format!("{}", error))
    }
}

impl From<crate::errors::OutputsDirectoryError> for PackageError {
    fn from(error: crate::errors::OutputsDirectoryError) -> Self {
        PackageError::Crate("std::io", format!("{}", error))
    }
}

impl From<crate::errors::SourceDirectoryError> for PackageError {
    fn from(error: crate::errors::SourceDirectoryError) -> Self {
        PackageError::Crate("std::io", format!("{}", error))
    }
}

impl From<crate::errors::StateFileError> for PackageError {
    fn from(error: crate::errors::StateFileError) -> Self {
        PackageError::Crate("std::io", format!("{}", error))
    }
}

impl From<crate::errors::LibFileError> for PackageError {
    fn from(error: crate::errors::LibFileError) -> Self {
        PackageError::Crate("std::io", format!("{}", error))
    }
}

impl From<crate::errors::ManifestError> for PackageError {
    fn from(error: crate::errors::ManifestError) -> Self {
        PackageError::Crate("std::io", format!("{}", error))
    }
}

impl From<crate::errors::MainFileError> for PackageError {
    fn from(error: crate::errors::MainFileError) -> Self {
        PackageError::Crate("std::io", format!("{}", error))
    }
}
