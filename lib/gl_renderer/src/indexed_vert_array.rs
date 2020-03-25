use crate::{Program, buffer, VertexAttribPointers};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct IndexedVertArray<V> {
    pub vertices: Vec<V>,
    pub indices: Vec<u32>,
}

impl<V: VertexAttribPointers> IndexedVertArray<V> {
    pub fn new(vertices: Vec<V>, indices: Vec<u32>) -> IndexedVertArray<V> {
        IndexedVertArray {
            vertices,
            indices
        }
    }
}