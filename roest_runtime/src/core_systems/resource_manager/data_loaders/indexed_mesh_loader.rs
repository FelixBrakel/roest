use gl_renderer::{VertexAttribPointers, IndexedVertArray};
use gl_renderer::data::mesh_data::IndexedMesh;
use crate::core_systems::resource_manager::{Loader};
use std::path::Path;
use failure::Fail;
use super::{ProgramLoader, IndexedVertArrayLoader, PrLoaderError, IVertArrLoaderError};
use failure::_core::marker::PhantomData;
use serde::de::DeserializeOwned;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Indexed Vertex Array Loader error")]
    IndexedVertArrayLoader(#[cause] IVertArrLoaderError),
    #[fail(display = "Program Loader error")]
    ProgramLoader(#[cause] PrLoaderError)
}

pub struct IndexedMeshLoader<V> {
    gl: gl::Gl,
    _marker: PhantomData<V>
}

impl<V: VertexAttribPointers> IndexedMeshLoader<V> {
    pub fn new(gl: gl::Gl) -> IndexedMeshLoader<V> {
        IndexedMeshLoader { gl, _marker: PhantomData }
    }
}

impl<V: VertexAttribPointers + DeserializeOwned> Loader for IndexedMeshLoader<V> {
    type E = Error;
    type R = IndexedMesh;

    fn load(&self, name: impl AsRef<Path>) -> Result<IndexedMesh, Error> {
        let vert_array: IndexedVertArray<V> = IndexedVertArrayLoader::new().load("resources/meshes/triangle.mesh")
            .map_err(|e| Error::IndexedVertArrayLoader(e))?;
        // let program = ProgramLoader::new(self.gl.clone()).load("resources/shaders/basic")
        //     .map_err(|e| Error::ProgramLoader(e))?;

        Ok(IndexedMesh::new(self.gl.clone(), &vert_array))
    }
}