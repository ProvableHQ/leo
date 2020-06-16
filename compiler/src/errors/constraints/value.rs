use crate::errors::{FieldError, GroupError};
use leo_types::IntegerError;

use snarkos_errors::gadgets::SynthesisError;
use std::{num::ParseIntError, str::ParseBoolError};

#[derive(Debug, Error)]
pub enum ValueError {
    #[error("{}", _0)]
    FieldError(#[from] FieldError),

    #[error("{}", _0)]
    GroupError(#[from] GroupError),

    #[error("{}", _0)]
    IntegerError(#[from] IntegerError),

    #[error("{}", _0)]
    ParseBoolError(#[from] ParseBoolError),

    #[error("{}", _0)]
    ParseIntError(#[from] ParseIntError),

    #[error("{}", _0)]
    SynthesisError(#[from] SynthesisError),
}
