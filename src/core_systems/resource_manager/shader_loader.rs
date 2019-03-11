use gl;
use std;
use std::fs;
use std::path::Path;
use std::ffi::CString;


/// Create a shader from a file.
/// TODO: Create a fallback shader to be used if an error occurs during the shader creation.
fn shader_from_file<P: AsRef<Path>>(path_str: P) -> Result<gl::types::GLuint, String> {
    let path = path_str.as_ref();
    let shader_src = fs::read_to_string(path);
    let fallback_shader = "";
    let kind = match path.extension() {
        Some(v) => {
            if let v = "frag" {
                gl::FRAGMENT_SHADER
            } else {
                gl::VERTEX_SHADER
            }
        },
        None => {
            return Err(String::from("error in determining file extension"));
        },
    };

    match shader_src {
        Ok(v) => shader_from_source(&v, kind),
        Err(e) => shader_from_source(fallback_shader, kind),
    }
}

fn shader_from_source(source: &str, kind: gl::types::GLenum) -> Result<gl::types::GLuint, String> {
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
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut len);
        }

        let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
        buffer.extend([b' '].iter().cycle().take(len as usize));

        let error: CString = unsafe {
            CString::from_vec_unchecked(buffer)
        };

        unsafe {
            gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut gl::types::GLchar);
        }

        Err(error.to_string_lossy().into_owned())
    }

    Ok(id)
}