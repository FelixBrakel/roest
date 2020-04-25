use std::ffi::{CStr};
use failure::Fail;
use super::{create_initialized_cstring};

#[derive(Debug, Fail)]
pub enum Error {
   #[fail(display = "Failed to compile shader: {}", message)]
    CompileError { message: String },
}

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    pub fn load_source(source: &CStr, kind: gl::types::GLenum) -> Result<Shader, Error> {
        let id = unsafe {
            gl::CreateShader(kind)
        };

        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_initialized_cstring(len as usize);

            unsafe {
                gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut gl::types::GLchar);
            }

            return Err(Error::CompileError { message: error.to_string_lossy().into_owned() });
        }


        Ok(Shader { id })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}