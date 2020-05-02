use failure::err_msg;
use nalgebra as na;
mod core_systems;
mod core_components;
use legion::prelude::*;

use gl_renderer::{Viewport, ColorBuffer, vertex, uniform_buffer::InterfaceBlock, uniform_buffer::UniformBlock, uniform_struct_shared::ShaderDefaultLayout, uniform_struct_shared::GPUAggregate};
use core_systems::resource_manager::data_loaders::{IndexedMeshLoader};
use crate::core_systems::resource_manager::Loader;
use gl_renderer::data::matrix_data::{mat3, mat4};
use crate::core_systems::resource_manager::data_loaders::ProgramLoader;
use gl_renderer::buffer::UniformBuffer;
use gl_renderer::data::vector_data_zst::f32_f32_f32;
use gl_renderer::uniform_struct_shared::TestStruct;
use gl::ffi_error_callback;

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
    gl::load_with(
        |s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    );

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::DEBUG_OUTPUT);

        gl::DebugMessageCallback(Some(gl::error_callback), std::ptr::null())
    }

    let mut viewport = Viewport::for_window(900, 700);
    viewport.set_used();
    let universe = Universe::new();
    let mut world = universe.create_world();

    // world.register::<core_components::Transform>();
    // world.register::<core_components::Camera>();
    // world.register::<core_components::renderable::IMeshRenderer>();
    // world.register::<core_components::IndexedMesh>();
    // world.register::<core_components::material::Flat>();

    let transform = core_components::Transform::new(
        1.,
        na::Vector3::new(0., 0., -2.),
        na::UnitQuaternion::from_euler_angles(0., 0., 0.),
        na::Rotation3::from_euler_angles(0., 0., 0.)
    );

    let cam_rot = mat3::new(
        1., 0., 0.,
        0., 1., 0.,
        0., 0., 1.
    );
    let camera = core_components::Camera::from_fov(
        (0., 0., 0.).into(),
        cam_rot,
        60.,
        viewport.w as f32 / viewport.h as f32,
        0.1,
        100.
    );

    let program = ProgramLoader::new().load("resources/shaders/basic").unwrap();
    let uniform_buffer = UniformBlock::new(&program, "Defaults");
    let interface_block = InterfaceBlock::<ShaderDefaultLayout>::new(&program, "Defaults", &uniform_buffer);

    let tmp = ShaderDefaultLayout {
        mvp: mat4::identity(),
        mv: mat4::identity(),
        test_arr: [(1., 1., 1.).into(), (1., 1., 1.).into()],
        test_struct: TestStruct {
            data: (0.9, 0.5, 0.5).into(),
            other_data: (1., 1., 1.).into()
        },
        test_struct_arr: [
        TestStruct {
            data: (0.9, 0.5, 0.5).into(),
            other_data: (1., 1., 1.).into()
        },
        TestStruct {
            data: (0.9, 0.5, 0.5).into(),
            other_data: (1., 1., 1.).into()
        },
        TestStruct {
            data: (0.9, 0.5, 0.5).into(),
            other_data: (1., 1., 1.).into()
        },
        ]
    };
    interface_block.uniform_struct.set(&tmp);

    let teapot_loader: IndexedMeshLoader<vertex::NormalVertex> = IndexedMeshLoader::new();

    let material = core_components::material::Flat::new((0.1, 0.1, 0.1, 1.0).into());

    let color_buffer = ColorBuffer::from_color(na::Vector3::new(0.3, 0.3, 0.5));
    color_buffer.set_used();

    let renderer = core_systems::RendererSystem::system();
    world.resources.insert(program);

    world.insert(
        (),
        (0..1).map(
            |_| (
                transform,
                material,
                teapot_loader.load("resources/meshes/triangle.mesh").unwrap())
        )
    );

    world.insert(
        (),
        (0..1).map(|_| (camera,))
    );


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
                    let mut cam_query = <(Write<core_components::Camera>,)>::query();

                    for (mut camera,) in cam_query.iter(&mut world) {
                        camera.update_perspective(
                            60.,
                            viewport.w as f32 / viewport.h as f32,
                            0.1,
                            100.
                        );
                    }

                    viewport.set_used();
                },
                _ => {},
            }
        }
        let mut query = <(Write<core_components::Transform>, Read<core_components::IndexedMesh>)>::query();

        for (mut transform, _) in query.iter(&mut world) {
            transform.rotate(0., 0.01, 0.);
        }

        color_buffer.clear();
        renderer.run(&world);
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