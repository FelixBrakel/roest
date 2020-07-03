use legion::prelude::*;
use crate::core_components::{
    IndexedMesh,
    IMeshDefaults,
    Camera,
    Transform,
    material,
};
use crate::{Lights, Matrices};
use gl_renderer::{Program, GPUAggregate};
use gl_renderer::uniform_buffer::InterfaceBlock;
use nalgebra::{Point, U3, Matrix};
use nalgebra as na;
use gl_renderer::light::PointLight;

pub struct RendererSystem { }

impl RendererSystem {
    pub fn system() -> Box<dyn Runnable> {
        SystemBuilder::<()>::new("renderer")
            .read_resource::<Program>()
            .read_resource::<InterfaceBlock<Lights>>()
            .read_resource::<InterfaceBlock<Matrices>>()
            .read_resource::<InterfaceBlock<material::TexturedBasic>>()
            .with_query(<(Read<Camera>,)>::query())
            .with_query(<(Read<Transform>, Read<material::TexturedBasic>, Read<IndexedMesh>)>::query())
            .with_query(<(Read<Transform>, Read<PointLight>)>::query())
            .build_thread_local( move |_, world, resource, (cam_query, mesh_query, light_query)| {
                let (program, gpu_lights, gpu_matrices, gpu_material) = resource;

                for (camera,) in cam_query.iter(&mut *world) {
                    let vp = camera.perspective * camera.view;
                    gpu_matrices.uniform_struct.v.set(&camera.view);
                    gpu_matrices.uniform_struct.p.set(&camera.perspective);


                    let mut lights = Lights::default();
                    for (transform, point_light) in light_query.iter(&mut *world) {
                        let mut light = (*point_light).clone();
                        let pos_vec = na::Vector4::new(
                            light.position.d0,
                            light.position.d1,
                            light.position.d2,
                            1.
                        );
                        let new_pos_vec = camera.view * transform.model() * pos_vec;

                        light.position.d0 = new_pos_vec[0];
                        light.position.d1 = new_pos_vec[1];
                        light.position.d2 = new_pos_vec[2];

                        lights.add_point_light(light);
                    }

                    gpu_lights.uniform_struct.set(&lights);

                    for (mesh_transform, material, mesh) in mesh_query.iter(&mut *world) {
                        let model: na::Matrix4<f32> = mesh_transform.model();
                        let mvp = vp * model;
                        let mv = camera.view * model;

                        gpu_matrices.uniform_struct.mvp.set(&mvp);
                        // gpu_matrices.uniform_struct.mvp.set(&na::Matrix4::identity());
                        // gpu_matrices.uniform_struct.mvp.set(&na::Matrix4::zeros());

                        gpu_matrices.uniform_struct.mv.set(&mv);
                        gpu_matrices.uniform_struct.m.set(&model);

                        let tmp = mv.fixed_slice::<U3, U3>(0, 0).try_inverse().unwrap().transpose();
                        gpu_matrices.uniform_struct.n.set(&tmp);

                        gpu_material.uniform_struct.set(&material);
                        program.set_used();
                        mesh.render()
                    }
                }
            }
        )
    }
}


