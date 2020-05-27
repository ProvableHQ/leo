use crate::errors::*;
use leo_compiler::errors::CompilerError;

#[derive(Debug, Error)]
pub enum CLIError {
    #[error("{}", _0)]
    BuildError(#[from] BuildError),

    #[error("{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[error("{}", _0)]
    ChecksumFileError(#[from] ChecksumFileError),

    #[error("{}", _0)]
    CompilerError(#[from] CompilerError),

    #[error("{}", _0)]
    GitignoreError(#[from] GitignoreError),

    #[error("{}", _0)]
    InitError(#[from] InitError),

    #[error("{}", _0)]
    InputsDirectoryError(#[from] InputsDirectoryError),

    #[error("{}", _0)]
    MainFileError(#[from] MainFileError),

    #[error("{}", _0)]
    ManifestError(#[from] ManifestError),

    #[error("{}", _0)]
    NewError(#[from] NewError),

    #[error("{}", _0)]
    OutputsDirectoryError(#[from] OutputsDirectoryError),

    #[error("{}", _0)]
    ProofFileError(#[from] ProofFileError),

    #[error("{}", _0)]
    ProvingKeyFileError(#[from] ProvingKeyFileError),

    #[error("{}", _0)]
    RunError(#[from] RunError),

    #[error("{}", _0)]
    SourceDirectoryError(#[from] SourceDirectoryError),

    #[error("{}", _0)]
    TestError(#[from] TestError),

    #[error("{}", _0)]
    VerificationKeyFileError(#[from] VerificationKeyFileError),
}

impl From<snarkos_errors::gadgets::SynthesisError> for CLIError {
    fn from(error: snarkos_errors::gadgets::SynthesisError) -> Self {
        CLIError::Crate("snarkos_errors", format!("{}", error))
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
