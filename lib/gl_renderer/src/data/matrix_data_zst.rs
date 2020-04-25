use crate::data::ZSTVariant;
use super::matrix_data;
use crate::Program;

#[allow(non_camel_case_types)]
pub struct mat2;

impl mat2 {
    pub unsafe fn gl_get_uniform(program: &Program, location: usize) -> matrix_data::mat2 {
        let mut buf: Vec<f32> = Vec::with_capacity(4);
        buf.resize(4, 0.);
        gl::GetUniformfv(
            program.get_id(),
            location as gl::types::GLint,
            buf.as_mut_ptr() as *mut gl::types::GLfloat
        );

        matrix_data::mat2::new(buf[0], buf[1], buf[2], buf[3])
    }
}

impl ZSTVariant for mat2 {
    type Original = matrix_data::mat2;
}

#[allow(non_camel_case_types)]
pub struct mat3;

impl mat3 {
    pub unsafe fn gl_get_uniform(program: &Program, location: usize) -> matrix_data::mat3 {
        let mut buf: Vec<f32> = Vec::with_capacity(9);
        buf.resize(9, 0.);
        gl::GetUniformfv(
            program.get_id(),
            location as gl::types::GLint,
            buf.as_mut_ptr() as *mut gl::types::GLfloat
        );

        matrix_data::mat3::new(buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7], buf[8])
    }
}

impl ZSTVariant for mat3 {
    type Original = matrix_data::mat3;
}


#[allow(non_camel_case_types)]
pub struct mat4;

impl mat4 {
    pub unsafe fn gl_get_uniform(program: &Program, location: usize) -> matrix_data::mat4 {
        let mut buf: Vec<f32> = Vec::with_capacity(16);
        buf.resize(16, 0.);
        gl::GetUniformfv(
            program.get_id(),
            location as gl::types::GLint,
            buf.as_mut_ptr() as *mut gl::types::GLfloat
        );

        matrix_data::mat4::new(
            buf[0], buf[1], buf[2], buf[3],
            buf[4], buf[5], buf[6], buf[7],
            buf[8], buf[9], buf[10], buf[11],
            buf[12], buf[13], buf[14], buf[15]
        )
    }
}

impl ZSTVariant for mat4 {
    type Original = matrix_data::mat4;
}
