use crate::data::mesh_data::{IndexedMesh, Mesh};
use crate::data::material_data::Material;
use crate::renderables::Renderer;

pub struct IMeshRenderer<M: Material> {
    gl: gl::Gl,
    mesh: IndexedMesh,
    pub material: M
}

impl<M: Material> IMeshRenderer<M> {
    pub fn new(gl: gl::Gl, mesh: IndexedMesh, material: M) -> Self {
        IMeshRenderer { gl, mesh, material }
    }
}

impl<M: Material> Renderer for IMeshRenderer<M> {
    fn render(&self) {
        self.material.set_used();
        self.mesh.bind();

        unsafe {
            self.gl.DrawElements(
                gl::TRIANGLES,
                self.mesh.n_indices as gl::types::GLsizei,
                gl::UNSIGNED_INT,
                0 as *const gl::types::GLvoid
            );
        }
    }
}