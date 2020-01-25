/**
 * A resource is anything use at runtime that is loaded from disk, for example textures, models or
 * shaders. A config file would not be considered a resource since it is not loaded at runtime
 **/
use std::path::{Path,};

pub trait ResError: std::fmt::Debug + failure::Fail {}

//impl<'a, E: ResError + 'a> From<E> for Box<dyn ResError + 'a> {
//    fn from(err: E) -> Box<dyn ResError + 'a> {
//        Box::new(err)
//    }
//}

// Struct should implement this trait for them to be able to be created from data on the disk, the
// The file_system mod should act as a library for the structs to easily implement asynchronous
// file streaming, loading to CStrings, etc. The main function of this trait is to make sure the
// correct loading procedure is done for a resource and that resources can return an error associated with the
// implementation.
pub trait Resource {
    type E: ResError; // Error that the implementation of this struct will throw in case of some problem, this error
    // should encase any underlying error with for example the file system.

    fn load(gl: &gl::Gl, path: impl AsRef<Path>) -> Result<Self, Self::E> where
        Self: Sized,;
}