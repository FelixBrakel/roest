use gl::Gl;
use crate::data::VertexData;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u16_float {
    pub d0: u16,
}

impl u16_float {
    pub fn new(d0: u16) -> u16_float {
        u16_float { d0 }
    }
}

impl VertexData for u16_float {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            1,                  // the number of components per generic vertex attribute
            gl::UNSIGNED_SHORT, // data type
            gl::TRUE,           // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<u16> for u16_float {
    fn from(other: u16) -> Self {
        u16_float::new(other)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u16_u16_float {
    pub d0: u16,
    pub d1: u16,
}

impl u16_u16_float {
    pub fn new(d0: u16, d1: u16) -> u16_u16_float {
        u16_u16_float { d0, d1 }
    }
}

impl VertexData for u16_u16_float {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            2,                  // the number of components per generic vertex attribute
            gl::UNSIGNED_SHORT, // data type
            gl::TRUE,           // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(u16, u16)> for u16_u16_float {
    fn from(other: (u16, u16)) -> Self {
        u16_u16_float::new(other.0, other.1)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u16_u16_u16_float {
    pub d0: u16,
    pub d1: u16,
    pub d2: u16,
}

impl u16_u16_u16_float {
    pub fn new(d0: u16, d1: u16, d2: u16) -> u16_u16_u16_float {
        u16_u16_u16_float { d0, d1, d2 }
    }
}

impl VertexData for u16_u16_u16_float {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            3,                  // the number of components per generic vertex attribute
            gl::UNSIGNED_SHORT, // data type
            gl::TRUE,           // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(u16, u16, u16)> for u16_u16_u16_float {
    fn from(other: (u16, u16, u16)) -> Self {
        u16_u16_u16_float::new(other.0, other.1, other.2)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u16_u16_u16_u16_float {
    pub d0: u16,
    pub d1: u16,
    pub d2: u16,
    pub d3: u16,
}

impl u16_u16_u16_u16_float {
    pub fn new(d0: u16, d1: u16, d2: u16, d3: u16) -> u16_u16_u16_u16_float {
        u16_u16_u16_u16_float { d0, d1, d2, d3 }
    }
}

impl VertexData for u16_u16_u16_u16_float {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            4,                  // the number of components per generic vertex attribute
            gl::UNSIGNED_SHORT, // data type
            gl::TRUE,           // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(u16, u16, u16, u16)> for u16_u16_u16_u16_float {
    fn from(other: (u16, u16, u16, u16)) -> Self {
        u16_u16_u16_u16_float::new(other.0, other.1, other.2, other.3)
    }
}
