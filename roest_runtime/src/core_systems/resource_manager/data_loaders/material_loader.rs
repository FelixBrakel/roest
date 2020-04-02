use crate::core_systems::resource_manager::data_loaders::{PrLoaderError, ProgramLoader};
use crate::core_systems::resource_manager::Loader;
use gl_renderer::data::material_data::flat;
use std::path::Path;
use failure::Fail;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Program Loader error")]
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
        let program = ProgramLoader::new(self.gl.clone()).load("resources/shaders/basic")
            .map_err(|e| Error::ProgramLoader(e))?;

        Ok(flat::Material::new(self.gl.clone(), program))
    }
}

