pub mod vector_data;
pub mod vertex_data;

pub trait VertexData {
    unsafe fn vertex_attrib_pointer(gl: &gl::Gl, stride: usize, location: usize, offset: usize);
}