// Copyright (C) 2019-2020 Aleo Systems Inc.
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
use leo_package::errors::*;

#[derive(Debug, Error)]
pub enum CLIError {
    #[error("{}", _0)]
    AddError(AddError),

    #[error("{}", _0)]
    BuildError(BuildError),

    #[error("{}", _0)]
    BytesFileError(ZipFileError),

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
    InputDirectoryError(InputsDirectoryError),

    #[error("{}", _0)]
    InputFileError(InputFileError),

    #[error("{}", _0)]
    LibFileError(LibFileError),

    #[error("{}", _0)]
    LoginError(LoginError),

    #[error("{}", _0)]
    MainFileError(MainFileError),

    #[error("{}", _0)]
    ManifestError(ManifestError),

    #[error("{}", _0)]
    NewError(NewError),

    #[error("{}", _0)]
    OutputDirectoryError(OutputsDirectoryError),

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
    SNARKError(snarkos_errors::algorithms::snark::SNARKError),

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

macro_rules! impl_T {
    (for $($t:tt), +) => {
        $(impl From<$t> for CLIError {
            fn from(error: $t) -> Self {
                tracing::error!("{}\n", error);

                CLIError::$t(error)
            }
        })*
    }
}

impl From<ZipFileError> for CLIError {
    fn from(error: ZipFileError) -> Self {
        tracing::error!("{}\n", error);
        CLIError::BytesFileError(error)
    }
}

impl From<BuildError> for CLIError {
    fn from(error: BuildError) -> Self {
        tracing::error!("{}\n", error);
        CLIError::BuildError(error)
    }
}

impl From<AddError> for CLIError {
    fn from(error: AddError) -> Self {
        tracing::error!("{}\n", error);
        CLIError::AddError(error)
    }
}

impl From<ChecksumFileError> for CLIError {
    fn from(error: ChecksumFileError) -> Self {
        tracing::error!("{}\n", error);
        CLIError::ChecksumFileError(error)
    }
}

impl From<CircuitFileError> for CLIError {
    fn from(error: CircuitFileError) -> Self {
        tracing::error!("{}\n", error);
        CLIError::CircuitFileError(error)
    }
}

impl From<GitignoreError> for CLIError {
    fn from(error: GitignoreError) -> Self {
        tracing::error!("{}\n", error);
        CLIError::GitignoreError(error)
    }
}

impl From<InitError> for CLIError {
    fn from(error: InitError) -> Self {
        tracing::error!("{}\n", error);
        CLIError::InitError(error)
    }
}

impl From<ImportsDirectoryError> for CLIError {
    fn from(error: ImportsDirectoryError) -> Self {
        tracing::error!("{}\n", error);
        CLIError::ImportsDirectoryError(error)
    }
}

impl From<InputsDirectoryError> for CLIError {
    fn from(error: InputsDirectoryError) -> Self {
        tracing::error!("{}\n", error);
        CLIError::InputDirectoryError(error)
    }
}

impl From<InputFileError> for CLIError {
    fn from(error: InputFileError) -> Self {
        tracing::error!("{}\n", error);
        CLIError::InputFileError(error)
    }
}

impl From<LibFileError> for CLIError {
    fn from(error: LibFileError) -> Self {
        tracing::error!("{}\n", error);
        CLIError::LibFileError(error)
    }
}

impl From<LoginError> for CLIError {
    fn from(error: LoginError) -> Self {
        tracing::error!("{}\n", error);
        CLIError::LoginError(error)
    }
}

impl From<MainFileError> for CLIError {
    fn from(error: MainFileError) -> Self {
        tracing::error!("{}\n", error);
        CLIError::MainFileError(error)
    }
}

impl From<ManifestError> for CLIError {
    fn from(error: ManifestError) -> Self {
        tracing::error!("{}\n", error);
        CLIError::ManifestError(error)
    }
}

impl From<NewError> for CLIError {
    fn from(error: NewError) -> Self {
        tracing::error!("{}\n", error);
        CLIError::NewError(error)
    }
}

impl From<OutputsDirectoryError> for CLIError {
    fn from(error: OutputsDirectoryError) -> Self {
        tracing::error!("{}\n", error);
        CLIError::OutputDirectoryError(error)
    }
}

impl From<ProofFileError> for CLIError {
    fn from(error: ProofFileError) -> Self {
        tracing::error!("{}\n", error);
        CLIError::ProofFileError(error)
    }
}

impl From<ProvingKeyFileError> for CLIError {
    fn from(error: ProvingKeyFileError) -> Self {
        tracing::error!("{}\n", error);
        CLIError::ProvingKeyFileError(error)
    }
}

impl From<PublishError> for CLIError {
    fn from(error: PublishError) -> Self {
        tracing::error!("{}\n", error);
        CLIError::PublishError(error)
    }
}

impl From<READMEError> for CLIError {
    fn from(error: READMEError) -> Self {
        tracing::error!("{}\n", error);
        CLIError::READMEError(error)
    }
}

impl From<RunError> for CLIError {
    fn from(error: RunError) -> Self {
        tracing::error!("{}\n", error);
        CLIError::RunError(error)
    }
}

impl From<SourceDirectoryError> for CLIError {
    fn from(error: SourceDirectoryError) -> Self {
        tracing::error!("{}\n", error);
        CLIError::SourceDirectoryError(error)
    }
}

impl From<StateFileError> for CLIError {
    fn from(error: StateFileError) -> Self {
        tracing::error!("{}\n", error);
        CLIError::StateFileError(error)
    }
}

impl From<TestError> for CLIError {
    fn from(error: TestError) -> Self {
        tracing::error!("{}\n", error);
        CLIError::TestError(error)
    }
}

impl From<VerificationKeyFileError> for CLIError {
    fn from(error: VerificationKeyFileError) -> Self {
        tracing::error!("{}\n", error);
        CLIError::VerificationKeyFileError(error)
    }
}

impl From<leo_compiler::errors::CompilerError> for CLIError {
    fn from(error: leo_compiler::errors::CompilerError) -> Self {
        tracing::error!("{}\n", error);
        CLIError::Crate("leo_compiler", "Program failed due to previous error".into())
    }
}

impl From<leo_input::errors::InputParserError> for CLIError {
    fn from(error: leo_input::errors::InputParserError) -> Self {
        tracing::error!("{}\n", error);
        CLIError::Crate("leo_input", "Program failed due to previous error".into())
    }
}

impl From<reqwest::Error> for CLIError {
    fn from(error: reqwest::Error) -> Self {
        tracing::error!("{}\n", error);
        CLIError::Crate("rewquest", format!("{}", error))
    }
}

impl From<snarkos_errors::algorithms::snark::SNARKError> for CLIError {
    fn from(error: snarkos_errors::algorithms::snark::SNARKError) -> Self {
        tracing::error!("{}\n", error);
        CLIError::Crate("snarkos_errors", format!("{}", error))
    }
}

impl From<snarkos_errors::gadgets::SynthesisError> for CLIError {
    fn from(error: snarkos_errors::gadgets::SynthesisError) -> Self {
        tracing::error!("{}\n", error);
        CLIError::Crate("snarkos_errors", format!("{}", error))
    }
}

impl From<serde_json::error::Error> for CLIError {
    fn from(error: serde_json::error::Error) -> Self {
        tracing::error!("{}\n", error);
        CLIError::Crate("serde_json", format!("{}", error))
    }
}

impl From<std::io::Error> for CLIError {
    fn from(error: std::io::Error) -> Self {
        tracing::error!("{}\n", error);
        CLIError::Crate("std::io", format!("{}", error))
    }
}
