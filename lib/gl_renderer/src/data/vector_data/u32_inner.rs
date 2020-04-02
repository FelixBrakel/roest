use serde::{Deserialize, Serialize};
use gl::Gl;
use crate::Program;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct u32_ {
    pub d0: u32,
}

impl u32_ {
    pub fn new(d0: u32) -> u32_ {
        u32_ { d0 }
    }
}

impl u32_ {
    pub unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            1,                // the number of components per generic vertex attribute
            gl::UNSIGNED_INT, // data type
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }

    pub unsafe fn gl_uniform(&self, gl: &Gl, location: usize) {
        gl.Uniform1ui(
            location as gl::types::GLint,
            self.d0 as gl::types::GLuint
        );
    }

    pub unsafe fn gl_get_uniform(gl: &Gl, program: &Program, location: usize) -> Self {
        let mut buf: u32 = 0;
        gl.GetUniformuiv(
            program.get_id(),
            location as gl::types::GLint,
            &mut buf as *mut gl::types::GLuint
        );

        return buf.into()
    }
}

impl From<u32> for u32_ {
    fn from(other: u32) -> Self {
        u32_::new(other)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct u32_u32 {
    pub d0: u32,
    pub d1: u32,
}

impl u32_u32 {
    pub fn new(d0: u32, d1: u32) -> u32_u32 {
        u32_u32 { d0, d1 }
    }
}

impl u32_u32 {
    pub unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            2,                // the number of components per generic vertex attribute
            gl::UNSIGNED_INT, // data type
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }

    pub unsafe fn gl_uniform(&self, gl: &Gl, location: usize) {
        gl.Uniform2ui(
            location as gl::types::GLint,
            self.d0 as gl::types::GLuint,
            self.d1 as gl::types::GLuint
        );
    }

    pub unsafe fn gl_get_uniform(gl: &Gl, program: &Program, location: usize) -> Self {
        let mut buf: Vec<u32> = Vec::with_capacity(2);
        gl.GetUniformuiv(
            program.get_id(),
            location as gl::types::GLint,
            buf.as_ptr() as *mut gl::types::GLuint
        );

        return (buf[0], buf[1]).into()
    }
}

impl From<(u32, u32)> for u32_u32 {
    fn from(other: (u32, u32)) -> Self {
        u32_u32::new(other.0, other.1)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct u32_u32_u32 {
    pub d0: u32,
    pub d1: u32,
    pub d2: u32,
}

impl u32_u32_u32 {
    pub fn new(d0: u32, d1: u32, d2: u32) -> u32_u32_u32 {
        u32_u32_u32 { d0, d1, d2 }
    }
}

impl u32_u32_u32 {
    pub unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            3,                // the number of components per generic vertex attribute
            gl::UNSIGNED_INT, // data type
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }

    pub unsafe fn gl_uniform(&self, gl: &Gl, location: usize) {
        gl.Uniform3ui(
            location as gl::types::GLint,
            self.d0 as gl::types::GLuint,
            self.d1 as gl::types::GLuint,
            self.d2 as gl::types::GLuint
        );
    }

    pub unsafe fn gl_get_uniform(gl: &Gl, program: &Program, location: usize) -> Self {
        let mut buf: Vec<u32> = Vec::with_capacity(3);
        gl.GetUniformuiv(
            program.get_id(),
            location as gl::types::GLint,
            buf.as_ptr() as *mut gl::types::GLuint
        );

        return (buf[0], buf[1], buf[2]).into()
    }
}

impl From<(u32, u32, u32)> for u32_u32_u32 {
    fn from(other: (u32, u32, u32)) -> Self {
        u32_u32_u32::new(other.0, other.1, other.2)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct u32_u32_u32_u32 {
    pub d0: u32,
    pub d1: u32,
    pub d2: u32,
    pub d3: u32,
}

impl u32_u32_u32_u32 {
    pub fn new(d0: u32, d1: u32, d2: u32, d3: u32) -> u32_u32_u32_u32 {
        u32_u32_u32_u32 { d0, d1, d2, d3 }
    }
}

impl u32_u32_u32_u32 {
    pub unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            4,                // the number of components per generic vertex attribute
            gl::UNSIGNED_INT, // data type
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
    pub unsafe fn gl_uniform(&self, gl: &Gl, location: usize) {
        gl.Uniform4ui(
            location as gl::types::GLint,
            self.d0 as gl::types::GLuint,
            self.d1 as gl::types::GLuint,
            self.d2 as gl::types::GLuint,
            self.d3 as gl::types::GLuint,
        );
    }

    pub unsafe fn gl_get_uniform(gl: &Gl, program: &Program, location: usize) -> Self {
        let mut buf: Vec<u32> = Vec::with_capacity(4);
        gl.GetUniformuiv(
            program.get_id(),
            location as gl::types::GLint,
            buf.as_ptr() as *mut gl::types::GLuint
        );

        return (buf[0], buf[1], buf[2], buf[3]).into()
    }
}

impl From<(u32, u32, u32, u32)> for u32_u32_u32_u32 {
    fn from(other: (u32, u32, u32, u32)) -> Self {
        u32_u32_u32_u32::new(other.0, other.1, other.2, other.3)
    }
}