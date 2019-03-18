use std::path::Path;
use std::fs;
use std::io;
use std::io::Read;
use std::ffi::{CString, NulError};

pub fn read_file_to_string<P: AsRef<Path>>(filepath: P) -> Result<String, io::Error> {
    let mut fp = fs::File::open(filepath)?;
    let mut str_buf = String::new();
    fp.read_to_string(&mut str_buf)?;

    Ok(str_buf)
}

pub fn read_file_to_cstring<P: AsRef<Path>>(filepath: P) -> Result<CString, NulError> {
    let str_buf = read_file_to_string(filepath).unwrap();
    CString::new(str_buf)
}

