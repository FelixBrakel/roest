pub mod data_loader;

mod resource;
pub use self::resource::{Resource, ResError};
use std::path::{Path};

pub fn load_resource<R: Resource>(gl: &gl::Gl, rel_path: impl AsRef<Path>) -> Result<R, R::E> {
//    let path = match file_name_to_path(rel_path) {
//        Ok(T) => T,
//        Err(E) => return Err(Box::new(Error::from(E))),
//    };

    R::load(gl, rel_path)
}