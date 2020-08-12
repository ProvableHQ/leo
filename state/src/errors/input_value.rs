use std::num::ParseIntError;

#[derive(Debug, Error)]
pub enum InputValueError {
    #[error("expected parameter array of u8 bytes, found `{}`", _0)]
    ExpectedBytes(String),

    #[error("expected integer parameter, found `{}`", _0)]
    ExpectedInteger(String),

    #[error("{}", _0)]
    ParseIntError(#[from] ParseIntError),
}
