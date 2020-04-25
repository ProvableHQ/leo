use crate::errors::{InitError, InputsDirectoryError, MainFileError, ManifestError, OutputsDirectoryError, RunError, SourceDirectoryError};

#[derive(Debug, Fail)]
pub enum CLIError {

    #[fail(display = "{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[fail(display = "{}", _0)]
    InitError(InitError),

    #[fail(display = "{}", _0)]
    InputsDirectoryError(InputsDirectoryError),

    #[fail(display = "{}", _0)]
    MainFileError(MainFileError),

    #[fail(display = "{}", _0)]
    ManifestError(ManifestError),

    #[fail(display = "{}", _0)]
    OutputsDirectoryError(OutputsDirectoryError),

    #[fail(display = "{}", _0)]
    RunError(RunError),

    #[fail(display = "{}", _0)]
    SourceDirectoryError(SourceDirectoryError),

}

impl From<InitError> for CLIError {
    fn from(error: InitError) -> Self {
        CLIError::InitError(error)
    }
}

impl From<InputsDirectoryError> for CLIError {
    fn from(error: InputsDirectoryError) -> Self {
        CLIError::InputsDirectoryError(error)
    }
}

impl From<MainFileError> for CLIError {
    fn from(error: MainFileError) -> Self {
        CLIError::MainFileError(error)
    }
}

impl From<ManifestError> for CLIError {
    fn from(error: ManifestError) -> Self {
        CLIError::ManifestError(error)
    }
}

impl From<OutputsDirectoryError> for CLIError {
    fn from(error: OutputsDirectoryError) -> Self {
        CLIError::OutputsDirectoryError(error)
    }
}

impl From<RunError> for CLIError {
    fn from(error: RunError) -> Self {
        CLIError::RunError(error)
    }
}

impl From<SourceDirectoryError> for CLIError {
    fn from(error: SourceDirectoryError) -> Self {
        CLIError::SourceDirectoryError(error)
    }
}

impl From<serde_json::error::Error> for CLIError {
    fn from(error: serde_json::error::Error) -> Self {
        CLIError::Crate("serde_json", format!("{}", error))
    }
}

impl From<std::io::Error> for CLIError {
    fn from(error: std::io::Error) -> Self {
        CLIError::Crate("std::io", format!("{}", error))
    }
}
