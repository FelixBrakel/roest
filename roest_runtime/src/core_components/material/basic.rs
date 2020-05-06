use gl_renderer::data::{vector_data::*};
use gl_renderer::{Program, GlUniform, GPUVariant};

#[derive(Copy, Clone, GPUVariant)]
pub struct Material {
    ambient: f32_f32_f32,
    diffuse: f32_f32_f32,
    specular: f32_f32_f32,
    shininess: f32_
}

impl Material {
    pub fn new(ambient: f32_f32_f32, diffuse: f32_f32_f32, specular: f32_f32_f32, shininess: f32_) -> Self {
        Material { ambient, diffuse, specular, shininess }
    }
}