use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct u32_float {
    pub d0: u32,
}

impl u32_float {
    pub fn new(d0: u32) -> u32_float {
        u32_float { d0 }
    }
}

impl u32_float {
    pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribPointer(
            location as gl::types::GLuint,
            1,                // the number of components per generic vertex attribute
            gl::UNSIGNED_INT, // data type
            gl::TRUE,         // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<u32> for u32_float {
    fn from(other: u32) -> Self {
        u32_float::new(other)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct u32_u32_float {
    pub d0: u32,
    pub d1: u32,
}

impl u32_u32_float {
    pub fn new(d0: u32, d1: u32) -> u32_u32_float {
        u32_u32_float { d0, d1 }
    }
}

impl u32_u32_float {
    pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribPointer(
            location as gl::types::GLuint,
            2,                // the number of components per generic vertex attribute
            gl::UNSIGNED_INT, // data type
            gl::TRUE,         // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(u32, u32)> for u32_u32_float {
    fn from(other: (u32, u32)) -> Self {
        u32_u32_float::new(other.0, other.1)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct u32_u32_u32_float {
    pub d0: u32,
    pub d1: u32,
    pub d2: u32,
}

impl u32_u32_u32_float {
    pub fn new(d0: u32, d1: u32, d2: u32) -> u32_u32_u32_float {
        u32_u32_u32_float { d0, d1, d2 }
    }
}

impl u32_u32_u32_float {
    pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribPointer(
            location as gl::types::GLuint,
            3,                // the number of components per generic vertex attribute
            gl::UNSIGNED_INT, // data type
            gl::TRUE,         // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(u32, u32, u32)> for u32_u32_u32_float {
    fn from(other: (u32, u32, u32)) -> Self {
        u32_u32_u32_float::new(other.0, other.1, other.2)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct u32_u32_u32_u32_float {
    pub d0: u32,
    pub d1: u32,
    pub d2: u32,
    pub d3: u32,
}

impl u32_u32_u32_u32_float {
    pub fn new(d0: u32, d1: u32, d2: u32, d3: u32) -> u32_u32_u32_u32_float {
        u32_u32_u32_u32_float { d0, d1, d2, d3 }
    }
}

impl u32_u32_u32_u32_float {
    pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribPointer(
            location as gl::types::GLuint,
            4,                // the number of components per generic vertex attribute
            gl::UNSIGNED_INT, // data type
            gl::TRUE,         // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(u32, u32, u32, u32)> for u32_u32_u32_u32_float {
    fn from(other: (u32, u32, u32, u32)) -> Self {
        u32_u32_u32_u32_float::new(other.0, other.1, other.2, other.3)
    }
}