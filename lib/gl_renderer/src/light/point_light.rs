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

impl PointLight {
    pub fn new(
        position: vector_data::f32_f32_f32,
        constant: f32,
        linear: f32,
        quadratic: f32,
        ambient: vector_data::f32_f32_f32,
        diffuse: vector_data::f32_f32_f32,
        specular: vector_data::f32_f32_f32
    ) -> Self {
        PointLight {
            position,
            constant: constant.into(),
            linear: linear.into(),
            quadratic: quadratic.into(),
            ambient,
            diffuse,
            specular
        }
    }
}