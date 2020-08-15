use std::ffi::OsString;

#[derive(Debug, Error)]
pub enum AddError {
    #[error("connection unavailable {:?}", _0)]
    ConnectionUnavalaible(OsString),
}
