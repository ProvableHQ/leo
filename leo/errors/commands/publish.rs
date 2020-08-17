use std::ffi::OsString;

#[derive(Debug, Error)]
pub enum PublishError {
    #[error("missing package author")]
    MissingPackageAuthor,
}
