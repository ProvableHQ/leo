use std::io;

#[derive(Debug, Fail)]
pub enum MainFileError {
    #[fail(display = "creating: {}", _0)]
    Creating(io::Error),
    #[fail(display = "writing: {}", _0)]
    Writing(io::Error),
}
