use crate::core_systems::renderer::data::VertexData;
use gl::Gl;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i16_ {
    pub d0: i16,
}

impl i16_ {
    pub fn new(d0: i16) -> i16_ {
        i16_ { d0 }
    }
}

impl VertexData for i16_ {
    unsafe fn vertex_attrib_pointer( gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            1,         // the number of components per generic vertex attribute
            gl::SHORT, // data type
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }

}

impl From<i16> for i16_ {
    fn from(other: i16) -> Self {
        i16_::new(other)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i16_i16 {
    pub d0: i16,
    pub d1: i16,
}

impl i16_i16 {
    pub fn new(d0: i16, d1: i16) -> i16_i16 {
        i16_i16 { d0, d1 }
    }
}

impl VertexData for i16_i16  {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            2,         // the number of components per generic vertex attribute
            gl::SHORT, // data type
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(i16, i16)> for i16_i16 {
    fn from(other: (i16, i16)) -> Self {
        i16_i16::new(other.0, other.1)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i16_i16_i16 {
    pub d0: i16,
    pub d1: i16,
    pub d2: i16,
}

impl i16_i16_i16 {
    pub fn new(d0: i16, d1: i16, d2: i16) -> i16_i16_i16 {
        i16_i16_i16 { d0, d1, d2 }
    }
}

impl VertexData for i16_i16_i16  {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            3,         // the number of components per generic vertex attribute
            gl::SHORT, // data type
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(i16, i16, i16)> for i16_i16_i16 {
    fn from(other: (i16, i16, i16)) -> Self {
        i16_i16_i16::new(other.0, other.1, other.2)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i16_i16_i16_i16 {
    pub d0: i16,
    pub d1: i16,
    pub d2: i16,
    pub d3: i16,
}

impl i16_i16_i16_i16 {
    pub fn new(d0: i16, d1: i16, d2: i16, d3: i16) -> i16_i16_i16_i16 {
        i16_i16_i16_i16 { d0, d1, d2, d3 }
    }
}

impl VertexData for i16_i16_i16_i16 {
     unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            4,         // the number of components per generic vertex attribute
            gl::SHORT, // data type
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(i16, i16, i16, i16)> for i16_i16_i16_i16 {
    fn from(other: (i16, i16, i16, i16)) -> Self {
        i16_i16_i16_i16::new(other.0, other.1, other.2, other.3)
    }
}