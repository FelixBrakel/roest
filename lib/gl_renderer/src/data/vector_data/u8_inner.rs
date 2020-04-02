use serde::{Deserialize, Serialize};
use gl::Gl;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct u8_ {
    pub d0: u8,
}

impl u8_ {
    pub fn new(d0: u8) -> u8_ {
        u8_ { d0 }
    }
}

impl u8_ {
    pub unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            1,                 // the number of components per generic vertex attribute
            gl::UNSIGNED_BYTE, // data type
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<u8> for u8_ {
    fn from(other: u8) -> Self {
        u8_::new(other)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct u8_u8 {
    pub d0: u8,
    pub d1: u8,
}

impl u8_u8 {
    pub fn new(d0: u8, d1: u8) -> u8_u8 {
        u8_u8 { d0, d1 }
    }
}

impl u8_u8 {
    pub unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            2,                 // the number of components per generic vertex attribute
            gl::UNSIGNED_BYTE, // data type
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(u8, u8)> for u8_u8 {
    fn from(other: (u8, u8)) -> Self {
        u8_u8::new(other.0, other.1)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct u8_u8_u8 {
    pub d0: u8,
    pub d1: u8,
    pub d2: u8,
}

impl u8_u8_u8 {
    pub fn new(d0: u8, d1: u8, d2: u8) -> u8_u8_u8 {
        u8_u8_u8 { d0, d1, d2 }
    }
}

impl u8_u8_u8 {
    pub unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            3,                 // the number of components per generic vertex attribute
            gl::UNSIGNED_BYTE, // data type
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(u8, u8, u8)> for u8_u8_u8 {
    fn from(other: (u8, u8, u8)) -> Self {
        u8_u8_u8::new(other.0, other.1, other.2)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct u8_u8_u8_u8 {
    pub d0: u8,
    pub d1: u8,
    pub d2: u8,
    pub d3: u8,
}

impl u8_u8_u8_u8 {
    pub fn new(d0: u8, d1: u8, d2: u8, d3: u8) -> u8_u8_u8_u8 {
        u8_u8_u8_u8 { d0, d1, d2, d3 }
    }
}

impl u8_u8_u8_u8 {
    pub unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            4,                 // the number of components per generic vertex attribute
            gl::UNSIGNED_BYTE, // data type
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(u8, u8, u8, u8)> for u8_u8_u8_u8 {
    fn from(other: (u8, u8, u8, u8)) -> Self {
        u8_u8_u8_u8::new(other.0, other.1, other.2, other.3)
    }
}
