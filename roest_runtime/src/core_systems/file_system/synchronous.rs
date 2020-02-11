/**
 * Synchronous file system API, as the name implies this version only contains blocking calls so it
 * should probably only be used during startup and not runtime.
 **/

use std::path::{Path,};
use std::fs;
use std::io::Read;
use std::ffi::{CString};
use crate::core_systems::file_system::{file_name_to_path, Error};

pub fn read_to_cstring(res_name: impl AsRef<Path>) -> Result<CString, Error> {
    let full_path = file_name_to_path(res_name)?;
    let mut fp = fs::File::open(full_path)?;
    let mut buff: Vec<u8> = Vec::with_capacity(fp.metadata()?.len() as usize + 1);
    fp.read_to_end(&mut buff)?;

    if buff.iter().find(|i| **i == 0).is_some() {
        return Err(Error::FileContainsNil);
    }

    Ok(unsafe {
        CString::from_vec_unchecked(buff)
    })
}

pub fn create_initialized_cstring(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}