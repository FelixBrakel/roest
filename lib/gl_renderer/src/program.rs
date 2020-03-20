use gl;
use super::{Shader};
use failure::Fail;
use super::{create_initialized_cstring};

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failed to link program: {}", message)]
    LinkError { message: String },
}

pub struct Program {
    gl: gl::Gl,
    id: gl::types::GLuint,
}


impl Program {
    pub fn load_shaders(gl: gl::Gl, shaders: &[Shader]) -> Result<Self, Error> {
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

            return Err(Error::LinkError { message: error.to_string_lossy().into_owned() });
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

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.id);
        }
    }
}