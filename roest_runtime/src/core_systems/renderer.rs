use legion::prelude::*;
use crate::core_components::{
    IndexedMesh,
    IMeshDefaults,
    Camera,
    Transform,
    material,
};
use crate::Lights;
use gl_renderer::{Program, GPUAggregate};
use gl_renderer::uniform_buffer::InterfaceBlock;
use nalgebra::Point;
use nalgebra as na;
use gl_renderer::light::PointLight;

pub struct RendererSystem { }

impl RendererSystem {
    pub fn system() -> Box<dyn Runnable> {
        SystemBuilder::<()>::new("renderer")
            .read_resource::<Program>()
            .read_resource::<InterfaceBlock<Lights>>()
            .with_query(<(Read<Camera>,)>::query())
            .with_query(<(Read<Transform>, Read<material::Flat>, Read<IndexedMesh>)>::query())
            .with_query(<(Read<Transform>, Write<PointLight>)>::query())
            .build_thread_local( move |_, world, resource, (cam_query, mesh_query, light_query)| {
                let mut imesh_defaults = IMeshDefaults::zero();
                let (program, gpu_lights) = resource;
                let mut lights = Lights::default();
                for (transform, mut point_light) in light_query.iter(&mut *world) {
                    let pos_vec = na::Vector4::new(
                        point_light.position.d0,
                        point_light.position.d1,
                        point_light.position.d2,
                        1.
                    );
                    let new_pos_vec = transform.model() * pos_vec;

                    point_light.position.d0 = new_pos_vec[0];
                    point_light.position.d1 = new_pos_vec[1];
                    point_light.position.d2 = new_pos_vec[2];

                    lights.add_point_light(*point_light);
                }

                gpu_lights.uniform_struct.set(&lights);

                for (camera,) in cam_query.iter(&mut *world) {
                    let vp = camera.perspective * camera.view;
                    for (mesh_transform, material, mesh) in mesh_query.iter(&mut *world) {
                        let model = mesh_transform.model();
                        let mvp = vp * model;
                        let mv = camera.view * model;
                        // let mvp = mat4::identity();
                        // println!("{}", mesh_transform.model());
                        imesh_defaults.set(mvp, mv, model, camera.view, camera.perspective);
                        program.set_used();
                        program.set_defaults(&imesh_defaults);
                        program.set_material(&*material);
                        mesh.render()
                    }
                }
            }
        )
    }
}


