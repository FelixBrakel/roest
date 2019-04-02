use gl;
use std;
use std::path::{Path, PathBuf};
use std::ffi::CStr;
use core_systems::file_system as fs;
use core_systems::renderer::create_initialized_cstring;
use core_systems::resource_manager::{Error as ResError, Resource};
use core_systems::resource_manager;
pub struct Shader {
    gl: gl::Gl,
    id: gl::types::GLuint,
}

impl Shader {
    pub fn from_source(gl: &gl::Gl, source: &CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
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

        Ok(Shader { gl: gl.clone(),  id })
    }

    /// Create a shader from a file.
    /// TODO: Create a fallback shader to be used if an error occurs during the shader creation.
    pub fn from_res(gl: &gl::Gl, res: &Resource, name: &str) -> Result<Shader, String>{
        let shader_src = match fs::synchronous::read_to_cstring(&filepath) {
            Ok(src) => src,
            Err(err) => return Err(String::from("NulError"))
        };

        let path = filepath.as_ref();
        let kind = match path.extension() {
            Some(v) => {
                match v.to_str() {
                    Some("frag") => gl::FRAGMENT_SHADER,
                    Some("vert") => gl::VERTEX_SHADER,
                    _ => return Err(String::from("Could not match file extension"))
                }
            },
            None => {
                return Err(String::from("error in determining file extension"));
            },
        };
        Shader::from_source(gl, &shader_src, kind)
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Resource for Shader {
    //TODO don't hardcode shader root folder
    const ROOT_PATH: PathBuf = {
        let bin_file_name = ::std::env::current_exe().map_err(
            |_| ResError::FailedToGetBinPath
        )?;

        bin_file_name.parent().ok_or(ResError::FailedToGetBinPath).join("resources/shaders")
    };

    fn from_relative_root_path<P: AsRef<Path>>(rel_path: P) -> Result<Resource, ResError> {

    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.id);
        }
    }
}