use gl::Gl;
use crate::data::VertexData;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u8_float {
    pub d0: u8,
}

impl u8_float {
    pub fn new(d0: u8) -> u8_float {
        u8_float { d0 }
    }
}

impl VertexData for u8_float {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            1,                 // the number of components per generic vertex attribute
            gl::UNSIGNED_BYTE, // data type
            gl::TRUE,          // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<u8> for u8_float {
    fn from(other: u8) -> Self {
        u8_float::new(other)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u8_u8_float {
    pub d0: u8,
    pub d1: u8,
}

impl u8_u8_float {
    pub fn new(d0: u8, d1: u8) -> u8_u8_float {
        u8_u8_float { d0, d1 }
    }
}

impl VertexData for u8_u8_float {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            2,                 // the number of components per generic vertex attribute
            gl::UNSIGNED_BYTE, // data type
            gl::TRUE,          // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(u8, u8)> for u8_u8_float {
    fn from(other: (u8, u8)) -> Self {
        u8_u8_float::new(other.0, other.1)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u8_u8_u8_float {
    pub d0: u8,
    pub d1: u8,
    pub d2: u8,
}

impl u8_u8_u8_float {
    pub fn new(d0: u8, d1: u8, d2: u8) -> u8_u8_u8_float {
        u8_u8_u8_float { d0, d1, d2 }
    }
}

impl VertexData for u8_u8_u8_float {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            3,                 // the number of components per generic vertex attribute
            gl::UNSIGNED_BYTE, // data type
            gl::TRUE,          // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(u8, u8, u8)> for u8_u8_u8_float {
    fn from(other: (u8, u8, u8)) -> Self {
        u8_u8_u8_float::new(other.0, other.1, other.2)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u8_u8_u8_u8_float {
    pub d0: u8,
    pub d1: u8,
    pub d2: u8,
    pub d3: u8,
}

impl u8_u8_u8_u8_float {
    pub fn new(d0: u8, d1: u8, d2: u8, d3: u8) -> u8_u8_u8_u8_float {
        u8_u8_u8_u8_float { d0, d1, d2, d3 }
    }
}

impl VertexData for u8_u8_u8_u8_float {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            4,                 // the number of components per generic vertex attribute
            gl::UNSIGNED_BYTE, // data type
            gl::TRUE,          // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(u8, u8, u8, u8)> for u8_u8_u8_u8_float {
    fn from(other: (u8, u8, u8, u8)) -> Self {
        u8_u8_u8_u8_float::new(other.0, other.1, other.2, other.3)
    }
}