use crate::{Program, buffer, VertexAttribPointers, IndexedVertArray};
use crate::data::mesh_data::Mesh;

pub struct IndexedMesh {
    _index_vbo: buffer::ElementArrayBuffer,
    _vertex_vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray,
    pub n_indices: usize,
}

impl IndexedMesh {
    pub fn new<V: VertexAttribPointers>(gl: gl::Gl, verts: &IndexedVertArray<V>) -> IndexedMesh {
        let vertex_vbo = buffer::ArrayBuffer::new(gl.clone());

        vertex_vbo.bind();
        vertex_vbo.static_draw_data(&verts.vertices);
        vertex_vbo.unbind();

        let index_vbo = buffer::ElementArrayBuffer::new(gl.clone());
        index_vbo.bind();
        index_vbo.static_draw_data(&verts.indices);
        index_vbo.unbind();

        let mut vao = buffer::VertexArray::new(gl.clone());
        vao.bind();
        vertex_vbo.bind();
        V::vertex_attrib_pointers(&gl);
        vertex_vbo.unbind();
        vao.unbind();

        IndexedMesh {
            _index_vbo: index_vbo,
            _vertex_vbo: vertex_vbo,
            vao,
            n_indices: verts.indices.len()
        }
    }
}

impl Mesh for IndexedMesh {
    fn bind(&self) {
        self.vao.bind();
        self._index_vbo.bind();
    }
}