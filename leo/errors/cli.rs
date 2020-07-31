use crate::errors::*;

#[derive(Debug, Error)]
pub enum CLIError {
    #[error("{}", _0)]
    BuildError(#[from] BuildError),

    #[error("{}", _0)]
    BytesFileError(#[from] ZipFileError),

    #[error("{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[error("{}", _0)]
    ChecksumFileError(#[from] ChecksumFileError),

    #[error("{}", _0)]
    GitignoreError(#[from] GitignoreError),

    #[error("{}", _0)]
    InitError(#[from] InitError),

    #[error("{}", _0)]
    InputsDirectoryError(#[from] InputsDirectoryError),

    #[error("{}", _0)]
    InputsFileError(#[from] InputsFileError),

    #[error("{}", _0)]
    LibFileError(#[from] LibFileError),

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
    SNARKError(#[from] snarkos_errors::algorithms::snark::SNARKError),

    #[error("{}", _0)]
    SourceDirectoryError(#[from] SourceDirectoryError),

    #[error("{}", _0)]
    TestError(#[from] TestError),

    #[error("{}", _0)]
    VerificationKeyFileError(#[from] VerificationKeyFileError),
}

impl From<leo_compiler::errors::CompilerError> for CLIError {
    fn from(error: leo_compiler::errors::CompilerError) -> Self {
        log::error!("{}\n", error);
        CLIError::Crate("leo_compiler", "Program failed due to previous error".into())
    }
}

impl From<leo_inputs::errors::InputParserError> for CLIError {
    fn from(error: leo_inputs::errors::InputParserError) -> Self {
        log::error!("{}\n", error);
        CLIError::Crate("leo_inputs", "Program failed due to previous error".into())
    }
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
