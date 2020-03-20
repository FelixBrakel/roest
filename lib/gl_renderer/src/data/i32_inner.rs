use gl::Gl;
use super::VertexData;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i32_ {
    pub d0: i32,
}

impl i32_ {
    pub fn new(d0: i32) -> i32_ {
        i32_ { d0 }
    }
}

impl VertexData for i32_ {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            1,       // the number of components per generic vertex attribute
            gl::INT, // data type
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<i32> for i32_ {
    fn from(other: i32) -> Self {
        i32_::new(other)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i32_i32 {
    pub d0: i32,
    pub d1: i32,
}

impl i32_i32 {
    pub fn new(d0: i32, d1: i32) -> i32_i32 {
        i32_i32 { d0, d1 }
    }
}

impl VertexData for i32_i32 {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            2,       // the number of components per generic vertex attribute
            gl::INT, // data type
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(i32, i32)> for i32_i32 {
    fn from(other: (i32, i32)) -> Self {
        i32_i32::new(other.0, other.1)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i32_i32_i32 {
    pub d0: i32,
    pub d1: i32,
    pub d2: i32,
}

impl i32_i32_i32 {
    pub fn new(d0: i32, d1: i32, d2: i32) -> i32_i32_i32 {
        i32_i32_i32 { d0, d1, d2 }
    }
}

impl VertexData for i32_i32_i32 {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            3,       // the number of components per generic vertex attribute
            gl::INT, // data type
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(i32, i32, i32)> for i32_i32_i32 {
    fn from(other: (i32, i32, i32)) -> Self {
        i32_i32_i32::new(other.0, other.1, other.2)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i32_i32_i32_i32 {
    pub d0: i32,
    pub d1: i32,
    pub d2: i32,
    pub d3: i32,
}

impl i32_i32_i32_i32 {
    pub fn new(d0: i32, d1: i32, d2: i32, d3: i32) -> i32_i32_i32_i32 {
        i32_i32_i32_i32 { d0, d1, d2, d3 }
    }
}

impl VertexData for i32_i32_i32_i32 {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            4,       // the number of components per generic vertex attribute
            gl::INT, // data type
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(i32, i32, i32, i32)> for i32_i32_i32_i32 {
    fn from(other: (i32, i32, i32, i32)) -> Self {
        i32_i32_i32_i32::new(other.0, other.1, other.2, other.3)
    }
}