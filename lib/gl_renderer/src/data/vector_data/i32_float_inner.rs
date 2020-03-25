use gl::Gl;
use crate::data::VertexData;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i32_float {
    pub d0: i32,
}

impl i32_float {
    pub fn new(d0: i32) -> i32_float {
        i32_float { d0 }
    }
}

impl VertexData for i32_float {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            1,        // the number of components per generic vertex attribute
            gl::INT,  // data type
            gl::TRUE, // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<i32> for i32_float {
    fn from(other: i32) -> Self {
        i32_float::new(other)
    }
}

// -----------------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i32_i32_float {
    pub d0: i32,
    pub d1: i32,
}

impl i32_i32_float {
    pub fn new(d0: i32, d1: i32) -> i32_i32_float {
        i32_i32_float { d0, d1 }
    }
}

impl VertexData for i32_i32_float {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            2,        // the number of components per generic vertex attribute
            gl::INT,  // data type
            gl::TRUE, // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(i32, i32)> for i32_i32_float {
    fn from(other: (i32, i32)) -> Self {
        i32_i32_float::new(other.0, other.1)
    }
}

// -----------------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i32_i32_i32_float {
    pub d0: i32,
    pub d1: i32,
    pub d2: i32,
}

impl i32_i32_i32_float {
    pub fn new(d0: i32, d1: i32, d2: i32) -> i32_i32_i32_float {
        i32_i32_i32_float { d0, d1, d2 }
    }
}

impl VertexData for i32_i32_i32_float {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            3,        // the number of components per generic vertex attribute
            gl::INT,  // data type
            gl::TRUE, // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(i32, i32, i32)> for i32_i32_i32_float {
    fn from(other: (i32, i32, i32)) -> Self {
        i32_i32_i32_float::new(other.0, other.1, other.2)
    }
}

// -----------------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i32_i32_i32_i32_float {
    pub d0: i32,
    pub d1: i32,
    pub d2: i32,
    pub d3: i32,
}

impl i32_i32_i32_i32_float {
    pub fn new(d0: i32, d1: i32, d2: i32, d3: i32) -> i32_i32_i32_i32_float {
        i32_i32_i32_i32_float { d0, d1, d2, d3 }
    }
}

impl VertexData for i32_i32_i32_i32_float {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            4,        // the number of components per generic vertex attribute
            gl::INT,  // data type
            gl::TRUE, // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(i32, i32, i32, i32)> for i32_i32_i32_i32_float {
    fn from(other: (i32, i32, i32, i32)) -> Self {
        i32_i32_i32_i32_float::new(other.0, other.1, other.2, other.3)
    }
}
