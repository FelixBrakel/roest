use gl_renderer::{VertexAttribPointers, IndexedVertArray};
use crate::core_components::IndexedMesh;
use crate::core_systems::resource_manager::{Loader};
use std::path::Path;
use failure::Fail;
use super::{IndexedVertArrayLoader, IVertArrLoaderError};
use failure::_core::marker::PhantomData;
use serde::de::DeserializeOwned;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Indexed Vertex Array Loader error")]
    IndexedVertArrayLoader(#[cause] IVertArrLoaderError),
}

pub struct IndexedMeshLoader<V> {
    _marker: PhantomData<V>
}

impl<V: VertexAttribPointers> IndexedMeshLoader<V> {
    pub fn new() -> IndexedMeshLoader<V> {
        IndexedMeshLoader { _marker: PhantomData }
    }
}

impl<V: VertexAttribPointers + DeserializeOwned> Loader for IndexedMeshLoader<V> {
    type E = Error;
    type R = IndexedMesh;

    fn load(&self, name: impl AsRef<Path>) -> Result<IndexedMesh, Error> {
        let vert_array: IndexedVertArray<V> = IndexedVertArrayLoader::new().load(name)
            .map_err(|e| Error::IndexedVertArrayLoader(e))?;

        Ok(IndexedMesh::new(&vert_array))
    }
}