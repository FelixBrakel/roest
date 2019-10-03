/**
 * Synchronous file system API, as the name implies this version only contains blocking calls so it
 * should probably only be used during startup and not runtime.
 **/

use std::path::{Path,};
use std::fs;
use std::io;
use std::io::Read;
use std::ffi::{CString};
use core_systems::resource_manager::{Resource,};
use core_systems::file_system::{file_name_to_path,};

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "I/O error")]
    Io(io::Error),
    #[fail(display = "Failed to get executable path")]
    FileContainsNil,
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}

pub fn read_to_string<R: Resource, P: AsRef<Path>>(res: &R,
                                                   res_name: &P) -> Result<String, io::Error> {
    let mut fp = fs::File::open(R::resource_name_to_path(&res_name))?;
    let mut str_buf = String::new();
    fp.read_to_string(&mut str_buf)?;

    Ok(str_buf)
}

pub fn read_to_cstring(res_name: &AsRef<Path>) -> Result<CString, Error> {
    let mut fp = fs::File::open(file_name_to_path(&res_name))?;
    let mut buff: Vec<u8> = Vec::with_capacity(fp.metadata()?.len() as usize + 1);
    fp.read_to_end(&mut buff)?;

    if buff.iter().find(|i| **i == 0).is_some() {
        return Err(Error::FileContainsNil);
    }

    Ok(unsafe {
        CString::from_vec_unchecked(buff)
    })
}
