use std::ffi::OsString;

#[derive(Debug, Error)]
pub enum LoginError {
    #[error("{:?}", _0)]
    CannotGetToken(OsString),

    #[error("connection unavailable {:?}", _0)]
    ConnectionUnavailable(OsString),

    #[error("wrong login or password {:?}", _0)]
    WrongLoginOrPassword(OsString),
}
