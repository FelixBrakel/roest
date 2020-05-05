use crate::data::{vector_data};
use crate::GPUVariant;

#[derive(GPUVariant, Default, Copy, Clone)]
pub struct SpotLight {
    position: vector_data::f32_f32_f32,
    direction: vector_data::f32_f32_f32,

    inner_cone: vector_data::f32_,
    outer_cone: vector_data::f32_,

    constant: vector_data::f32_,
    linear: vector_data::f32_,
    quadratic: vector_data::f32_,

    ambient: vector_data::f32_f32_f32,
    diffuse: vector_data::f32_f32_f32,
    specular: vector_data::f32_f32_f32,
}