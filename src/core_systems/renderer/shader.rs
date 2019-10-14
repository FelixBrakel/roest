use std;
use std::path::{Path, PathBuf};
use std::ffi::CStr;
use crate::core_systems::renderer::create_initialized_cstring;
use crate::core_systems::resource_manager::{Resource, ResError};
use crate::core_systems::file_system::synchronous::{read_to_cstring,};
use crate::core_systems::file_system::{Error as FsError};
use failure::Fail;
use std::fmt;

#[derive(Debug, Fail)]
pub enum Error {
//    #[fail(display = "Failed to load resource {}", name)]
    ResourceLoad { name: PathBuf, #[cause] inner: FsError },
//    #[fail(display = "Can not determine shader type for resource {}", name)]
    CanNotDetermineShaderTypeForResource { name: PathBuf },
//    #[fail(display = "Failed to compile shader {}: {}", name, message)]
    CompileError { name: PathBuf, message: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ResourceLoad { name, inner: _ } => write!(f, "Failed to load resource {}", name.display()),
            Error::CanNotDetermineShaderTypeForResource { name } => write!(f, "Can not determine shader type for resource {}", name.display()),
            Error::CompileError { name, message } => write!(f, "Failed to compile shader {}: {}", name.display(), message)
        }
    }
}

impl ResError for Error {}

pub struct Shader {
    gl: gl::Gl,
    id: gl::types::GLuint,
}

impl Shader {
    fn load_source(gl: &gl::Gl, source: &CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
        let id = unsafe {
            gl.CreateShader(kind)
        };

        unsafe {
            gl.ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl.CompileShader(id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_initialized_cstring(len as usize);

            unsafe {
                gl.GetShaderInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut gl::types::GLchar);
            }

            return Err(error.to_string_lossy().into_owned());
        }


        Ok(Shader { gl: gl.clone(), id: id })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Resource for Shader {
    type E = Error;

    fn load(gl: &gl::Gl, name: impl AsRef<Path>) -> Result<Shader, Error> {
        let name_path = name.as_ref();

        let shader_kind = match name_path.extension() {
            None => return Err(Error::CanNotDetermineShaderTypeForResource { name: name_path.to_path_buf() }),
            Some(ext) => {
                match ext.to_str() {
                    Some("vert") => gl::VERTEX_SHADER,
                    Some("frag") => gl::FRAGMENT_SHADER,
                    _ => return Err(Error::CanNotDetermineShaderTypeForResource { name: name_path.to_path_buf() })
                }
            }
        };

        let shader_src = read_to_cstring(&name)
            .map_err(|e| Error::ResourceLoad { name: name_path.to_path_buf(), inner: e })?;

        Self::load_source(gl, &shader_src, shader_kind)
            .map_err(|message| Error::CompileError { name: name_path.to_path_buf(), message })
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.id);
        }
    }
}