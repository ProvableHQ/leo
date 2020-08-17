use std::ffi::OsString;

#[derive(Debug, Error)]
pub enum PublishError {
    #[error("connection unavailable {:?}", _0)]
    ConnectionUnavalaible(OsString),

    #[error("package not published {:?}", _0)]
    PackageNotPublished(OsString),
}
