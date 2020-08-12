use crate::{RecordVerificationError, StateLeafValuesError, StateValuesError};

use snarkos_errors::algorithms::{CommitmentError, MerkleError};

use std::io::Error as IOError;

#[derive(Debug, Error)]
pub enum LocalDataVerificationError {
    #[error("{}", _0)]
    CommitmentError(#[from] CommitmentError),

    #[error("{}", _0)]
    MerkleError(#[from] MerkleError),

    #[error("{}", _0)]
    IOError(#[from] IOError),

    #[error("{}", _0)]
    RecordVerificationError(#[from] RecordVerificationError),

    #[error("{}", _0)]
    StateLeafValuesError(#[from] StateLeafValuesError),

    #[error("{}", _0)]
    StateValuesError(#[from] StateValuesError),
}
