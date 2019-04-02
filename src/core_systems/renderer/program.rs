use gl;
use std;
use super::Shader;
use core_systems::renderer::create_initialized_cstring;
use std::path::{PathBuf, Path};

pub struct Program {
    gl: gl::Gl,
    id: gl::types::GLuint,
}

impl Program {
    pub fn from_shaders(gl: &gl::Gl, shaders: &[Shader]) -> Result<Program, String>{
        let id = unsafe {
            gl.CreateProgram()
        };

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

        Ok(Program { gl: gl.clone(), id })
    }

    pub fn from_res(gl: &gl::Gl, name: &str) -> Result<Program, String> {
        const POSSIBLE_EXT: [&str; 2] = [
            ".vert",
            ".frag",
        ];

        let shaders = POSSIBLE_EXT.iter()
            .map(|file_extension| {
                Shader::from_relative_root_path(gl, &format!("{}{}", name, file_extension))
            })
            .collect::<Result<Vec<Shader>, String>>()?;

        Program::from_shaders(gl, &shaders[..])

    }

    pub fn set_used(&self) {
        unsafe {
            self.gl.UseProgram(self.id);
        }
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.id);
        }
    }
}