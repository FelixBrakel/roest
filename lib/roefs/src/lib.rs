use thiserror::Error;
use std::io;

pub mod synchronous;

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error")]
    Io(#[source] io::Error),
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}