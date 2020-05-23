use serde::{Deserialize, Serialize};
use crate::{Program, GPUVariant, buffered_uniform_struct_shared::{GPUBasic, GPUBasicArray}};

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize, Default)]
#[repr(C, packed)]
pub struct i32_ {
    pub d0: i32,
}

impl GPUVariant for i32_ {
    type Variant = GPUBasic<i32_>;
    type ArrayVariant = GPUBasicArray<i32_>;
}

impl i32_ {
    pub fn new(d0: i32) -> i32_ {
        i32_ { d0 }
    }
}

impl i32_ {
    pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribIPointer(
            location as gl::types::GLuint,
            1,       // the number of components per generic vertex attribute
            gl::INT, // data type
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }

    pub unsafe fn gl_uniform(&self, location: usize) {
        gl::Uniform1i(
            location as gl::types::GLint,
            self.d0 as gl::types::GLint
        );
    }

    pub unsafe fn gl_get_uniform(program: &Program, location: usize) -> Self {
        let mut buf= 0;
        gl::GetUniformiv(
            program.get_id(),
            location as gl::types::GLint,
            &mut buf as *mut gl::types::GLint
        );

        return buf.into()
    }
}

impl From<i32> for i32_ {
    fn from(other: i32) -> Self {
        i32_::new(other)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize, Default)]
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

impl i32_i32 {
    pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribIPointer(
            location as gl::types::GLuint,
            2,       // the number of components per generic vertex attribute
            gl::INT, // data type
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }

    pub unsafe fn gl_uniform(&self, location: usize) {
        gl::Uniform2i(
            location as gl::types::GLint,
            self.d0 as gl::types::GLint,
            self.d1 as gl::types::GLint
        );
    }

    pub unsafe fn gl_get_uniform(program: &Program, location: usize) -> Self {
        let mut buf: Vec<i32> = Vec::with_capacity(2);
        buf.resize(2, 0);
        gl::GetUniformiv(
            program.get_id(),
            location as gl::types::GLint,
            buf.as_mut_ptr() as *mut gl::types::GLint
        );

        return (buf[0], buf[1]).into()
    }
}

impl From<(i32, i32)> for i32_i32 {
    fn from(other: (i32, i32)) -> Self {
        i32_i32::new(other.0, other.1)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize, Default)]
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

impl i32_i32_i32 {
    pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribIPointer(
            location as gl::types::GLuint,
            3,       // the number of components per generic vertex attribute
            gl::INT, // data type
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }

    pub unsafe fn gl_uniform(&self, location: usize) {
        gl::Uniform3i(
            location as gl::types::GLint,
            self.d0 as gl::types::GLint,
            self.d1 as gl::types::GLint,
            self.d2 as gl::types::GLint
        );
    }

    pub unsafe fn gl_get_uniform(program: &Program, location: usize) -> Self {
        let mut buf: Vec<i32> = Vec::with_capacity(3);
        buf.resize(3, 0);
        gl::GetUniformiv(
            program.get_id(),
            location as gl::types::GLint,
            buf.as_mut_ptr() as *mut gl::types::GLint
        );

        return (buf[0], buf[1], buf[2]).into()
    }
}

impl From<(i32, i32, i32)> for i32_i32_i32 {
    fn from(other: (i32, i32, i32)) -> Self {
        i32_i32_i32::new(other.0, other.1, other.2)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize, Default)]
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

impl i32_i32_i32_i32 {
    pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribIPointer(
            location as gl::types::GLuint,
            4,       // the number of components per generic vertex attribute
            gl::INT, // data type
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }

    pub unsafe fn gl_uniform(&self, location: usize) {
        gl::Uniform4i(
            location as gl::types::GLint,
            self.d0 as gl::types::GLint,
            self.d1 as gl::types::GLint,
            self.d2 as gl::types::GLint,
            self.d3 as gl::types::GLint
        );
    }

    pub unsafe fn gl_get_uniform(program: &Program, location: usize) -> Self {
        let mut buf: Vec<i32> = Vec::with_capacity(4);
        buf.resize(4, 0);
        gl::GetUniformiv(
            program.get_id(),
            location as gl::types::GLint,
            buf.as_mut_ptr() as *mut gl::types::GLint
        );

        return (buf[0], buf[1], buf[2], buf[3]).into()
    }
}

impl From<(i32, i32, i32, i32)> for i32_i32_i32_i32 {
    fn from(other: (i32, i32, i32, i32)) -> Self {
        i32_i32_i32_i32::new(other.0, other.1, other.2, other.3)
    }
}