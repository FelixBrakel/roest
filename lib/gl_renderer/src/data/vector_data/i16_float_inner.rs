use serde::{Deserialize, Serialize};
use gl::Gl;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct i16_float {
    pub d0: i16,
}

impl i16_float {
    pub fn new(d0: i16) -> i16_float {
        i16_float { d0 }
    }
}

impl i16_float {
    pub unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            1,         // the number of components per generic vertex attribute
            gl::SHORT, // data type
            gl::TRUE,  // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<i16> for i16_float {
    fn from(other: i16) -> Self {
        i16_float::new(other)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct i16_i16_float {
    pub d0: i16,
    pub d1: i16,
}

impl i16_i16_float {
    pub fn new(d0: i16, d1: i16) -> i16_i16_float {
        i16_i16_float { d0, d1 }
    }
}

impl i16_i16_float {
    pub unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            2,         // the number of components per generic vertex attribute
            gl::SHORT, // data type
            gl::TRUE,  // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(i16, i16)> for i16_i16_float {
    fn from(other: (i16, i16)) -> Self {
        i16_i16_float::new(other.0, other.1)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct i16_i16_i16_float {
    pub d0: i16,
    pub d1: i16,
    pub d2: i16,
}

impl i16_i16_i16_float {
    pub fn new(d0: i16, d1: i16, d2: i16) -> i16_i16_i16_float {
        i16_i16_i16_float { d0, d1, d2 }
    }
}

impl i16_i16_i16_float {
    pub unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            3,         // the number of components per generic vertex attribute
            gl::SHORT, // data type
            gl::TRUE,  // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(i16, i16, i16)> for i16_i16_i16_float {
    fn from(other: (i16, i16, i16)) -> Self {
        i16_i16_i16_float::new(other.0, other.1, other.2)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct i16_i16_i16_i16_float {
    pub d0: i16,
    pub d1: i16,
    pub d2: i16,
    pub d3: i16,
}

impl i16_i16_i16_i16_float {
    pub fn new(d0: i16, d1: i16, d2: i16, d3: i16) -> i16_i16_i16_i16_float {
        i16_i16_i16_i16_float { d0, d1, d2, d3 }
    }
}

impl i16_i16_i16_i16_float {
    pub unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            4,         // the number of components per generic vertex attribute
            gl::SHORT, // data type
            gl::TRUE,  // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(i16, i16, i16, i16)> for i16_i16_i16_i16_float {
    fn from(other: (i16, i16, i16, i16)) -> Self {
        i16_i16_i16_i16_float::new(other.0, other.1, other.2, other.3)
    }
}