use failure::err_msg;
use nalgebra as na;
mod core_systems;
mod core_components;
use legion::prelude::*;

use gl_renderer::{
    Viewport,
    ColorBuffer,
    vertex,
    GPUVariant,
    data::matrix_data::{mat3},
    data::vector_data,
    light::*,
    uniform_buffer::InterfaceBlock
};
use core_systems::resource_manager::data_loaders::{IndexedMeshLoader};
use crate::core_systems::resource_manager::Loader;
use crate::core_systems::resource_manager::data_loaders::ProgramLoader;
use crate::core_components::Transform;

#[derive(GPUVariant, Default)]
pub struct Lights {
    directional: DirectionalLight,
    point_lights: [PointLight; 16],
    spot_lights: [SpotLight; 16],

    num_point_lights: vector_data::i32_,
    num_spot_lights: vector_data::i32_
}

pub enum LightsError {
    MaxPointLightsReached,
    PointLightIdxOutOfBounds,
    MaxSpotLightsReached,
    SpotLightIdxOutOfBounds,
}

impl Lights {
    pub fn add_point_light(&mut self, light: PointLight) -> Result<(), LightsError> {
        if self.num_point_lights.d0 > 16 {
            return Err(LightsError::MaxPointLightsReached);
        }

        self.point_lights[self.num_point_lights.d0 as usize] = light;
        self.num_point_lights.d0 += 1;

        Ok(())
    }

    pub fn set_point_light(&mut self, idx: usize, light: PointLight) -> Result<(), LightsError> {
        if idx >= self.num_point_lights.d0 as usize {
            return Err(LightsError::PointLightIdxOutOfBounds);
        }

        self.point_lights[idx] = light;

        Ok(())
    }

    pub fn add_spot_light(&mut self, light: SpotLight) -> Result<(), LightsError> {
        if self.num_spot_lights.d0 > 16 {
            return Err(LightsError::MaxSpotLightsReached);
        }

        self.spot_lights[self.num_spot_lights.d0 as usize] = light;
        self.num_spot_lights.d0 += 1;

        Ok(())
    }

    pub fn set_spot_light(&mut self, idx: usize, light: SpotLight) -> Result<(), LightsError> {
        if idx >= self.num_spot_lights.d0 as usize {
            return Err(LightsError::SpotLightIdxOutOfBounds);
        }

        self.spot_lights[idx] = light;

        Ok(())
    }
}

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
    let interface_block = InterfaceBlock::<Lights>::new(&program, "Lights");

    let teapot_loader: IndexedMeshLoader<vertex::NormalVertex> = IndexedMeshLoader::new();

    let material = core_components::material::Flat::new((0.2, 0.2, 0.3, 1.0).into());

    let color_buffer = ColorBuffer::from_color(na::Vector3::new(0.3, 0.3, 0.5));
    color_buffer.set_used();

    let renderer = core_systems::RendererSystem::system();
    world.resources.insert(program);
    world.resources.insert(interface_block);

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
        (0..2).map(
            |i| (
                Transform::new(1., na::Vector3::new(i as f32, i as f32, i as f32), na::UnitQuaternion::from_euler_angles(0., 0., 0.)),
                PointLight {
                    position: (0., 0., 0.).into(),
                    constant: (1.).into(),
                    linear: (0.7).into(),
                    quadratic: (1.8).into(),
                    ambient: (0.1, 0.1, 0.1).into(),
                    diffuse: (0.1, 0.1, 0.1).into(),
                    specular: (0.1, 0.1, 0.1).into()
                }
                )
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
                    let cam_query = <(Write<core_components::Camera>,)>::query();

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
        let query = <(Write<core_components::Transform>, Read<core_components::IndexedMesh>)>::query();

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