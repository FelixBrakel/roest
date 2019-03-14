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
    let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let vert_shader = Shader::from_file("assets/shaders/basic.vert").unwrap();
    let frag_shader = Shader::from_file("assets/shaders/basic.frag").unwrap();

    let shader_program = Program::from_shaders(&[vert_shader, frag_shader]).unwrap();
    shader_program.set_used();

    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let mut event_pump = _sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_swap_window();

    }
}