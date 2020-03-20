use gl_renderer::{Program, buffer, VertexAttribPointers};
use crate::core_systems::resource_manager::{Loader};
use crate::core_systems::resource_manager::data_loaders::{ProgramLoader};
// use crate::core_systems::renderer::data;
// use crate::core_systems::renderer::data::VertexData;

pub struct IndexedMesh {
    program: Program,
    _index_vbo: buffer::ElementArrayBuffer,
    _vertex_vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray,
    n_indices: usize,
}

impl IndexedMesh {
    pub fn new<V: VertexAttribPointers>(gl: &gl::Gl, vertices: &Vec<V>, indices: &Vec<u32>) -> Result<IndexedMesh, <ProgramLoader as Loader>::E> {
        let loader = ProgramLoader::new(gl.clone());
        let shader_program: Program = loader.load("resources/shaders/basic")?;

        let vertex_vbo = buffer::ArrayBuffer::new(gl.clone());

        vertex_vbo.bind();
        vertex_vbo.static_draw_data(vertices);
        vertex_vbo.unbind();

        let index_vbo = buffer::ElementArrayBuffer::new(gl.clone());
        index_vbo.bind();
        index_vbo.static_draw_data(indices);
        index_vbo.unbind();

        let mut vao = buffer::VertexArray::new(gl.clone());
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