pub mod data_loader;

mod resource;
pub use self::resource::Resource;
use core_systems::file_system::{file_name_to_path};
use std::path::{Path};

pub fn load_resource<R: Resource>(gl: &gl::Gl, rel_path: &AsRef<Path>, e: R) -> Result<R, failure::Error> {
    let path = file_name_to_path(rel_path)?;
    R::load(gl, path)
}