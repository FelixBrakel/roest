use gl::Gl;
use crate::data::VertexData;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f16_ {
    pub d0: ::half::f16,
}

impl f16_ {
    pub fn new(d0: ::half::f16) -> f16_ {
        f16_ { d0 }
    }
}

impl VertexData for f16_ {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            1,              // the number of components per generic vertex attribute
            gl::HALF_FLOAT, // data type
            gl::FALSE,      // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<::half::f16> for f16_ {
    fn from(other: ::half::f16) -> Self {
        f16_::new(other)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f16_f16 {
    pub d0: ::half::f16,
    pub d1: ::half::f16,
}

impl f16_f16 {
    pub fn new(d0: ::half::f16, d1: ::half::f16) -> f16_f16 {
        f16_f16 { d0, d1 }
    }
}

impl VertexData for f16_f16 {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            2,              // the number of components per generic vertex attribute
            gl::HALF_FLOAT, // data type
            gl::FALSE,      // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(::half::f16, ::half::f16)> for f16_f16 {
    fn from(other: (::half::f16, ::half::f16)) -> Self {
        f16_f16::new(other.0, other.1)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f16_f16_f16 {
    pub d0: ::half::f16,
    pub d1: ::half::f16,
    pub d2: ::half::f16,
}

impl f16_f16_f16 {
    pub fn new(d0: ::half::f16, d1: ::half::f16, d2: ::half::f16) -> f16_f16_f16 {
        f16_f16_f16 { d0, d1, d2 }
    }
}

impl VertexData for f16_f16_f16 {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            3,              // the number of components per generic vertex attribute
            gl::HALF_FLOAT, // data type
            gl::FALSE,      // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(::half::f16, ::half::f16, ::half::f16)> for f16_f16_f16 {
    fn from(other: (::half::f16, ::half::f16, ::half::f16)) -> Self {
        f16_f16_f16::new(other.0, other.1, other.2)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f16_f16_f16_f16 {
    pub d0: ::half::f16,
    pub d1: ::half::f16,
    pub d2: ::half::f16,
    pub d3: ::half::f16,
}

impl f16_f16_f16_f16 {
    pub fn new(
        d0: ::half::f16,
        d1: ::half::f16,
        d2: ::half::f16,
        d3: ::half::f16,
    ) -> f16_f16_f16_f16 {
        f16_f16_f16_f16 { d0, d1, d2, d3 }
    }
}

impl VertexData for f16_f16_f16_f16 {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            4,              // the number of components per generic vertex attribute
            gl::HALF_FLOAT, // data type
            gl::FALSE,      // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(::half::f16, ::half::f16, ::half::f16, ::half::f16)> for f16_f16_f16_f16 {
    fn from(other: (::half::f16, ::half::f16, ::half::f16, ::half::f16)) -> Self {
        f16_f16_f16_f16::new(other.0, other.1, other.2, other.3)
    }
}