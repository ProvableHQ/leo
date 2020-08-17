use std::ffi::OsString;

#[derive(Debug, Error)]
pub enum AddError {
    #[error("connection unavailable {:?}", _0)]
    ConnectionUnavailable(OsString),

    #[error("missing author or package name")]
    MissingAuthorOrPackageName,

    #[error("{:?}", _0)]
    ZipError(OsString),
}
