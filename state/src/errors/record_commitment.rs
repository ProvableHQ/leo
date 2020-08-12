use crate::DPCRecordValuesError;

use snarkos_errors::algorithms::CommitmentError;

use std::io::Error as IOError;

#[derive(Debug, Error)]
pub enum RecordVerificationError {
    #[error("record commitment does not match record data")]
    CommitmentsDoNotMatch,

    #[error("{}", _0)]
    CommitmentError(#[from] CommitmentError),

    #[error("{}", _0)]
    DPCRecordValuesError(#[from] DPCRecordValuesError),

    #[error("{}", _0)]
    IOError(#[from] IOError),
}
