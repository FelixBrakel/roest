use gl;
use std;
use super::Shader;
use core_systems::renderer::create_initialized_cstring;
use std::path::{Path,};
use core_systems::resource_manager::Resource;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failed to link program {}: {}", name, message)]
    LinkError {name: String, message: String},
}

pub struct Program {
    gl: gl::Gl,
    id: gl::types::GLuint,
}

impl Program {
    fn load_shaders(gl: &gl::Gl, shaders: &[Shader]) -> Result<Self, String>{
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
        Ok(Program { gl: gl.clone(), id: id })
    }

    pub fn set_used(&self) {
        unsafe {
            self.gl.UseProgram(self.id);
        }
    }
}

impl Resource for Program {
    fn load(gl: &gl::Gl, name: &AsRef<Path>) -> Result<Program, Error> {
        const POSSIBLE_EXT: [&str; 2] = [
            ".vert",
            ".frag",
        ];

        let shaders = POSSIBLE_EXT.iter()
            .map(|file_extension| {
                Shader::from_relative_root_path(gl, &format!("{}{}", name, file_extension))
            })
            .collect::<Result<Vec<Shader>, String>>()?;

        Self::load_shaders(gl, &shaders[..])
            .map_err(|message| Error::LinkError {name: name.into(), message })
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.id);
        }
    }
}