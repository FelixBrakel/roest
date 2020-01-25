use renderer_derive::{VertexAttribPointers};
use crate::core_systems::renderer::{Program, buffer};
use crate::core_systems::resource_manager::{load_resource, Resource};
use crate::core_systems::renderer::data;
use crate::core_systems::renderer::data::VertexData;

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
    pub fn new(gl: &gl::Gl) -> Result<Triangle, <Program as Resource>::E> {
        let shader_program: Program = load_resource(&gl, "resources/shaders/basic")?;

        let vertices: Vec<Vertex> = vec![
            Vertex { pos: (-0.5, -0.5, 0.0).into(), clr: (1.0, 0.0, 0.0, 1.0).into() },
            Vertex { pos: (0.5, -0.5, 0.0).into(), clr: (0.0, 1.0, 0.0, 1.0).into() },
            Vertex { pos: (0.0, 0.5, 0.0).into(), clr: (0.0, 0.0, 1.0, 1.0).into() },
        ];
        let vbo = buffer::ArrayBuffer::new(&gl);

        vbo.bind();
        vbo.static_draw_data(&vertices);
        vbo.unbind();

        let mut vao = buffer::VertexArray::new(&gl);
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