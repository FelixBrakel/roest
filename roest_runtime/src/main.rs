use nalgebra as na;
mod core_systems;
mod core_components;

pub mod core_resources;
use legion::*;

use anyhow::{
    Result,
    anyhow
};

use gl_renderer::{
    Viewport,
    ColorBuffer,
    vertex,
    GPUVariant,
    data::matrix_data::{mat3, mat4},
    data::vector_data,
    uniform_buffer::InterfaceBlock,
    texture::{
        Texture,
        Texture2D,
        TexWrapMode,
        TexMinFilterMode,
        TexMagFilterMode,
        NonResidentBindlessTexture,
        ResidentBindlessTexture
    }
};

use core_systems::resource_manager::{
    Loader,
    data_loaders::{
        ProgramLoader,
        IndexedMeshLoader,
        ImageLoader
    }
};

use core_components::{
    Transform,
    light::PointLight
};

use core_resources::{
    gpu_blocks::*,
};

use image::GenericImageView;

fn main() -> Result<()> {
    run()
}

fn run() -> Result<(), anyhow::Error> {
    let sdl = sdl2::init().map_err(|err|anyhow!(err))?;
    let video_subsystem = sdl.video().map_err(|err|anyhow!(err))?;
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);
    let window = video_subsystem
        .window("Roest", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().map_err(|err|anyhow!(err))?;
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
    let mut world = World::default();

    let transform = core_components::Transform::new(
        1.,
        na::Vector3::new(0., 0., -2.),
        na::UnitQuaternion::from_euler_angles(0.2, 0., 0.),
    );

    let program = ProgramLoader::new().load("resources/shaders/basic").unwrap();
    let lights_block = InterfaceBlock::<Lights>::new(&program, "Lights", 1);
    let matrices_block = InterfaceBlock::<Matrices>::new(&program, "Matrices", 2);

    let teapot_loader: IndexedMeshLoader<vertex::BasicVertex> = IndexedMeshLoader::new();

    let img = ImageLoader::new().load("resources/textures/penguin.png").unwrap();

    let tex = Texture::<Texture2D>::new(
        TexWrapMode::Repeat,
        TexWrapMode::Repeat,
        TexMinFilterMode::Linear,
        TexMagFilterMode::Linear
    );
    let pixels =  img.to_rgba().into_raw();
    tex.storage_2d(img.width() as i32, img.height() as i32);
    tex.sub_image_2d(img.width() as i32, img.height() as i32, &pixels);

    let non_resident: NonResidentBindlessTexture<Texture2D> = tex.into();
    let resident: ResidentBindlessTexture<Texture2D> = non_resident.into();

    let penguin_material = core_components::material::TexturedBasic::new(
        (0.25, 0.20725, 0.20725).into(),
        resident,
        (0.0296648*2., 0.0296648*2., 0.0296648*2.).into(),
        (5.078431).into()
    );

    let material_block = InterfaceBlock::<core_components::material::TexturedBasic>::new(&program, "Material", 3);

    let color_buffer = ColorBuffer::from_color(na::Vector3::new(0.3, 0.3, 0.5));
    color_buffer.set_used();

    let mut resources = Resources::default();
    resources.insert(program);
    resources.insert(lights_block);
    resources.insert(matrices_block);
    resources.insert(material_block);


    world.push(
        (
            transform,
            penguin_material,
            teapot_loader.load("resources/meshes/penguin.mesh").unwrap()
        )
    );

    world.push(
        (
            Transform::new(1., na::Vector3::new(-1., 0.5, -1.2), na::UnitQuaternion::from_euler_angles(0., 0., 0.)),
            PointLight {
                constant: 1.,
                linear: 0.35,
                quadratic: 0.44,
                ambient: na::Vector3::new(0.2, 0.2, 0.2),
                diffuse: na::Vector3::new(0.662, 0.450, 0.137),
                specular: na::Vector3::new(1.0, 1.0, 1.0)
            }
        )
    );

    world.push(
        (
            Transform::new(1., na::Vector3::new(1., 0., -1.2), na::UnitQuaternion::from_euler_angles(0., 0., 0.)),
            PointLight {
                constant: 1.,
                linear: 0.35,
                quadratic: 0.44,
                ambient: na::Vector3::new(0.2, 0.2, 0.2),
                diffuse: na::Vector3::new(0.266, 0.470, 0.678),
                specular: na::Vector3::new(1.0, 1.0, 1.0)
            }
        )
    );

    // world.insert(
    //     (),
    //     (0..1).map(
    //         |_| (
    //             Transform::new(1., na::Vector3::new(0.5, 0., 0.), na::UnitQuaternion::from_euler_angles(0., 0., 0.)),
    //             PointLight {
    //                 position: (0., 0., 0.).into(),
    //                 constant: (1.).into(),
    //                 linear: (0.7).into(),
    //                 quadratic: (1.8).into(),
    //                 ambient: (0.2, 0.2, 0.2).into(),
    //                 diffuse: (0.5, 0.5, 0.5).into(),
    //                 specular: (1.0, 1.0, 1.0).into()
    //             }
    //         )
    //     )
    // );

    let camera = core_components::Camera::from_fov(
        60.,
        viewport.w as f32 / viewport.h as f32,
        0.1,
        100.
    );

    let cam_transform = Transform::new(
        1.,
        na::Vector3::new(0., 0., 0.),
        na::UnitQuaternion::from_euler_angles(0., 0., 0.)
    );
    world.push((camera, cam_transform));

    let mut schedule = Schedule::builder()
        .add_thread_local(core_systems::renderer::render_system())
        .build();

    let mut event_pump = sdl.event_pump().map_err(|err|anyhow!(err))?;
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

                    for (mut camera,) in cam_query.iter_mut(&mut world) {
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

        for (mut transform, _) in query.iter_mut(&mut world) {
            transform.rotate_by(0., 0.01, 0.);
        }

        color_buffer.clear();
        schedule.execute(&mut world, &mut resources);
        window.gl_swap_window();
    }

    Ok(())
}