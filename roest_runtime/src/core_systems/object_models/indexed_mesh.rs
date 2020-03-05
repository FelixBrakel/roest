use crate::core_systems::renderer::{Program, buffer, VertexAttribPointers};
use crate::core_systems::resource_manager::{load_resource, Resource};
use crate::core_systems::renderer::data;
use crate::core_systems::renderer::data::VertexData;

pub struct IndexedMesh {
    program: Program,
    _index_vbo: buffer::ElementArrayBuffer,
    _vertex_vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray,
    n_indices: usize,
}

impl IndexedMesh {
    pub fn new<V: VertexAttribPointers>(gl: &gl::Gl, vertices: &Vec<V>, indices: &Vec<u32>) -> Result<IndexedMesh, <Program as Resource>::E> {
        let shader_program: Program = load_resource(&gl, "resources/shaders/basic")?;

        let vertex_vbo = buffer::ArrayBuffer::new(&gl);

        vertex_vbo.bind();
        vertex_vbo.static_draw_data(vertices);
        vertex_vbo.unbind();

        let index_vbo = buffer::ElementArrayBuffer::new(&gl);
        index_vbo.bind();
        index_vbo.static_draw_data(indices);
        index_vbo.unbind();

        let mut vao = buffer::VertexArray::new(&gl);
        vao.bind();
        vertex_vbo.bind();
        V::vertex_attrib_pointers(&gl);
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