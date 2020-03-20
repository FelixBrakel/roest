use gl_renderer::{Program, buffer, VertexAttribPointers, data};
use renderer_derive::{VertexAttribPointers};
use crate::core_systems::resource_manager::{Loader};
use crate::core_systems::resource_manager::data_loaders::{ProgramLoader};

#[derive(Copy, Clone, Debug, VertexAttribPointers)]
#[repr(C, packed)]
struct Vertex {
    #[location = 0]
    pos: data::f32_f32_f32,
    #[location = 1]
    clr: data::u2_u10_u10_u10_rev_float,
}

pub struct Triangle {
    program: Program,
    _vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray,
}

impl Triangle {
    pub fn new(gl: &gl::Gl) -> Result<Triangle, <ProgramLoader as Loader>::E> {
        let loader = ProgramLoader::new(gl.clone());
        let shader_program: Program = loader.load("resources/shaders/basic")?;

        let vertices: Vec<Vertex> = vec![
            Vertex { pos: (-0.5, -0.5, 0.0).into(), clr: (1.0, 0.0, 0.0, 1.0).into() },
            Vertex { pos: (0.5, -0.5, 0.0).into(), clr: (0.0, 1.0, 0.0, 1.0).into() },
            Vertex { pos: (0.0, 0.5, 0.0).into(), clr: (0.0, 0.0, 1.0, 1.0).into() },
        ];

        let vbo = buffer::ArrayBuffer::new(gl.clone());

        vbo.bind();
        vbo.static_draw_data(&vertices);
        vbo.unbind();

        let mut vao = buffer::VertexArray::new(gl.clone());
        vao.bind();
        vbo.bind();
        Vertex::vertex_attrib_pointers(&gl);
        vbo.unbind();
        vao.unbind();

        Ok(Triangle { program: shader_program, _vbo: vbo, vao, })
    }

    pub fn render(&self, gl: &gl::Gl) {
        self.program.set_used();
        self.vao.bind();

        unsafe {
            gl.DrawArrays(
                gl::TRIANGLES,
                0,
                3
            )
        }
    }
}