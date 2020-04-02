mod indexed_mesh;

pub use indexed_mesh::IndexedMesh;

pub trait Mesh {
    fn bind(&self);
}