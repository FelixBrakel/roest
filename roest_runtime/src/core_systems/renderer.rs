use legion::prelude::*;
use crate::core_components::{
    IndexedMesh,
    IMeshDefaults,
    Camera,
    Transform,
    material,
};
use gl_renderer::{Program, data::matrix_data::mat4};
use hibitset::BitSetLike;
use gl_renderer::uniform_buffer::InterfaceBlock;
use gl_renderer::uniform_struct_shared::ShaderDefaultLayout;

pub struct RendererSystem { }

impl RendererSystem {
    pub fn system() -> Box<dyn Runnable> {
        SystemBuilder::<()>::new("renderer")
            .read_resource::<Program>()
            .read_resource::<InterfaceBlock<ShaderDefaultLayout>>()
            .with_query(<(Read<Camera>,)>::query())
            .with_query(<(Read<Transform>, Read<material::Flat>, Read<IndexedMesh>)>::query())
            .build_thread_local( move |_, world, resource, (cam_query, mesh_query)| {
                let mut imesh_defaults = IMeshDefaults::zero();
                let (program, interface_block) = resource;


                for (camera,) in cam_query.iter(&mut *world) {
                    let vp = camera.perspective * camera.view;
                    for (mesh_transform, material, mesh) in mesh_query.iter(&mut *world) {
                        let model = mesh_transform.model();
                        let mvp = vp * model;
                        let mv = camera.view * model;
                        // let mvp = mat4::identity();
                        // println!("{}", mesh_transform.model());
                        interface_block.uniform_struct.mvp.set(&mvp);
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


