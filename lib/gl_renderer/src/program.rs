use gl;
use super::{Shader};
use thiserror::Error;
use super::{create_initialized_cstring};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to link program: {}", message)]
    LinkError { message: String },
}

pub struct Program {
    id: gl::types::GLuint,
}

pub trait GlUniform {
    fn gl_uniform(&self);

    fn from_uniform(program: &Program) -> Self;
}

impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Self, Error> {
        let id = Self::load_shaders(shaders).unwrap();
        Ok(Program { id })
    }

    fn load_shaders(shaders: &[Shader]) -> Result<gl::types::GLuint, Error> {
        let id = unsafe { gl::CreateProgram() };
        for shader in shaders {
            unsafe {
                gl::AttachShader(id, shader.id());
            }
        }

        unsafe {
            gl::LinkProgram(id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe  {
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_initialized_cstring(len as usize);
            unsafe {
                gl::GetProgramInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }

            return Err(Error::LinkError { message: error.to_string_lossy().into_owned() });
        }

        for shader in shaders {
            unsafe {
                gl::DetachShader(id, shader.id());
            }
        }
        Ok(id)
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn get_id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}