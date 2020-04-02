mod indexed_mesh;
mod mesh_renderer;

pub use mesh_renderer::IMeshRenderer;
pub use indexed_mesh::IndexedMesh;

pub trait Renderer {
    fn render(&self);
}