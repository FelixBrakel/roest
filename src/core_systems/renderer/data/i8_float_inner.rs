use crate::core_systems::renderer::data::VertexData;
use gl::Gl;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i8_float {
    pub d0: i8,
}

impl i8_float {
    pub fn new(d0: i8) -> i8_float {
        i8_float { d0 }
    }
}

impl VertexData for i8_float {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            1,
            gl::BYTE,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid
        );
    }
}

impl From<i8> for i8_float {
    fn from(other: i8) -> i8_float {
        i8_float::new(other)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i8_i8_float {
    pub d0: i8,
    pub d1: i8,
}

impl i8_i8_float {
    fn new(d0: i8, d1: i8) -> i8_i8_float {
        i8_i8_float { d0, d1 }
    }
}

impl VertexData for i8_i8_float {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            2,
            gl::BYTE,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid
        );
    }
}

impl From<(i8, i8)> for i8_i8_float {
    fn from(other: (i8, i8)) -> i8_i8_float {
        i8_i8_float::new(other.0, other.1)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i8_i8_i8_float {
    pub d0: i8,
    pub d1: i8,
    pub d2: i8,
}

impl i8_i8_i8_float {
    fn new(d0: i8, d1: i8, d2: i8) -> i8_i8_i8_float {
        i8_i8_i8_float { d0, d1, d2 }
    }
}

impl VertexData for i8_i8_i8_float {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            3,
            gl::BYTE,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid
        );
    }
}

impl From<(i8, i8, i8)> for i8_i8_i8_float {
    fn from(other: (i8, i8, i8)) -> i8_i8_i8_float {
        i8_i8_i8_float::new(other.0, other.1, other.2)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i8_i8_i8_i8_float {
    pub d0: i8,
    pub d1: i8,
    pub d2: i8,
    pub d3: i8,
}

impl i8_i8_i8_i8_float {
    fn new(d0: i8, d1: i8, d2: i8, d3: i8) -> i8_i8_i8_i8_float {
        i8_i8_i8_i8_float { d0, d1, d2, d3 }
    }
}

impl VertexData for i8_i8_i8_i8_float {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            4,
            gl::BYTE,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid
        );
    }
}

impl From<(i8, i8, i8, i8)> for i8_i8_i8_i8_float {
    fn from(other: (i8, i8, i8, i8)) -> i8_i8_i8_i8_float {
        i8_i8_i8_i8_float::new(other.0, other.1, other.2, other.3)
    }
}