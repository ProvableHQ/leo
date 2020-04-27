use std::io;

#[derive(Debug, Fail)]
pub enum ManifestError {

    #[fail(display = "`{}` creating: {}", _0, _1)]
    Creating(&'static str, io::Error),

    #[fail(display = "`{}` metadata: {}", _0, _1)]
    Metadata(&'static str, io::Error),

    #[fail(display = "`{}` opening: {}", _0, _1)]
    Opening(&'static str, io::Error),

    #[fail(display = "`{}` parsing: {}", _0, _1)]
    Parsing(&'static str, toml::de::Error),

    #[fail(display = "`{}` reading: {}", _0, _1)]
    Reading(&'static str, io::Error),

    #[fail(display = "`{}` writing: {}", _0, _1)]
    Writing(&'static str, io::Error),

}
