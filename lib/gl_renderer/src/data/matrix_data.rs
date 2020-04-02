use gl::Gl;
use crate::Program;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct mat2 {
    pub d00: f32,
    pub d10: f32,
    pub d01: f32,
    pub d11: f32,
}

impl mat2 {
    pub fn new(d00: f32, d10: f32, d01: f32, d11: f32) -> Self {
        mat2 { d00, d10, d01, d11 }
    }

    pub unsafe fn gl_uniform(&self, gl: &Gl, location: usize) {
        gl.UniformMatrix2fv(
            location as gl::types::GLint,
            1,
            gl::FALSE,
            &self.d00 as *const gl::types::GLfloat
        );
    }

    pub unsafe fn gl_get_uniform(gl: &Gl, program: &Program, location: usize) -> Self {
        let mut buf: Vec<f32> = Vec::with_capacity(4);
        gl.GetUniformfv(
            program.get_id(),
            location as gl::types::GLint,
            buf.as_ptr() as *mut gl::types::GLfloat
        );

        return (buf[0], buf[1], buf[2], buf[3]).into()
    }
}

impl From<(f32, f32, f32, f32)> for mat2 {
    fn from(other: (f32, f32, f32, f32)) -> Self {
        Self::new(other.0, other.1, other.2, other.3)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct mat3 {
    pub d00: f32,
    pub d10: f32,
    pub d20: f32,
    pub d01: f32,
    pub d11: f32,
    pub d21: f32,
    pub d02: f32,
    pub d12: f32,
    pub d22: f32,
}

impl mat3 {
    pub fn new(
        d00: f32, d10: f32, d20: f32,
        d01: f32, d11: f32, d21: f32,
        d02: f32, d12: f32, d22: f32
    ) -> Self {
        mat3 { d00, d10, d20, d01, d11, d21, d02, d12, d22 }
    }

    pub unsafe fn gl_uniform(&self, gl: &Gl, location: usize) {
        gl.UniformMatrix3fv(
            location as gl::types::GLint,
            1,
            gl::FALSE,
            &self.d00 as *const gl::types::GLfloat
        );
    }

    pub unsafe fn gl_get_uniform(gl: &Gl, program: &Program, location: usize) -> Self {
        let mut buf: Vec<f32> = Vec::with_capacity(9);
        gl.GetUniformfv(
            program.get_id(),
            location as gl::types::GLint,
            buf.as_ptr() as *mut gl::types::GLfloat
        );

        return (buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7], buf[8]).into()
    }
}

impl From<(f32, f32, f32, f32, f32, f32, f32, f32, f32)> for mat3 {
    fn from(other: (f32, f32, f32, f32, f32, f32, f32, f32, f32)) -> Self {
        Self::new(
            other.0, other.1, other.2,
            other.3, other.4, other.5,
            other.6, other.7, other.8
        )
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct mat4 {
    pub d00: f32,
    pub d10: f32,
    pub d20: f32,
    pub d30: f32,
    pub d01: f32,
    pub d11: f32,
    pub d21: f32,
    pub d31: f32,
    pub d02: f32,
    pub d12: f32,
    pub d22: f32,
    pub d32: f32,
    pub d03: f32,
    pub d13: f32,
    pub d23: f32,
    pub d33: f32,
}

impl mat4 {
    pub fn new(
        d00: f32, d10: f32, d20: f32, d30: f32,
        d01: f32, d11: f32, d21: f32, d31: f32,
        d02: f32, d12: f32, d22: f32, d32: f32,
        d03: f32, d13: f32, d23: f32, d33: f32
    ) -> Self {
        mat4 {
            d00, d10, d20, d30,
            d01, d11, d21, d31,
            d02, d12, d22, d32,
            d03, d13, d23, d33
        }
    }

    pub unsafe fn gl_uniform(&self, gl: &Gl, location: usize) {
        gl.UniformMatrix4fv(
            location as gl::types::GLint,
            1,
            gl::FALSE,
            &self.d00 as *const gl::types::GLfloat
        );
    }

    pub unsafe fn gl_get_uniform(gl: &Gl, program: &Program, location: usize) -> Self {
        let mut buf: Vec<f32> = Vec::with_capacity(16);
        gl.GetUniformfv(
            program.get_id(),
            location as gl::types::GLint,
            buf.as_ptr() as *mut gl::types::GLfloat
        );

        (
            buf[0], buf[1], buf[2], buf[3],
            buf[4], buf[5], buf[6], buf[7],
            buf[8], buf[9], buf[10], buf[11],
            buf[12], buf[13], buf[14], buf[15]
        )
            .into()
    }
}

impl From<(f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32)> for mat4 {
    fn from(other: (f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32)) -> Self {
        Self::new(
            other.0, other.1, other.2, other.3,
            other.4, other.5, other.6, other.7,
            other.8, other.9, other.10, other.11,
            other.12, other.13, other.14, other.15,
        )
    }
}