use crate::errors::{ManifestError, NewError};

#[derive(Debug, Fail)]
pub enum CLIError {
    #[fail(display = "{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[fail(display = "{}", _0)]
    ManifestError(ManifestError),

    #[fail(display = "{}", _0)]
    NewError(NewError),
}

impl From<ManifestError> for CLIError {
    fn from(error: ManifestError) -> Self {
        CLIError::ManifestError(error)
    }
}

impl From<NewError> for CLIError {
    fn from(error: NewError) -> Self {
        CLIError::NewError(error)
    }
}

impl From<serde_json::error::Error> for CLIError {
    fn from(error: serde_json::error::Error) -> Self {
        CLIError::Crate("serde_json", format!("{:?}", error))
    }
}
