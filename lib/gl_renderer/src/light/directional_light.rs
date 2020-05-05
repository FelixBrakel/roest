use crate::data::{vector_data};
use crate::GPUVariant;

#[derive(GPUVariant, Default, Copy, Clone)]
pub struct DirectionalLight {
    direction: vector_data::f32_f32_f32,

    ambient: vector_data::f32_f32_f32,
    diffuse: vector_data::f32_f32_f32,
    specular: vector_data::f32_f32_f32,
}