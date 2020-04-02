use super::vector_data;
use crate::data::ZSTVariant;
use gl::Gl;
use crate::Program;

#[allow(non_camel_case_types)]
pub struct i32_;

impl ZSTVariant for i32_ {
    type Original = vector_data::i32_;
}

#[allow(non_camel_case_types)]
pub struct i32_i32;

impl i32_ {
    pub unsafe fn gl_get_uniform(gl: &Gl, program: &Program, location: usize) -> vector_data::i32_ {
        let mut buf= 0;
        gl.GetUniformiv(
            program.get_id(),
            location as gl::types::GLint,
            &mut buf as *mut gl::types::GLint
        );

        return buf.into()
    }
}

impl ZSTVariant for i32_i32 {
    type Original = vector_data::i32_i32;
}

impl i32_i32 {
    pub unsafe fn gl_get_uniform(gl: &Gl, program: &Program, location: usize) -> vector_data::i32_i32 {
        let mut buf: Vec<i32> = Vec::with_capacity(2);
        gl.GetUniformiv(
            program.get_id(),
            location as gl::types::GLint,
            buf.as_ptr() as *mut gl::types::GLint
        );

        return (buf[0], buf[1]).into()
    }
}

#[allow(non_camel_case_types)]
pub struct i32_i32_i32;

impl i32_i32_i32 {
    pub unsafe fn gl_get_uniform(gl: &Gl, program: &Program, location: usize) -> vector_data::i32_i32_i32 {
        let mut buf: Vec<i32> = Vec::with_capacity(3);
        gl.GetUniformiv(
            program.get_id(),
            location as gl::types::GLint,
            buf.as_ptr() as *mut gl::types::GLint
        );

        return (buf[0], buf[1], buf[2]).into()
    }
}

impl ZSTVariant for i32_i32_i32 {
    type Original = vector_data::i32_i32_i32;
}

#[allow(non_camel_case_types)]
pub struct i32_i32_i32_i32;

impl i32_i32_i32_i32 {
    pub unsafe fn gl_get_uniform(gl: &Gl, program: &Program, location: usize) -> vector_data::i32_i32_i32_i32 {
        let mut buf: Vec<i32> = Vec::with_capacity(4);
        gl.GetUniformiv(
            program.get_id(),
            location as gl::types::GLint,
            buf.as_ptr() as *mut gl::types::GLint
        );

        return (buf[0], buf[1], buf[2], buf[3]).into()
    }
}

impl ZSTVariant for i32_i32_i32_i32 {
    type Original = vector_data::i32_i32_i32_i32;
}

#[allow(non_camel_case_types)]
pub struct u32_;

impl u32_ {
    pub unsafe fn gl_get_uniform(gl: &Gl, program: &Program, location: usize) -> vector_data::u32_ {
        let mut buf: u32 = 0;
        gl.GetUniformuiv(
            program.get_id(),
            location as gl::types::GLint,
            &mut buf as *mut gl::types::GLuint
        );

        return buf.into()
    }
}

impl ZSTVariant for u32_ {
    type Original = vector_data::u32_;
}

#[allow(non_camel_case_types)]
pub struct u32_u32;

impl u32_u32 {
    pub unsafe fn gl_get_uniform(gl: &Gl, program: &Program, location: usize) -> vector_data::u32_u32 {
        let mut buf: Vec<u32> = Vec::with_capacity(2);
        gl.GetUniformuiv(
            program.get_id(),
            location as gl::types::GLint,
            buf.as_ptr() as *mut gl::types::GLuint
        );

        return (buf[0], buf[1]).into()
    }
}

impl ZSTVariant for u32_u32 {
    type Original = vector_data::u32_u32;
}

#[allow(non_camel_case_types)]
pub struct u32_u32_u32;

impl u32_u32_u32 {
    pub unsafe fn gl_get_uniform(gl: &Gl, program: &Program, location: usize) -> vector_data::u32_u32_u32 {
        let mut buf: Vec<u32> = Vec::with_capacity(3);
        gl.GetUniformuiv(
            program.get_id(),
            location as gl::types::GLint,
            buf.as_ptr() as *mut gl::types::GLuint
        );

        return (buf[0], buf[1], buf[2]).into()
    }
}

impl ZSTVariant for u32_u32_u32 {
    type Original = vector_data::u32_u32_u32;
}

#[allow(non_camel_case_types)]
pub struct u32_u32_u32_u32;

impl u32_u32_u32_u32 {
    pub unsafe fn gl_get_uniform(gl: &Gl, program: &Program, location: usize) -> vector_data::u32_u32_u32_u32 {
        let mut buf: Vec<u32> = Vec::with_capacity(4);
        gl.GetUniformuiv(
            program.get_id(),
            location as gl::types::GLint,
            buf.as_ptr() as *mut gl::types::GLuint
        );

        return (buf[0], buf[1], buf[2], buf[3]).into()
    }
}

impl ZSTVariant for u32_u32_u32_u32 {
    type Original = vector_data::u32_u32_u32_u32;
}

#[allow(non_camel_case_types)]
pub struct f32_;

impl f32_ {
    pub unsafe fn gl_get_uniform(gl: &Gl, program: &Program, location: usize) -> vector_data::f32_ {
        let mut buf = 0_f32;
        gl.GetUniformfv(
            program.get_id(),
            location as gl::types::GLint,
            &mut buf as *mut gl::types::GLfloat
        );

        return buf.into()
    }
}

impl ZSTVariant for f32_ {
    type Original = vector_data::f32_;
}

#[allow(non_camel_case_types)]
pub struct f32_f32;

impl f32_f32 {
    pub unsafe fn gl_get_uniform(gl: &Gl, program: &Program, location: usize) -> vector_data::f32_f32 {
        let mut buf: Vec<f32> = Vec::with_capacity(2);
        gl.GetUniformfv(
            program.get_id(),
            location as gl::types::GLint,
            buf.as_ptr() as *mut gl::types::GLfloat
        );

        return (buf[0], buf[1]).into()
    }
}

impl ZSTVariant for f32_f32 {
    type Original = vector_data::f32_f32;
}

#[allow(non_camel_case_types)]
pub struct f32_f32_f32;

impl f32_f32_f32 {
    pub unsafe fn gl_get_uniform(gl: &Gl, program: &Program, location: usize) -> vector_data::f32_f32_f32 {
        let mut buf: Vec<f32> = Vec::with_capacity(3);
        gl.GetUniformfv(
            program.get_id(),
            location as gl::types::GLint,
            buf.as_ptr() as *mut gl::types::GLfloat
        );

        return (buf[0], buf[1], buf[2]).into()
    }
}

impl ZSTVariant for f32_f32_f32 {
    type Original = vector_data::f32_f32_f32;
}

#[allow(non_camel_case_types)]
pub struct f32_f32_f32_f32;

impl f32_f32_f32_f32 {
    pub unsafe fn gl_get_uniform(gl: &Gl, program: &Program, location: usize) -> vector_data::f32_f32_f32_f32 {
        let mut buf: Vec<f32> = Vec::with_capacity(4);
        gl.GetUniformfv(
            program.get_id(),
            location as gl::types::GLint,
            buf.as_ptr() as *mut gl::types::GLfloat
        );

        return (buf[0], buf[1], buf[2], buf[3]).into()
    }
}

impl ZSTVariant for f32_f32_f32_f32 {
    type Original = vector_data::f32_f32_f32_f32;
}