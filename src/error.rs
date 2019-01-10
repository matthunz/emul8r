use failure::Fail;
use std::io::Error as IOError;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Please provide a file path.")]
    Path,
    #[fail(display = "IO Error: {:?}", inner)]
    IO { inner: IOError },
    #[fail(display = "Unimplemented: {:x}", op)]
    Unimplemented { op: u16 },
}

impl From<IOError> for Error {
    fn from(inner: IOError) -> Self {
        Error::IO { inner }
    }
}
