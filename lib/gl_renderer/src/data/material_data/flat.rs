use super::super::vector_data;
use crate::data::ZSTVariant;
use crate::data::vector_data_zst::{f32_f32_f32 as vec3, f32_f32 as vec2};
use crate::data::matrix_data_zst;
use crate::{gl_getters, gl_setters, Program};
use crate::data::material_data::Material as MatTrait;

#[derive(gl_getters, gl_setters)]
pub struct Material {
    gl: gl::Gl,
    program: Program,
    #[location = 0]
    MVP: matrix_data_zst::mat4,
}

impl Material {
    pub fn new(gl: gl::Gl, program: Program) -> Self {
        Material { gl, program, MVP: matrix_data_zst::mat4 }
    }
}

impl MatTrait for Material {
    fn set_used(&self) {
        self.program.set_used();
    }
}