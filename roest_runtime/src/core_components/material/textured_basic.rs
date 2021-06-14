use gl_renderer::data::{vector_data::*};
use gl_renderer::{GPUVariant};
use gl_renderer::texture::{Texture2D, ResidentBindlessTexture};

#[derive(GPUVariant)]
pub struct Material {
    ambient: f32_f32_f32,
    diffuse: ResidentBindlessTexture<Texture2D>,
    specular: f32_f32_f32,
    shininess: f32_
}
//TODO: Add shader to material.
impl Material {
    pub fn new(ambient: f32_f32_f32, diffuse: ResidentBindlessTexture<Texture2D>, specular: f32_f32_f32, shininess: f32_) -> Self {
        Material { ambient, diffuse, specular, shininess }
    }
}