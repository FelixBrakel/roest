use crate::core_systems::renderer::data::VertexData;
use gl::Gl;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i2_i10_i10_i10_rev {
    pub inner: u32, // TODO: nicer abstraction
}

impl i2_i10_i10_i10_rev {
    pub fn new(inner: u32) -> i2_i10_i10_i10_rev {
        i2_i10_i10_i10_rev { inner }
    }
}

impl VertexData for i2_i10_i10_i10_rev {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            4,                      // the number of components per generic vertex attribute
            gl::INT_2_10_10_10_REV, // data type
            gl::FALSE,              // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u2_u10_u10_u10_rev {
    pub inner: ::vec_2_10_10_10::Vector,
}

impl u2_u10_u10_u10_rev {
    pub fn new(inner: ::vec_2_10_10_10::Vector) -> u2_u10_u10_u10_rev {
        u2_u10_u10_u10_rev { inner }
    }
}

impl VertexData for u2_u10_u10_u10_rev {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            4, // the number of components per generic vertex attribute
            gl::UNSIGNED_INT_2_10_10_10_REV, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(f32, f32, f32, f32)> for u2_u10_u10_u10_rev {
    fn from(other: (f32, f32, f32, f32)) -> Self {
        u2_u10_u10_u10_rev {
            inner: ::vec_2_10_10_10::Vector::new(other.0, other.1, other.2, other.3),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u10_u11_u11_rev {
    pub inner: u32, // TODO: nicer abstraction
}

impl u10_u11_u11_rev {
    pub fn new(inner: u32) -> u10_u11_u11_rev {
        u10_u11_u11_rev { inner }
    }
}

impl VertexData for u10_u11_u11_rev {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            3, // the number of components per generic vertex attribute
            gl::UNSIGNED_INT_10F_11F_11F_REV, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i2_i10_i10_i10_rev_float {
    pub inner: u32, // TODO: nicer abstraction
}

impl i2_i10_i10_i10_rev_float {
    pub fn new(inner: u32) -> i2_i10_i10_i10_rev_float {
        i2_i10_i10_i10_rev_float { inner }
    }
}

impl VertexData for i2_i10_i10_i10_rev_float {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            4,                      // the number of components per generic vertex attribute
            gl::INT_2_10_10_10_REV, // data type
            gl::TRUE,               // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u2_u10_u10_u10_rev_float {
    pub inner: ::vec_2_10_10_10::Vector,
}

impl u2_u10_u10_u10_rev_float {
    pub fn new(inner: ::vec_2_10_10_10::Vector) -> u2_u10_u10_u10_rev_float {
        u2_u10_u10_u10_rev_float { inner }
    }
}

impl VertexData for u2_u10_u10_u10_rev_float {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            4, // the number of components per generic vertex attribute
            gl::UNSIGNED_INT_2_10_10_10_REV, // data type
            gl::TRUE, // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(f32, f32, f32, f32)> for u2_u10_u10_u10_rev_float {
    fn from(other: (f32, f32, f32, f32)) -> Self {
        u2_u10_u10_u10_rev_float {
            inner: ::vec_2_10_10_10::Vector::new(other.0, other.1, other.2, other.3),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u10_u11_u11_rev_float {
    pub inner: u32, // TODO: nicer abstraction
}

impl u10_u11_u11_rev_float {
    pub fn new(inner: u32) -> u10_u11_u11_rev_float {
        u10_u11_u11_rev_float { inner }
    }
}
impl VertexData for u10_u11_u11_rev_float {
    unsafe fn vertex_attrib_pointer(gl: &Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            3, // the number of components per generic vertex attribute
            gl::UNSIGNED_INT_10F_11F_11F_REV, // data type
            gl::TRUE, // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}