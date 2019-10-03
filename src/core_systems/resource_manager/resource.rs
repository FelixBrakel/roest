/**
 * A resource is anything use at runtime that is loaded from disk, for example textures, models or
 * shaders. A config file would not be considered a resource since it is not loaded at runtime
 **/
use std::path::{Path,};

// Struct should implement this trait for them to be able to be created from data on the disk, the
// The file_system mod should act as a library for the structs to easily implement asynchronous
// file streaming, loading to CStrings, etc. The main function of this trait is to make sure the
// correct loading precedure is done for a resource.
pub trait Resource {
    fn load(gl: &gl::Gl, path: &AsRef<Path>) -> Result<Self, failure::Error> where Self: Sized;
}