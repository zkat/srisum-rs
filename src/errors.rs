use failure::Fail;
use ssri;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "File failed integrity check: {}", _0)]
    IntegrityError(String),
    #[fail(display = "failed to parse integrity string")]
    ParseIntegrityError(#[cause] ssri::Error),
    #[fail(display = "Got an IO Error: {}", _0)]
    Io(std::io::Error),
}

impl From<ssri::Error> for Error {
    fn from(error: ssri::Error) -> Self {
        Error::ParseIntegrityError(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}
