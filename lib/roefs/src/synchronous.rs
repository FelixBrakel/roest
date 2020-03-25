/**
 * Synchronous file system API, as the name implies this version only contains blocking calls so it
 * should probably only be used during startup and not runtime.
 **/

use std::path::{Path,};
use std::fs;
use std::io;
use std::io::{Read};
use crate::Error;

pub use std::fs::{Metadata};

pub struct File {
    inner: fs::File,
}

impl File {
    pub fn open(path: impl AsRef<Path>) -> Result<File, Error> {
        Ok(fs::File::open(path)?.into())
    }

    pub fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        Ok(self.inner.read_to_end(buf)?)
    }

    pub fn metadata(&self) -> Result<Metadata, Error> {
        Ok(self.inner.metadata()?)
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        self.inner.read(buf)
    }
}

impl From<fs::File> for File {
    fn from(other: fs::File) -> Self {
        File { inner: other }
    }
}