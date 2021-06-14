use crate::editor_components::{EditorComponent, ComponentRepresentation};
use roest_runtime::core_components::Camera;
use crate::EditorWorld;

use std::{
    rc::Rc,
    cell::RefCell
};

use gtk::prelude::*;

use legion::prelude::*;

impl EditorComponent for Camera {
    fn component_representation() -> ComponentRepresentation {
        let insert_func = |world: &mut EditorWorld, entity| {
            world.add_component(entity, Camera::default()).unwrap();
        };

        // let remove_func = |world: &mut EditorWorld, entity| {
        //     world.remove_component(entity).unwrap();
        // };

        // TODO: Build UI for Camera component
        let ui_func = |world: Rc<RefCell<EditorWorld>>, entity: Entity, paned: &gtk::Paned| {
            let src = include_str!("component_none.glade");
            let builder = gtk::Builder::from_string(src);
            let frame: gtk::Frame = builder.get_object("empty_frame").unwrap();

            paned.add2(&frame);
            paned.set_child_shrink(&frame, false)
        };

        ComponentRepresentation::new("Camera".to_string(), insert_func, ui_func)
    }
}