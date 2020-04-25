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

pub struct RendererSystem { }

impl RendererSystem {
    pub fn system() -> Box<dyn Runnable> {
        SystemBuilder::<()>::new("renderer")
            .read_resource::<Program>()
            .with_query(<(Read<Camera>,)>::query())
            .with_query(<(Read<Transform>, Read<material::Flat>, Read<IndexedMesh>)>::query())
            .build_thread_local( move |_, world, resource, (cam_query, mesh_query)| {
                let mut imesh_defaults = IMeshDefaults::zero();
                let program = resource;


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


