use gl_renderer::data::{vector_data};
use gl_renderer::{Program, GlUniform};
// use specs::{Component, VecStorage};

#[derive(Copy, Clone)]
pub struct Material {
    color: vector_data::f32_f32_f32_f32
}

// impl Component for Material {
//     type Storage = VecStorage<Material>;
// }

impl GlUniform for Material {
    fn gl_uniform(&self) {
        unsafe {
            self.color.gl_uniform(5);
        }
    }

    fn from_uniform(program: &Program) -> Self {
        let color = unsafe {
            vector_data::f32_f32_f32_f32::from_gl_uniform(&program, 0)
        };

        Material { color }
    }
}

impl Material {
    pub fn new(color: vector_data::f32_f32_f32_f32) -> Self {
        Material { color }
    }
}