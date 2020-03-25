use crate::data::vector_data;
use crate::VertexAttribPointers;
use crate::data::VertexData;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, VertexAttribPointers, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct Vertex {
    #[location = 0]
    pub pos: vector_data::f32_f32_f32,
    #[location = 1]
    pub clr: vector_data::u2_u10_u10_u10_rev_float,
}