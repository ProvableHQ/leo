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

use crate::errors::*;
use leo_compiler::errors::OutputFileError;
use leo_package::errors::*;

#[derive(Debug, Error)]
pub enum CLIError {
    #[error("{}", _0)]
    AddError(AddError),

    #[error("{}", _0)]
    BuildError(BuildError),

    #[error("{}", _0)]
    ZipFileError(ZipFileError),

    #[error("{}", _0)]
    ChecksumFileError(ChecksumFileError),

    #[error("{}", _0)]
    CircuitFileError(CircuitFileError),

    #[error("{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[error("{}", _0)]
    GitignoreError(GitignoreError),

    #[error("{}", _0)]
    InitError(InitError),

    #[error("{}", _0)]
    ImportsDirectoryError(ImportsDirectoryError),

    #[error("{}", _0)]
    InputsDirectoryError(InputsDirectoryError),

    #[error("{}", _0)]
    InputFileError(InputFileError),

    #[error("{}", _0)]
    LibraryFileError(LibraryFileError),

    #[error("{}", _0)]
    LoginError(LoginError),

    #[error("{}", _0)]
    MainFileError(MainFileError),

    #[error("{}", _0)]
    ManifestError(ManifestError),

    #[error("{}", _0)]
    NewError(NewError),

    #[error("{}", _0)]
    OutputFileError(OutputFileError),

    #[error("{}", _0)]
    OutputsDirectoryError(OutputsDirectoryError),

    #[error("{}", _0)]
    PackageError(PackageError),

    #[error("{}", _0)]
    ProofFileError(ProofFileError),

    #[error("{}", _0)]
    ProvingKeyFileError(ProvingKeyFileError),

    #[error("{}", _0)]
    PublishError(PublishError),

    #[error("{}", _0)]
    READMEError(READMEError),

    #[error("{}", _0)]
    RunError(RunError),

    #[error("{}", _0)]
    SNARKError(snarkvm_errors::algorithms::snark::SNARKError),

    #[error("{}", _0)]
    SourceDirectoryError(SourceDirectoryError),

    #[error("{}", _0)]
    StateFileError(StateFileError),

    #[error("{}", _0)]
    TestError(TestError),

    #[error("TomlSerError: {0}")]
    TomlSerError(#[from] toml::ser::Error),

    #[error("TomlDeError: {0}")]
    TomlDeError(#[from] toml::de::Error),

    #[error("{}", _0)]
    VerificationKeyFileError(VerificationKeyFileError),
}

macro_rules! impl_cli_error {
    ($($t:tt), +) => {
        $(impl From<$t> for CLIError {
            fn from(error: $t) -> Self {
                tracing::error!("{}\n", error);

                CLIError::$t(error)
            }
        })*
    }
}

impl_cli_error!(
    AddError,
    BuildError,
    CircuitFileError,
    ChecksumFileError,
    GitignoreError,
    ImportsDirectoryError,
    InitError,
    InputsDirectoryError,
    InputFileError,
    LibraryFileError,
    LoginError,
    MainFileError,
    ManifestError,
    NewError,
    OutputFileError,
    OutputsDirectoryError,
    PackageError,
    ProofFileError,
    ProvingKeyFileError,
    PublishError,
    READMEError,
    RunError,
    SourceDirectoryError,
    StateFileError,
    TestError,
    VerificationKeyFileError,
    ZipFileError
);

impl From<clap::Error> for CLIError {
    fn from(error: clap::Error) -> Self {
        tracing::error!("{}\n", error);
        CLIError::Crate("clap", error.to_string())
    }
}

impl From<leo_compiler::errors::CompilerError> for CLIError {
    fn from(error: leo_compiler::errors::CompilerError) -> Self {
        tracing::error!("{}\n", error);
        CLIError::Crate("leo-compiler", "Program failed due to previous error".into())
    }
}

impl From<leo_input::errors::InputParserError> for CLIError {
    fn from(error: leo_input::errors::InputParserError) -> Self {
        tracing::error!("{}\n", error);
        CLIError::Crate("leo-input", "Program failed due to previous error".into())
    }
}

impl From<reqwest::Error> for CLIError {
    fn from(error: reqwest::Error) -> Self {
        tracing::error!("{}\n", error);
        CLIError::Crate("rewquest", error.to_string())
    }
}

impl From<snarkvm_errors::algorithms::snark::SNARKError> for CLIError {
    fn from(error: snarkvm_errors::algorithms::snark::SNARKError) -> Self {
        tracing::error!("{}\n", error);
        CLIError::Crate("snarkvm_errors", error.to_string())
    }
}

impl From<snarkvm_errors::gadgets::SynthesisError> for CLIError {
    fn from(error: snarkvm_errors::gadgets::SynthesisError) -> Self {
        tracing::error!("{}\n", error);
        CLIError::Crate("snarkvm_errors", error.to_string())
    }
}

impl From<serde_json::error::Error> for CLIError {
    fn from(error: serde_json::error::Error) -> Self {
        tracing::error!("{}\n", error);
        CLIError::Crate("serde_json", error.to_string())
    }
}

impl From<std::io::Error> for CLIError {
    fn from(error: std::io::Error) -> Self {
        tracing::error!("{}\n", error);
        CLIError::Crate("std::io", error.to_string())
    }
}
