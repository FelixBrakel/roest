use gl_renderer::data::{vector_data::*};
use gl_renderer::{Program, GlUniform, GPUVariant};

#[derive(Copy, Clone, GPUVariant)]
pub struct Material {
    pub ambient: f32_f32_f32,
    pub diffuse: f32_f32_f32,
    pub specular: f32_f32_f32,
    pub shininess: f32_
}

impl Material {
    pub fn new(ambient: f32_f32_f32, diffuse: f32_f32_f32, specular: f32_f32_f32, shininess: f32_) -> Self {
        Material { ambient, diffuse, specular, shininess }
    }
}

impl Default for Material {
    fn default() -> Self {
        Material::new(
            (0.25, 0.20725, 0.20725).into(),
            (1., 0.829, 0.829).into(),
            (0.296648, 0.296648, 0.296648).into(),
            (0.088 *  128.).into()
        )
    }
}