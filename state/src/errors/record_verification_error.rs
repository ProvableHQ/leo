use snarkos_errors::{algorithms::CommitmentError, objects::account::AccountError};

use std::{io::Error as IOError, num::ParseIntError, str::ParseBoolError};

#[derive(Debug, Error)]
pub enum RecordVerificationError {
    #[error("{}", _0)]
    AccountError(#[from] AccountError),

    #[error("{}", _0)]
    CommitmentError(#[from] CommitmentError),

    #[error("expected parameter array of u8 bytes, found `{}`", _0)]
    ExpectedBytes(String),

    #[error("expected integer parameter, found `{}`", _0)]
    ExpectedInteger(String),

    #[error("{}", _0)]
    IOError(#[from] IOError),

    #[error("record parameter `{}` not found in state file", _0)]
    MissingParameter(String),

    #[error("{}", _0)]
    ParseBoolError(#[from] ParseBoolError),

    #[error("{}", _0)]
    ParseIntError(#[from] ParseIntError),
}
