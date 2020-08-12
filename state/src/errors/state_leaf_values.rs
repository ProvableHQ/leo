use crate::InputValueError;

use std::{num::ParseIntError, str::ParseBoolError};

#[derive(Debug, Error)]
pub enum StateLeafValuesError {
    #[error("{}", _0)]
    InputValueError(#[from] InputValueError),

    #[error("{}", _0)]
    ParseBoolError(#[from] ParseBoolError),

    #[error("{}", _0)]
    ParseIntError(#[from] ParseIntError),
}
