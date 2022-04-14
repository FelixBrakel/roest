use legion::{
    *,
    systems::*,
    world::SubWorld

};
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
mod pipeline_stage;


#[system]
pub fn render(
    world: &mut SubWorld,
    #[resource] program: &Program,
    #[resource] gpu_lights: &InterfaceBlock<Lights>,
    #[resource] gpu_matrices: &InterfaceBlock<Matrices>,
    #[resource] gpu_material: &InterfaceBlock<material::TexturedBasic>,
    cam_query: &mut Query<(&Transform, &Camera)>,
    mesh_query: &mut Query<(&Transform, &material::TexturedBasic, &IndexedMesh)>,
    light_query: &mut Query<(&Transform, &PointLight)>) {
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
