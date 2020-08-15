use std::ffi::OsString;

#[derive(Debug, Error)]
pub enum LoginError {
    #[error("{:?}", _0)]
    CannotGetToken(OsString),

    #[error("No connection found {:?}", _0)]
    NoConnectionFound(OsString),

    #[error("No login credentials were provided")]
    NoCredentialsProvided,

    #[error("Wrong login or password {:?}", _0)]
    WrongLoginOrPassword(OsString),
}
