extern crate sdl2;
extern crate tobj;
extern crate gl;

mod core_systems;
mod runtime_systems;
use core_systems::renderer::{Shader, Program};

fn main() {
    let _sdl = sdl2::init().unwrap();
    let video_subsystem = _sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);
    let window = video_subsystem
        .window("Roest", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();
    let gl = gl::Gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl.Viewport(0, 0, 900, 700);
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0,  1.0, 0.0, 0.0,
        0.5, -0.5, 0.0,   0.0, 1.0, 0.0,
        0.0, 0.5, 0.0,     0.0, 0.0, 1.0,
    ];
    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl.GenBuffers(1, &mut vbo);
    }

    unsafe {
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW
        );

        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl.GenVertexArrays(1, &mut vao);
    }

    unsafe {
        gl.BindVertexArray(vao);
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.EnableVertexAttribArray(0);
        gl.VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            std::ptr::null()
        );
        gl.EnableVertexAttribArray(1);
        gl.VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
        );

        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        gl.BindVertexArray(0);
    }

//    let vert_shader = Shader::from_res(&gl, "resources/shaders/basic.vert").unwrap();
//    let frag_shader = Shader::from_res(&gl, "resources/shaders/basic.frag").unwrap();

    let shader_program = Program::from_res(&gl, "basic").unwrap();
    shader_program.set_used();
    let mut event_pump = _sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }

        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
            gl.BindVertexArray(vao);
            gl.DrawArrays(gl::TRIANGLES, 0, 3)
        }

        window.gl_swap_window();

    }
}