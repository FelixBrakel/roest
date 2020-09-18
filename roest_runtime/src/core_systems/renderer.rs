use legion::prelude::*;
use crate::core_components::{
    IndexedMesh,
    Camera,
    Transform,
    material,
    light::PointLight
};
use crate::core_resources::gpu_blocks::{Lights, Matrices};
use gl_renderer::{Program, GPUAggregate};
use gl_renderer::uniform_buffer::InterfaceBlock;
use nalgebra::{U3};
use nalgebra as na;
use gl_renderer::light::PointLight as GPUPointLight;

pub struct RendererSystem { }

impl RendererSystem {
    pub fn system() -> Box<dyn Runnable> {
        SystemBuilder::<()>::new("renderer")
            .read_resource::<Program>()
            .read_resource::<InterfaceBlock<Lights>>()
            .read_resource::<InterfaceBlock<Matrices>>()
            .read_resource::<InterfaceBlock<material::Basic>>()
            .with_query(<(Read<Transform>, Read::<Camera>)>::query())
            .with_query(<(Read<Transform>, Read<material::Basic>, Read<IndexedMesh>)>::query())
            .with_query(<(Read<Transform>, Read<PointLight>)>::query())
            .read_component::<Camera>() // This
            .build_thread_local( move |_, world, resource, (cam_query, mesh_query, light_query)| {
                let (program, gpu_lights, gpu_matrices, gpu_material) = resource;

                for (cam_transform, camera) in cam_query.iter(world) {
                    let view_matrix = match cam_transform.model().try_inverse() {
                        Some(matrix) => matrix,
                        None => panic!("Camera transform matrix is not invertible!")
                    };
                    let vp = camera.perspective() * view_matrix;
                    gpu_matrices.uniform_struct.v.set(&view_matrix);
                    gpu_matrices.uniform_struct.p.set(camera.perspective());


                    let mut lights = Lights::default();
                    for (transform, point_light) in light_query.iter(world) {
                        let pos_vec = view_matrix * transform.model() * na::Vector4::new(0., 0., 0., 1.);
                        let light = GPUPointLight::new(
                            (pos_vec[0], pos_vec[1], pos_vec[2]).into(),
                            point_light.constant.into(),
                            point_light.linear.into(),
                            point_light.quadratic.into(),
                            (point_light.ambient[0], point_light.ambient[1], point_light.ambient[2]).into(),
                            (point_light.diffuse[0], point_light.diffuse[1], point_light.diffuse[2]).into(),
                            (point_light.specular[0], point_light.specular[1], point_light.specular[2]).into()
                        );

                        match lights.add_point_light(light) {
                            Ok(_) => (),
                            Err(_) => {
                                println!("WARNING: max points lights reached")
                            }
                        };
                    }

                    gpu_lights.uniform_struct.set(&lights);

                    for (mesh_transform, material, mesh) in mesh_query.iter(world) {
                        let model: na::Matrix4<f32> = mesh_transform.model();
                        let mvp = vp * model;
                        let mv = view_matrix * model;

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


