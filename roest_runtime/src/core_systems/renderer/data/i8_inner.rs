use crate::core_systems::renderer::data::VertexData;
use gl::Gl;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i8_ {
    pub d0: i8,
}

impl i8_{
    fn new(d0: i8) -> i8_ {
        i8_ { d0 }
    }
}

impl VertexData for i8_ {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            1,
            gl::BYTE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid
        );
    }
}

impl From<i8> for i8_ {
    fn from(other: i8) -> i8_ {
        i8_::new(other)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i8_i8 {
    pub d0: i8,
    pub d1: i8,
}

impl i8_i8 {
    fn new(d0: i8, d1: i8) -> i8_i8 {
        i8_i8 { d0, d1 }
    }
}

impl VertexData for i8_i8 {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            2,
            gl::BYTE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid
        );
    }
}

impl From<(i8, i8)> for i8_i8 {
    fn from(other: (i8, i8)) -> i8_i8 {
        i8_i8::new(other.0, other.1)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i8_i8_i8 {
    pub d0: i8,
    pub d1: i8,
    pub d2: i8,
}

impl i8_i8_i8 {
    fn new(d0: i8, d1: i8, d2: i8) -> i8_i8_i8 {
        i8_i8_i8 { d0, d1, d2 }
    }
}

impl VertexData for i8_i8_i8 {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            3,
            gl::BYTE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid
        );
    }
}

impl From<(i8, i8, i8)> for i8_i8_i8 {
    fn from(other: (i8, i8, i8)) -> i8_i8_i8 {
        i8_i8_i8::new(other.0, other.1, other.2)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i8_i8_i8_i8 {
    pub d0: i8,
    pub d1: i8,
    pub d2: i8,
    pub d3: i8,
}

impl i8_i8_i8_i8 {
    fn new(d0: i8, d1: i8, d2: i8, d3: i8) -> i8_i8_i8_i8 {
        i8_i8_i8_i8 { d0, d1, d2, d3 }
    }
}

impl VertexData for i8_i8_i8_i8 {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            4,
            gl::BYTE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid
        );
    }
}

impl From<(i8, i8, i8, i8)> for i8_i8_i8_i8 {
    fn from(other: (i8, i8, i8, i8)) -> i8_i8_i8_i8 {
        i8_i8_i8_i8::new(other.0, other.1, other.2, other.3)
    }
}