use crate::data::{vector_data};
use crate::GPUVariant;

#[derive(GPUVariant, Default, Copy, Clone)]
pub struct PointLight {
    pub position: vector_data::f32_f32_f32,

    pub constant: vector_data::f32_,
    pub linear: vector_data::f32_,
    pub quadratic: vector_data::f32_,

    pub ambient: vector_data::f32_f32_f32,
    pub diffuse: vector_data::f32_f32_f32,
    pub specular: vector_data::f32_f32_f32,
}