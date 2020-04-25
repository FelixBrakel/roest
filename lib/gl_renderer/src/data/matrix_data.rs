use crate::Program;
use std::slice::from_raw_parts;
use std::mem::size_of;
use nalgebra as na;

pub trait GlMat {
    unsafe fn gl_uniform(&self, location: usize);
    /// Can return nonsensical data if the uniform at the corresponding location has not yet been set
    /// before calling this function
    unsafe fn from_gl_uniform(program: &Program, location: usize) -> Self;
}

pub trait AsColSlices {
    fn as_col_slices(&self) -> &[&[u8]];
}

#[allow(non_camel_case_types)]
pub type mat2 = na::Matrix2<f32>;

impl GlMat for mat2 {
    unsafe fn gl_uniform(&self, location: usize) {
        gl::UniformMatrix2fv(
            location as gl::types::GLint,
            1,
            gl::FALSE,
            self.as_slice().as_ptr() as *const gl::types::GLfloat
        );
    }

    unsafe fn from_gl_uniform(program: &Program, location: usize) -> Self {
        let mut buf: Vec<f32> = Vec::with_capacity(4);
        buf.resize(4, 0.);
        gl::GetUniformfv(
            program.get_id(),
            location as gl::types::GLint,
            buf.as_mut_ptr() as *mut gl::types::GLfloat
        );

        return mat2::new(
            buf[0], buf[1],
            buf[2], buf[3]
        )
    }
}


#[allow(non_camel_case_types)]
pub type mat3 = na::Matrix3<f32>;

impl GlMat for mat3 {
    unsafe fn gl_uniform(&self, location: usize) {
        gl::UniformMatrix3fv(
            location as gl::types::GLint,
            1,
            gl::FALSE,
            self.as_slice().as_ptr() as *const gl::types::GLfloat
        );
    }

    unsafe fn from_gl_uniform(program: &Program, location: usize) -> Self {
        let mut buf: Vec<f32> = Vec::with_capacity(9);
        buf.resize(9, 0.);
        gl::GetUniformfv(
            program.get_id(),
            location as gl::types::GLint,
            buf.as_mut_ptr() as *mut gl::types::GLfloat
        );

        return mat3::new(
            buf[0], buf[1], buf[2],
            buf[3], buf[4], buf[5],
            buf[6], buf[7], buf[8]
        )
    }
}

#[allow(non_camel_case_types)]
pub type mat4 = na::Matrix4<f32>;

impl GlMat for mat4 {
    unsafe fn gl_uniform(&self, location: usize) {
        gl::UniformMatrix4fv(
            location as gl::types::GLint,
            1,
            gl::FALSE,
            self.as_slice().as_ptr() as *const gl::types::GLfloat
        );
    }

    unsafe fn from_gl_uniform(program: &Program, location: usize) -> Self {
        let mut buf: Vec<f32> = Vec::with_capacity(16);
        buf.resize(16, 0.);
        gl::GetUniformfv(
            program.get_id(),
            location as gl::types::GLint,
            buf.as_mut_ptr() as *mut gl::types::GLfloat
        );

        mat4::new(
            buf[0], buf[1], buf[2], buf[3],
            buf[4], buf[5], buf[6], buf[7],
            buf[8], buf[9], buf[10], buf[11],
            buf[12], buf[13], buf[14], buf[15]
        )
    }
}

impl AsColSlices for mat4 {
    fn as_col_slices(&self) -> &[&[u8]] {
        let mvp_col1 = self.columns(0, 1).as_slice();
        let mvp_col2 = self.columns(1, 1).as_slice();
        let mvp_col3 = self.columns(2, 1).as_slice();
        let mvp_col4 = self.columns(3, 1).as_slice();

        unsafe {
            &[
                from_raw_parts(mvp_col1.as_ptr() as *const u8, mvp_col1.len() * size_of::<f32>()),
                from_raw_parts(mvp_col2.as_ptr() as *const u8, mvp_col2.len() * size_of::<f32>()),
                from_raw_parts(mvp_col3.as_ptr() as *const u8, mvp_col3.len() * size_of::<f32>()),
                from_raw_parts(mvp_col4.as_ptr() as *const u8, mvp_col4.len() * size_of::<f32>()),
            ]
        }
    }
}