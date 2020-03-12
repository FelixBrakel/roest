use std::path::{Path, PathBuf};
use failure::Fail;
use std::io;

pub mod synchronous;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "I/O error")]
    Io(#[cause] io::Error),
    #[fail(display = "Failed to read CString from file that contains 0")]
    FileContainsNil,
    #[fail(display = "Failed to get executable path")]
    FailedToGetExePath,
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}

struct Config {
    root_path: PathBuf
}



// Converts a path relative to the project root to an absolute one
pub fn file_name_to_path(name: impl AsRef<Path>) -> Result<PathBuf, Error> {
    let exe = std::env::current_exe().map_err(|_| Error::FailedToGetExePath)?;
    let path = exe.parent().ok_or(Error::FailedToGetExePath)?;

    Ok(path.join(name))

}