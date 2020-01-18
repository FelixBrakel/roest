use crate::core_systems::renderer::data::VertexData;
use gl::Gl;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i8_ {
    pub d0: i8,
}

impl i8_{
    fn new(d0: i8) -> i8_ {
        i8_ { d0 }
    }
}

impl VertexData for i8_ {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            1,
            gl::BYTE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid
        );
    }
}

impl From<i8> for i8_ {
    fn from(other: i8) -> i8_ {
        i8_::new(other)
    }
}