use std::ffi::OsString;

#[derive(Debug, Error)]
pub enum PublishError {
    #[error("connection unavailable {:?}", _0)]
    ConnectionUnavalaible(OsString),

    #[error("package toml file is missing a description")]
    MissingPackageDescription,

    #[error("package toml file is missing a license")]
    MissingPackageLicense,

    #[error("package toml file is missing a remote")]
    MissingPackageRemote,

    #[error("package not published {:?}", _0)]
    PackageNotPublished(OsString),
}
