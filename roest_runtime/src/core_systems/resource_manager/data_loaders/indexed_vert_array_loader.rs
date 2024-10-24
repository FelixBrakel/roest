use gl_renderer::{IndexedVertArray, VertexAttribPointers};
use crate::core_systems::resource_manager;
use crate::core_systems::resource_manager::{Loader, open_file};
use std::path::Path;
use thiserror::Error;
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Deserializer error")]
    Deserializer(#[source] bincode::Error),
    #[error("Failed to load resource {}", name)]
    ResourceLoad { name: String, #[source] inner: resource_manager::Error },
}

pub struct IndexedVertArrayLoader<V> {
    _marker: PhantomData<V>
}

impl<V: VertexAttribPointers> IndexedVertArrayLoader<V> {
    pub fn new() -> IndexedVertArrayLoader<V> {
        IndexedVertArrayLoader { _marker: PhantomData }
    }
}

impl<V: VertexAttribPointers + DeserializeOwned> Loader for IndexedVertArrayLoader<V> {
    type E = Error;
    type R = IndexedVertArray<V>;

    fn load(&self, path: impl AsRef<Path>) -> Result<IndexedVertArray<V>, Error> {
        let file = open_file(&path)
            .map_err(|e| Error::ResourceLoad {name: path.as_ref().to_string_lossy().into_owned(), inner: e})?;
        let deserialized: Result<IndexedVertArray<V>, Error> = bincode::deserialize_from(file)
            .map_err(|e| Error::Deserializer(e));
        deserialized
    }
}