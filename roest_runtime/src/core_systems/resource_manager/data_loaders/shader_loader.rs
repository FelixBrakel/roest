use gl_renderer::{Shader, ShaderError};
use std::path::{Path};
use crate::core_systems::resource_manager::{Loader, read_to_cstring, ResError};
use crate::core_systems::resource_manager;
use failure::Fail;

impl ResError for Error {}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Shader error {}", name)]
    ShaderError { name: String, #[cause] inner:  ShaderError },
    #[fail(display = "Failed to load resource {}", name)]
    ResourceLoad { name: String, #[cause] inner: resource_manager::Error },
    #[fail(display = "Can not determine shader type for resource {}", name)]
    CanNotDetermineShaderTypeForResource { name: String },
}

pub struct ShaderLoader {
    gl: gl::Gl,
}

impl ShaderLoader {
    pub fn new(gl: gl::Gl) -> Self {
        ShaderLoader { gl }
    }
}

impl Loader for ShaderLoader {
    type E = Error;
    type R = Shader;

    fn load(&self, name: impl AsRef<Path>) -> Result<Shader, Error> {
        let name_path = name.as_ref();

        let shader_kind = match name_path.extension() {
            None => return Err(Error::CanNotDetermineShaderTypeForResource { name: name_path.to_string_lossy().into_owned()}),
            Some(ext) => {
                match ext.to_str() {
                    Some("vert") => gl::VERTEX_SHADER,
                    Some("frag") => gl::FRAGMENT_SHADER,
                    _ => return Err(Error::CanNotDetermineShaderTypeForResource { name: name_path.to_string_lossy().into_owned() })
                }
            }
        };

        let shader_src = read_to_cstring(&name)
            .map_err(|e| Error::ResourceLoad { name: name_path.to_string_lossy().into_owned(), inner: e })?;
        Shader::load_source(self.gl.clone(), &shader_src, shader_kind)
            .map_err(|e| Error::ShaderError { name: name_path.to_string_lossy().into_owned(), inner: e })
    }
}
