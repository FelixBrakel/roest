use gl_renderer::{Program, ProgramError, Shader};
use std::path::{Path};
use crate::core_systems::resource_manager::{Loader};
use super::{ShaderLoader, ShLoaderError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("ShaderLoader error")]
    ShaderLoader { name: String, #[source] inner: ShLoaderError},
    #[error("Program error")]
    Program { name: String, #[source] inner: ProgramError }
}

pub struct ProgramLoader {  }

impl ProgramLoader {
    pub fn new() -> Self {
        ProgramLoader {  }
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
                let loadable = ShaderLoader::new();
                loadable.load( format!("{}{}", name_path.display(), file_extension))
            })
            .collect::<Result<Vec<Shader>, ShLoaderError>>().map_err(|e| Error::ShaderLoader { name: name_path.to_string_lossy().into_owned(), inner: e })?;

        Program::from_shaders(&shaders[..])
            .map_err(|e| Error::Program { name: name_path.to_string_lossy().into_owned(), inner: e })
    }
}
