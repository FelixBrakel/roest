use crate::core_systems::renderer

pub struct Triangle {
    program: renderer::Program,
    _vbo: renderer::buffer::ArrayBuffer,
    vao: renderer::buffer::VertexArray,
}

impl Triangle {}