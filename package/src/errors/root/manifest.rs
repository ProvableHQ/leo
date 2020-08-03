use std::io;

#[derive(Debug, Error)]
pub enum ManifestError {
    #[error("`{}` creating: {}", _0, _1)]
    Creating(&'static str, io::Error),

    #[error("`{}` metadata: {}", _0, _1)]
    Metadata(&'static str, io::Error),

    #[error("`{}` opening: {}", _0, _1)]
    Opening(&'static str, io::Error),

    #[error("`{}` parsing: {}", _0, _1)]
    Parsing(&'static str, toml::de::Error),

    #[error("`{}` reading: {}", _0, _1)]
    Reading(&'static str, io::Error),

    #[error("`{}` writing: {}", _0, _1)]
    Writing(&'static str, io::Error),
}
