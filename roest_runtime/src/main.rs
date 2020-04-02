use failure::err_msg;
// use renderer_derive::{VertexAttribPointers};
use nalgebra as na;
mod core_systems;

use gl_renderer::{Viewport, ColorBuffer, data, data::mesh_data::IndexedMesh, data::matrix_data::mat4};
use core_systems::resource_manager::data_loaders::{IndexedMeshLoader, FlatMatLoader};
use crate::core_systems::resource_manager::Loader;
use gl_renderer::renderables::{IMeshRenderer, Renderer};

fn main() {
    if let Err(e) = run() {
        println!("{}", failure_to_string(e));
    }
}

fn run() -> Result<(), failure::Error> {
    let sdl = sdl2::init().map_err(err_msg)?;
    let video_subsystem = sdl.video().map_err(err_msg)?;
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);
    let window = video_subsystem
        .window("Roest", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().map_err(err_msg)?;
    let gl = gl::Gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let teapot_loader: IndexedMeshLoader<data::vertex_data::ColoredVertex> = IndexedMeshLoader::new(gl.clone());
    let teapot = teapot_loader.load("REPLACE").unwrap();

    let flatmat_loader = FlatMatLoader::new(gl.clone());
    let flatmat = flatmat_loader.load("REPLACE ME").unwrap();

    let teapot_render = IMeshRenderer::new(gl.clone(), teapot, flatmat);
    let mat = na::Matrix::from_data()
    let mvp = mat4::new(
        0.5, 0., 0., 0.,
        0., 0.5, 0., 0.,
        0., 0., 0.5, 0.,
        0., 0., 0., 1.);
    teapot_render.material.gl_set_MVP(mvp);

    let mut viewport = Viewport::for_window(900, 700);
    viewport.set_used(&gl);

    let color_buffer = ColorBuffer::from_color(na::Vector3::new(0.3, 0.3, 0.5));
    color_buffer.set_used(&gl);

    let mut event_pump = sdl.event_pump().map_err(err_msg)?;
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                sdl2::event::Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(w, h),
                    ..
                } => {
                    viewport.update_size(w, h);
                    viewport.set_used(&gl);
                },
                _ => {},
            }
        }

        color_buffer.clear(&gl);

        teapot_render.render();

        window.gl_swap_window();
    }

    Ok(())
}

pub fn failure_to_string(e: failure::Error) -> String {
    use std::fmt::Write;

    let mut result = String::new();
    for (i, cause) in e
        .iter_chain()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .enumerate()
    {
        if i > 0 {
            let _ = writeln!(&mut result, "  Which caused the following issue:");
        }

        let _ = write!(&mut result, "{}", cause);
        if let Some(backtrace) = cause.backtrace() {
            let bracktrace_str = format!("{}", backtrace);
            if bracktrace_str.len() > 0 {
                let _ = writeln!(&mut result,  " This happened at {}", backtrace);
            } else {
                let _ = writeln!(&mut result);
            }
        } else {
            let _ = writeln!(&mut result);
        }
    }

    result
}