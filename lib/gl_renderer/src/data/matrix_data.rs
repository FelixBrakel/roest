use crate::Program;
use std::slice::{from_raw_parts};
use std::mem::size_of;
use nalgebra as na;

pub trait GlMat {
    unsafe fn gl_uniform(&self, location: usize);
    /// Can return nonsensical data if the light at the corresponding location has not yet been set
    /// before calling this function
    unsafe fn from_gl_uniform(program: &Program, location: usize) -> Self;
}

pub trait AsColSlices {
    fn as_col_slices(&self) -> Vec<&[u8]>;
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

impl AsColSlices for mat3 {
    fn as_col_slices(&self) -> Vec<&[u8]> {
        let tmp = self.columns(0, 1);
        let col1 = tmp.as_slice();

        let tmp = self.columns(1, 1);
        let col2 =  tmp.as_slice();

        let tmp = self.columns(2, 1);
        let col3 = tmp.as_slice();

        unsafe {
            vec![
                from_raw_parts(col1.as_ptr() as *const _, col1.len() * size_of::<f32>()),
                from_raw_parts(col2.as_ptr() as *const _, col2.len() * size_of::<f32>()),
                from_raw_parts(col3.as_ptr() as *const _, col3.len() * size_of::<f32>()),
            ]
        }
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
    fn as_col_slices(&self) -> Vec<&[u8]> {
        let tmp = self.columns(0, 1);
        let col1 = tmp.as_slice();

        let tmp = self.columns(1, 1);
        let col2 =  tmp.as_slice();

        let tmp = self.columns(2, 1);
        let col3 = tmp.as_slice();

        let tmp = self.columns(3, 1);
        let col4 =  tmp.as_slice();

        // println!("{:#?}", col1);
        // println!("{:#?}", col2);
        // println!("{:#?}", col3);
        // println!("{:#?}", col4);

        unsafe {
            vec![
                from_raw_parts(col1.as_ptr() as *const _, col1.len() * size_of::<f32>()),
                from_raw_parts(col2.as_ptr() as *const _, col2.len() * size_of::<f32>()),
                from_raw_parts(col3.as_ptr() as *const _, col3.len() * size_of::<f32>()),
                from_raw_parts(col4.as_ptr() as *const _, col4.len() * size_of::<f32>()),
            ]
        }
    }
}