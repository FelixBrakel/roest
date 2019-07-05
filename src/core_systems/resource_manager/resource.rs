use std::path::{Path, PathBuf};
use std::io::{self};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    FileContainsNil,
    FailedToGetBinPath,
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}

// Struct should implement this trait for them to be able to be created from data on the disk, the
// The file_system mod should act as a library for the structs to easily implement asynchronous
// file streaming, loading to CStrings, etc. The main function of this trait is to make sure struct
// have a file root from which relative paths can be build and for the structs to implement custom
// loading (and unloading?) procedures. More functionality will sure
pub trait Resource {
    fn get_name() -> PathBuf;
}