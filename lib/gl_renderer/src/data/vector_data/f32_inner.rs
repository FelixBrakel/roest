use serde::{Deserialize, Serialize};
use crate::Program;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct f32_ {
    pub d0: f32,
}

impl f32_ {
    pub fn new(d0: f32) -> f32_ {
        f32_ { d0 }
    }
}

impl f32_ {
    pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribPointer(
            location as gl::types::GLuint,
            1,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }

    pub unsafe fn gl_uniform(&self, location: usize) {
        gl::Uniform1f(
            location as gl::types::GLint,
            self.d0 as gl::types::GLfloat
        );
    }

    pub unsafe fn from_gl_uniform(program: &Program, location: usize) -> Self {
        let mut buf = 0_f32;
        gl::GetUniformfv(
            program.get_id(),
            location as gl::types::GLint,
            &mut buf as *mut gl::types::GLfloat
        );

        return buf.into()
    }
}

impl From<f32> for f32_ {
    fn from(other: f32) -> Self {
        f32_::new(other)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct f32_f32 {
    pub d0: f32,
    pub d1: f32,
}

impl f32_f32 {
    pub fn new(d0: f32, d1: f32) -> f32_f32 {
        f32_f32 { d0, d1 }
    }
}

impl f32_f32 {
    pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribPointer(
            location as gl::types::GLuint,
            2,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }

    pub unsafe fn gl_uniform(&self, location: usize) {
        gl::Uniform2f(
            location as gl::types::GLint,
            self.d0 as gl::types::GLfloat,
            self.d1 as gl::types::GLfloat
        );
    }

    pub unsafe fn from_gl_uniform(program: &Program, location: usize) -> Self {
        let mut buf: Vec<f32> = Vec::with_capacity(2);
        buf.resize(2, 0.);
        gl::GetUniformfv(
            program.get_id(),
            location as gl::types::GLint,
            buf.as_mut_ptr() as *mut gl::types::GLfloat
        );

        return (buf[0], buf[1]).into()
    }
}

impl From<(f32, f32)> for f32_f32 {
    fn from(other: (f32, f32)) -> Self {
        f32_f32::new(other.0, other.1)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct f32_f32_f32 {
    pub d0: f32,
    pub d1: f32,
    pub d2: f32,
}

impl f32_f32_f32 {
    pub fn new(d0: f32, d1: f32, d2: f32) -> f32_f32_f32 {
        f32_f32_f32 { d0, d1, d2 }
    }
}

impl f32_f32_f32 {
    pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribPointer(
            location as gl::types::GLuint,
            3,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }

    pub unsafe fn gl_uniform(&self, location: usize) {
        gl::Uniform3f(
            location as gl::types::GLint,
            self.d0 as gl::types::GLfloat,
            self.d1 as gl::types::GLfloat,
            self.d2 as gl::types::GLfloat
        );
    }

    pub unsafe fn from_gl_uniform(program: &Program, location: usize) -> Self {
        let mut buf: Vec<f32> = Vec::with_capacity(3);
        buf.resize(3, 0.);
        gl::GetUniformfv(
            program.get_id(),
            location as gl::types::GLint,
            buf.as_mut_ptr() as *mut gl::types::GLfloat
        );

        return (buf[0], buf[1], buf[2]).into()
    }
}

impl From<(f32, f32, f32)> for f32_f32_f32 {
    fn from(other: (f32, f32, f32)) -> Self {
        f32_f32_f32::new(other.0, other.1, other.2)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct f32_f32_f32_f32 {
    pub d0: f32,
    pub d1: f32,
    pub d2: f32,
    pub d3: f32,
}

impl f32_f32_f32_f32 {
    pub fn new(d0: f32, d1: f32, d2: f32, d3: f32) -> f32_f32_f32_f32 {
        f32_f32_f32_f32 { d0, d1, d2, d3 }
    }
}

impl f32_f32_f32_f32 {
    pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribPointer(
            location as gl::types::GLuint,
            4,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }

    pub unsafe fn gl_uniform(&self, location: usize) {
        gl::Uniform4f(
            location as gl::types::GLint,
            self.d0 as gl::types::GLfloat,
            self.d1 as gl::types::GLfloat,
            self.d2 as gl::types::GLfloat,
            self.d3 as gl::types::GLfloat
        );
    }

    pub unsafe fn from_gl_uniform(program: &Program, location: usize) -> Self {
        let mut buf: Vec<f32> = Vec::with_capacity(4);
        buf.resize(4, 0.);
        gl::GetUniformfv(
            program.get_id(),
            location as gl::types::GLint,
            buf.as_mut_ptr() as *mut gl::types::GLfloat
        );

        return (buf[0], buf[1], buf[2], buf[3]).into()
    }
}

impl From<(f32, f32, f32, f32)> for f32_f32_f32_f32 {
    fn from(other: (f32, f32, f32, f32)) -> Self {
        f32_f32_f32_f32::new(other.0, other.1, other.2, other.3)
    }
}