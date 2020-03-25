use gl_renderer::{Program, ProgramError, Shader};
use std::path::{Path};
use crate::core_systems::resource_manager::{Loader};
use super::{ShaderLoader, ShLoaderError};
use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "ShaderLoader error")]
    ShaderLoader { name: String, #[cause] inner: ShLoaderError},
    #[fail(display = "Program error")]
    Program { name: String, #[cause] inner: ProgramError }
}

pub struct ProgramLoader {
    gl: gl::Gl,
}

impl ProgramLoader {
    pub fn new(gl: gl::Gl) -> Self {
        ProgramLoader { gl }
    }
}

impl Loader for ProgramLoader {
    type E = Error;
    type R = Program;

    fn load(&self, name: impl AsRef<Path>) -> Result<Program, Error> {
        const POSSIBLE_EXT: [&str; 2] = [
            ".vert",
            ".frag",
        ];

        let name_path = name.as_ref();

        let shaders = POSSIBLE_EXT.iter()
            .map(|file_extension| {
                let loadable = ShaderLoader::new(self.gl.clone());
                loadable.load( format!("{}{}", name_path.display(), file_extension))
            })
            .collect::<Result<Vec<Shader>, ShLoaderError>>().map_err(|e| Error::ShaderLoader { name: name_path.to_string_lossy().into_owned(), inner: e })?;

        Program::load_shaders(self.gl.clone(), &shaders[..])
            .map_err(|e| Error::Program { name: name_path.to_string_lossy().into_owned(), inner: e })
    }
}
