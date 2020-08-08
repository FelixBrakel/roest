use crate::core_systems::resource_manager::data_loaders::{PrLoaderError, ProgramLoader};
use crate::core_systems::resource_manager::Loader;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Program Loader error")]
    ProgramLoader(#[cause] PrLoaderError)
}

pub struct FlatMatLoader {
    gl: gl::Gl
}

impl FlatMatLoader {
    pub fn new(gl: gl::Gl) -> Self {
        FlatMatLoader { gl }
    }
}

impl Loader for FlatMatLoader {
    type E = Error;
    type R = flat::Material;

    fn load(&self, name: impl AsRef<Path>) -> Result<flat::Material, Error>{
        let program = ProgramLoader::new(self.gl.clone()).load("core_resources/shaders/basic")
            .map_err(|e| Error::ProgramLoader(e))?;

        Ok(flat::Material::new(self.gl.clone(), program))
    }
}

