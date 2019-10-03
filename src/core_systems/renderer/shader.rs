use std;
use std::path::{Path,};
use std::ffi::CStr;
use core_systems::renderer::create_initialized_cstring;
use core_systems::resource_manager::{Resource,};
use core_systems::file_system::synchronous::read_to_cstring;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failed to load resource {}", name)]
    ResourceLoad { name: String, inner: failure::Error },
    #[fail(display = "Can not determine shader type for resource {}", name)]
    CanNotDetermineShaderTypeForResource { name: String },
    #[fail(display = "Failed to compile shader {}: {}", name, message)]
    CompileError { name: String, message: String },
}

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
    fn load(gl: &gl::Gl, name: &AsRef<Path>) -> Result<Shader, Error> {
        const POSSIBLE_EXT: [(&str, gl::types::GLenum); 2] = [
            (".vert", gl::VERTEX_SHADER),
            (".frag", gl::FRAGMENT_SHADER),
        ];

        let name_path = name.as_ref();
        let shader_kind = POSSIBLE_EXT
            .iter()
            .find(|&&(file_extension, _)| { name_path.ends_with(file_extension) })
            .map(|&(_, kind)| kind)
            .ok_or_else(|| Error::CanNotDetermineShaderTypeForResource { name: name_path })?;

        let shader_src = read_to_cstring(&name)
            .map_err(|e| Error::ResourceLoad { name: name, inner: e })?;

        Self::load_source(gl, &shader_src, shader_kind)
            .map_err(|message| Error::CompileError { name: name, message })
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.id);
        }
    }
}