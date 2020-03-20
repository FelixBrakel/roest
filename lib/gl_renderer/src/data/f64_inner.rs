use gl::Gl;
use super::VertexData;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f64_ {
    pub d0: f64,
}

impl f64_ {
    pub fn new(d0: f64) -> f64_ {
        f64_ { d0 }
    }
}

impl VertexData for f64_ {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribLPointer(
            location as gl::types::GLuint,
            1,          // the number of components per generic vertex attribute
            gl::DOUBLE, // data type
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<f64> for f64_ {
    fn from(other: f64) -> Self {
        f64_::new(other)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f64_f64 {
    pub d0: f64,
    pub d1: f64,
}

impl f64_f64 {
    pub fn new(d0: f64, d1: f64) -> f64_f64 {
        f64_f64 { d0, d1 }
    }
}

impl VertexData for f64_f64 {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribLPointer(
            location as gl::types::GLuint,
            2,          // the number of components per generic vertex attribute
            gl::DOUBLE, // data type
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(f64, f64)> for f64_f64 {
    fn from(other: (f64, f64)) -> Self {
        f64_f64::new(other.0, other.1)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f64_f64_f64 {
    pub d0: f64,
    pub d1: f64,
    pub d2: f64,
}

impl f64_f64_f64 {
    pub fn new(d0: f64, d1: f64, d2: f64) -> f64_f64_f64 {
        f64_f64_f64 { d0, d1, d2 }
    }
}

impl VertexData for f64_f64_f64 {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribLPointer(
            location as gl::types::GLuint,
            3,          // the number of components per generic vertex attribute
            gl::DOUBLE, // data type
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(f64, f64, f64)> for f64_f64_f64 {
    fn from(other: (f64, f64, f64)) -> Self {
        f64_f64_f64::new(other.0, other.1, other.2)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f64_f64_f64_f64 {
    pub d0: f64,
    pub d1: f64,
    pub d2: f64,
    pub d3: f64,
}

impl f64_f64_f64_f64 {
    pub fn new(d0: f64, d1: f64, d2: f64, d3: f64) -> f64_f64_f64_f64 {
        f64_f64_f64_f64 { d0, d1, d2, d3 }
    }
}

impl VertexData for f64_f64_f64_f64 {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribLPointer(
            location as gl::types::GLuint,
            4,          // the number of components per generic vertex attribute
            gl::DOUBLE, // data type
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(f64, f64, f64, f64)> for f64_f64_f64_f64 {
    fn from(other: (f64, f64, f64, f64)) -> Self {
        f64_f64_f64_f64::new(other.0, other.1, other.2, other.3)
    }
}
