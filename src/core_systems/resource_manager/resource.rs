use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self, Read};
use std::ffi;

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
// loading (and unloading?) procedures.
pub trait Resource {
    const ROOT_PATH: PathBuf;
    fn from_relative_root_path<P: AsRef<Path>>(&rel_path: P) -> Result<Resource, Error>;

    //Note:
    // maybe this shouldn't be part of the resource trait? Might be better off moving this to
    // the file_system mod as a helper function.
    fn resource_name_to_path<P: AsRef<Path>>(&name: P) -> PathBuf {
        Resource::ROOT_PATH.join(name)
    }
}