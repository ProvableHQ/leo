use crate::errors::{BooleanError, FieldError, GroupError};
use leo_types::IntegerError;

use snarkos_errors::gadgets::SynthesisError;

#[derive(Debug, Error)]
pub enum ValueError {
    #[error("{}", _0)]
    BooleanError(#[from] BooleanError),

    #[error("{}", _0)]
    FieldError(#[from] FieldError),

    #[error("{}", _0)]
    GroupError(#[from] GroupError),

    #[error("{}", _0)]
    IntegerError(#[from] IntegerError),

    #[error("{}", _0)]
    SynthesisError(#[from] SynthesisError),
}
