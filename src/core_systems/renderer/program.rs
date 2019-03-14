use gl;
use std;
use super::Shader;
use core_systems::renderer::create_initialized_cstring;

pub struct Program {
    id: gl::types::GLuint
}

impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String>{
        let id = unsafe {
            gl::CreateProgram()
        };

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

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe {
                gl::DetachShader(id, shader.id());
            }
        }

        Ok(Program { id })

    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn id(&self) -> gl::types::GLuint {
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