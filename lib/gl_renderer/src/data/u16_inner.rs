use gl::Gl;
use super::VertexData;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u16_ {
    pub d0: u16,
}

impl u16_ {
    pub fn new(d0: u16) -> u16_ {
        u16_ { d0 }
    }
}

impl VertexData for u16_ {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            1,                  // the number of components per generic vertex attribute
            gl::UNSIGNED_SHORT, // data type
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<u16> for u16_ {
    fn from(other: u16) -> Self {
        u16_::new(other)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u16_u16 {
    pub d0: u16,
    pub d1: u16,
}

impl u16_u16 {
    pub fn new(d0: u16, d1: u16) -> u16_u16 {
        u16_u16 { d0, d1 }
    }
}

impl VertexData for u16_u16 {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            2,                  // the number of components per generic vertex attribute
            gl::UNSIGNED_SHORT, // data type
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(u16, u16)> for u16_u16 {
    fn from(other: (u16, u16)) -> Self {
        u16_u16::new(other.0, other.1)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u16_u16_u16 {
    pub d0: u16,
    pub d1: u16,
    pub d2: u16,
}

impl u16_u16_u16 {
    pub fn new(d0: u16, d1: u16, d2: u16) -> u16_u16_u16 {
        u16_u16_u16 { d0, d1, d2 }
    }
}

impl VertexData for u16_u16_u16 {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            3,                  // the number of components per generic vertex attribute
            gl::UNSIGNED_SHORT, // data type
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(u16, u16, u16)> for u16_u16_u16 {
    fn from(other: (u16, u16, u16)) -> Self {
        u16_u16_u16::new(other.0, other.1, other.2)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u16_u16_u16_u16 {
    pub d0: u16,
    pub d1: u16,
    pub d2: u16,
    pub d3: u16,
}

impl u16_u16_u16_u16 {
    pub fn new(d0: u16, d1: u16, d2: u16, d3: u16) -> u16_u16_u16_u16 {
        u16_u16_u16_u16 { d0, d1, d2, d3 }
    }
}

impl VertexData for u16_u16_u16_u16 {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            4,                  // the number of components per generic vertex attribute
            gl::UNSIGNED_SHORT, // data type
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(u16, u16, u16, u16)> for u16_u16_u16_u16 {
    fn from(other: (u16, u16, u16, u16)) -> Self {
        u16_u16_u16_u16::new(other.0, other.1, other.2, other.3)
    }
}