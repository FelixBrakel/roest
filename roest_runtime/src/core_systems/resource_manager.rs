pub mod data_loaders;

mod resource;
pub use self::resource::{Loader};
use roefs::synchronous::{File};
use std::path::{Path, PathBuf};
use std::ffi::{CString};
use lazy_static::{lazy_static};
use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failed to read CString from file that contains 0")]
    FileContainsNil,
    #[fail(display = "Failed to get executable path")]
    FailedToGetExePath,
    #[fail(display = "I/O error")]
    Roefs(#[cause] roefs::Error),
}

impl From<roefs::Error> for Error {
    fn from(other: roefs::Error) -> Self {
        Error::Roefs(other)
    }
}

struct Config {
    root_path: PathBuf
}

lazy_static!(
    static ref CONFIG: Config = {
        let exe = std::env::current_exe().map_err(|_| Error::FailedToGetExePath).unwrap();
        let path = exe.parent().ok_or(Error::FailedToGetExePath).unwrap();

        Config { root_path: path.to_path_buf() }
    };
);

// Converts a path relative to the project root to an absolute one
pub fn file_name_to_path(name: impl AsRef<Path>) -> Result<PathBuf, Error> {
    Ok(CONFIG.root_path.join(name))
}

fn read_to_cstring(res_name: impl AsRef<Path>) -> Result<CString, Error> {
    let full_path = file_name_to_path(res_name)?;
    let mut fp = File::open(full_path)?;
    let mut buff: Vec<u8> = Vec::with_capacity(fp.metadata()?.len() as usize + 1);
    fp.read_to_end(&mut buff)?;

    if buff.iter().find(|i| **i == 0).is_some() {
        return Err(Error::FileContainsNil);
    }

    Ok(unsafe {
        CString::from_vec_unchecked(buff)
    })
}

fn open_file(res_name: impl AsRef<Path>) -> Result<File, Error> {
    let full_path = file_name_to_path(res_name)?;
    File::open(full_path).map_err(|e| Error::Roefs(e))
}