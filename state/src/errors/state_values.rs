use crate::InputValueError;

use std::{num::ParseIntError, str::ParseBoolError};

#[derive(Debug, Error)]
pub enum StateValuesError {
    #[error("{}", _0)]
    InputValueError(#[from] InputValueError),

    #[error("state parameter `{}` not found in state file", _0)]
    MissingParameter(String),

    #[error("{}", _0)]
    ParseBoolError(#[from] ParseBoolError),

    #[error("{}", _0)]
    ParseIntError(#[from] ParseIntError),
}
