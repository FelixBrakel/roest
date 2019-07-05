use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use std::io::Read;
use std::ffi::{CString};
use core_systems::resource_manager::{Resource, Error as ResError};

pub fn read_to_string<R: Resource, P: AsRef<Path>>(res: &R,
                                                   res_name: &P) -> Result<String, io::Error> {
    let mut fp = fs::File::open(R::resource_name_to_path(&res_name))?;
    let mut str_buf = String::new();
    fp.read_to_string(&mut str_buf)?;

    Ok(str_buf)
}

pub fn read_to_cstring<R: Resource, P: AsRef<Path>>(res_name: &P) -> Result<CString, ResError> {
    let mut fp = fs::File::open(R::resource_name_to_path(&res_name))?;
    let mut buff: Vec<u8> = Vec::with_capacity(fp.metadata()?.len() as usize + 1);
    fp.read_to_end(&mut buff)?;

    if buff.iter().find(|i| **i == 0).is_some() {
        return Err(ResError::FileContainsNil);
    }

    Ok(unsafe {
        CString::from_vec_unchecked(buff)
    })
}

