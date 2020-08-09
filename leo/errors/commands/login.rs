use std::ffi::OsString;

#[derive(Debug, Error)]
pub enum LoginError {
    #[error("{:?}", _0)]
    CannotGetToken(OsString),

    #[error("connectin unavalaible {:?}", _0)]
    ConnectionUnavalaible(OsString),

    #[error("wrong login or password {:?}", _0)]
    WrongLoginOrPassword(OsString),
}
