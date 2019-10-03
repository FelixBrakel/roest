use std::path::{Path, PathBuf};

pub mod synchronous;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failed to read CString from file that contains 0")]
    FailedToGetExePath,
}

struct FileSystem {
    root_path: AsRef<Path>
}

//impl FileSystem {
//    fn new() -> *Self {
//
//    }
//}

//pub fn initialize() -> *FileSystem {
//    FileSystem::new()
//}

// Converts a path relative to the root to an absolute one
pub fn file_name_to_path<P: AsRef<Path>>(name: &P) -> Result<PathBuf, Error> {
    let exe = std::env::current_exe().map_err(|_| Error::FailedToGetExePath)?;
    let path = exe.parent().ok_or(Error::FailedToGetExePath)?;

    Ok(path.join(name))
}