use gl_renderer::{VertexAttribPointers, IndexedVertArray};
use crate::core_components::IndexedMesh;
use crate::core_systems::resource_manager::{Loader};
use std::path::Path;
use thiserror::Error;
use super::{IndexedVertArrayLoader, IVertArrLoaderError};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Indexed Vertex Array Loader error")]
    IndexedVertArrayLoader(#[source] IVertArrLoaderError),
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