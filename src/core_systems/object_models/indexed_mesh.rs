use renderer_derive::{VertexAttribPointers};
use crate::core_systems::renderer::{Program, buffer};
use crate::core_systems::resource_manager::{load_resource, Resource};
use crate::core_systems::renderer::data;
use crate::core_systems::renderer::data::VertexData;

#[derive(Copy, Clone, Debug, VertexAttribPointers)]
#[repr(C, packed)]
struct Vertex {
    #[location = 0]
    pos: data::f32_f32_f32,
    #[location = 1]
    clr: data::u2_u10_u10_u10_rev_float,
}

pub struct IndexedMesh {
    program: Program,
    _index_vbo: buffer::ElementArrayBuffer,
    _vertex_vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray,
    n_indices: usize,
}

impl IndexedMesh {
    pub fn new(gl: &gl::Gl) -> Result<IndexedMesh, <Program as Resource>::E> {
        let shader_program: Program = load_resource(&gl, "resources/shaders/basic")?;

        let vertices: Vec<Vertex> = vec![
            Vertex { pos: (-0.5, -0.5, 0.0).into(), clr: (1.0, 0.0, 0.0, 1.0).into() },
            Vertex { pos: (0.5, -0.5, 0.0).into(), clr: (0.0, 1.0, 0.0, 1.0).into() },
            Vertex { pos: (0.0, 0.5, 0.0).into(), clr: (0.0, 0.0, 1.0, 1.0).into() },
        ];

        let vertex_vbo = buffer::ArrayBuffer::new(&gl);

        vertex_vbo.bind();
        vertex_vbo.static_draw_data(&vertices);
        vertex_vbo.unbind();

        let indices: Vec<u32> = vec![0, 1, 2];

        let index_vbo = buffer::ElementArrayBuffer::new(&gl);
        index_vbo.bind();
        index_vbo.static_draw_data(&indices);
        index_vbo.unbind();

        let mut vao = buffer::VertexArray::new(&gl);
        vao.bind();
        vertex_vbo.bind();
        Vertex::vertex_attrib_pointers(&gl);
        vertex_vbo.unbind();
        vao.unbind();

        Ok(IndexedMesh {
            program: shader_program,
            _index_vbo: index_vbo,
            _vertex_vbo: vertex_vbo,
            vao,
            n_indices: indices.len()
        })
    }

    pub fn render(&self, gl: &gl::Gl) {
        self.program.set_used();
        self.vao.bind();
        self._index_vbo.bind();

        unsafe {
            gl.DrawElements(
                gl::TRIANGLES,
                self.n_indices as gl::types::GLsizei,
                gl::UNSIGNED_INT,
                0 as *const gl::types::GLvoid
            );
        }
    }


}