use crate::InputValueError;

use snarkos_errors::objects::account::AccountError;

use std::{num::ParseIntError, str::ParseBoolError};

#[derive(Debug, Error)]
pub enum DPCRecordValuesError {
    #[error("{}", _0)]
    AccountError(#[from] AccountError),

    #[error("{}", _0)]
    InputValueError(#[from] InputValueError),

    #[error("{}", _0)]
    ParseBoolError(#[from] ParseBoolError),

    #[error("{}", _0)]
    ParseIntError(#[from] ParseIntError),
}
