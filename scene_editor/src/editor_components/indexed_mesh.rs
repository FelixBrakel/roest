use roest_runtime::{
    core_components::IndexedMesh,
    core_systems::resource_manager::{
        data_loaders::IndexedMeshLoader,
        Loader,
    },
};

use gl_renderer::{vertex, IndexedVertArray};

use super::EditorComponent;
use crate::editor_components::ComponentRepresentation;
use crate::EditorWorld;
use std::{
    rc::Rc,
    cell::RefCell
};

use gtk::prelude::*;
use legion::prelude::*;
use roest_runtime::core_components::Transform;
use gl_renderer::vertex::ColoredVertex;

impl EditorComponent for IndexedMesh {
    fn component_representation() -> ComponentRepresentation {
        let insert_func = |world: &mut EditorWorld, entity| {
            // TODO: don't hardcode this mesh but for that we need to trea meshes as resources, not components...
            // lot of work though
            let loader: IndexedMeshLoader<vertex::NormalVertex> = IndexedMeshLoader::new();

            let res = world.add_component(entity, loader.load("resources/meshes/triangle.mesh").unwrap());
            match res {
                Ok(v) => return,
                Err(e) => panic!("{:?}", e)
            }
            // let imesh = IndexedMesh::new(&IndexedVertArray::new(
            //     vec![
            //         ColoredVertex {pos: (-1., -1., 1.).into(), clr: (0., 1., 0., 0.).into()},
            //         ColoredVertex {pos: (1., -1., 1.).into(), clr: (0., 0., 1., 0.).into()},
            //         ColoredVertex {pos: (0., 1., 1.).into(), clr: (0., 0., 0., 1.).into()}
            //     ],
            //     vec![0, 1, 2]));
            // world.add_component(entity, imesh);
            // world.add_component(entity, Transform::default());
        };

        // TODO: Placeholder UI should at least have the name of the component in it
        let ui_func = |world: Rc<RefCell<EditorWorld>>, entity: Entity, paned: &gtk::Paned| {
            let src = include_str!("component_none.glade");
            let builder = gtk::Builder::from_string(src);
            let frame: gtk::Frame = builder.get_object("empty_frame").unwrap();

            paned.add2(&frame);
            paned.set_child_shrink(&frame, false)
        };

        ComponentRepresentation::new("Indexed Mesh".to_string(), insert_func, ui_func)
    }
}