use gl;
use std;
use std::fmt;
use crate::core_systems::renderer::shader::{Shader, Error as ShaderError, ShaderLoader};
use std::path::{Path, PathBuf};
use crate::core_systems::resource_manager::{Resource, ResError, create_initialized_cstring, Loadable};
use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    LinkError {name: PathBuf, message: String},
    ShaderError {name: PathBuf, #[cause] inner: ShaderError},
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::LinkError { name, message } =>
                write!(f, "Failed to link program {}: {}", name.display(), message),
            Error::ShaderError { name, inner: _ } =>
                write!(f, "Failed to compile shader {}", name.display())
        }
    }
}

impl ResError for Error {}

pub struct Program {
    gl: gl::Gl,
    id: gl::types::GLuint,
}

impl Resource for Program {}

impl Program {
    fn load_shaders(gl: gl::Gl, shaders: &[Shader]) -> Result<Self, String> {
        let id = unsafe { gl.CreateProgram() };
        for shader in shaders {
            unsafe {
                gl.AttachShader(id, shader.id());
            }
        }

        unsafe {
            gl.LinkProgram(id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl.GetProgramiv(id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe  {
                gl.GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_initialized_cstring(len as usize);
            unsafe {
                gl.GetProgramInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe {
                gl.DetachShader(id, shader.id());
            }
        }
        Ok(Program { gl, id })
    }

    pub fn set_used(&self) {
        unsafe {
            self.gl.UseProgram(self.id);
        }
    }
}

pub struct ProgramLoader {
    gl: gl::Gl,
}

impl ProgramLoader {
    pub fn new(gl: gl::Gl) -> Self {
        ProgramLoader { gl }
    }
}

impl Loadable for ProgramLoader {
    type E = Error;
    type R = Program;

    fn load(&self, name: impl AsRef<Path>) -> Result<Program, Error> {
        const POSSIBLE_EXT: [&str; 2] = [
            ".vert",
            ".frag",
        ];

        let shaders = POSSIBLE_EXT.iter()
            .map(|file_extension| {
                let loadable = ShaderLoader::new(self.gl.clone());
                loadable.load( format!("{}{}", name.as_ref().display(), file_extension))
            })
            .collect::<Result<Vec<Shader>, ShaderError>>().map_err(|e| Error::ShaderError {name: name.as_ref().to_path_buf(), inner: e})?;

        Program::load_shaders(self.gl.clone(), &shaders[..])
            .map_err(|message| Error::LinkError { name: name.as_ref().to_path_buf(), message })
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.id);
        }
    }
}